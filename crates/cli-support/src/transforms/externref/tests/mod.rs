//! A small test framework to execute a test function over all files in a
//! directory.
//!
//! Each file in the directory has its own `CHECK-ALL` annotation indicating the
//! expected output of the test. That can be automatically updated with
//! `BLESS=1` in the environment. Otherwise the test are checked against the
//! listed expectation.

use anyhow::{anyhow, bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walrus::ModuleConfig;
use wast::parser::{Parse, Parser};

fn runtest(test: &Test) -> Result<String> {
    let wasm = wat::parse_file(&test.file)?;
    let mut walrus = ModuleConfig::new()
        .generate_producers_section(false)
        .parse(&wasm)?;
    let mut cx = super::Context::new(&mut walrus)?;
    for directive in test.directives.iter() {
        match &directive.kind {
            DirectiveKind::Export(name) => {
                let export = walrus
                    .exports
                    .iter()
                    .find(|e| e.name == *name)
                    .ok_or_else(|| anyhow!("failed to find export"))?;
                cx.export_xform(export.id(), &directive.args, directive.ret_externref);
            }
            DirectiveKind::Import(module, field) => {
                let import = walrus
                    .imports
                    .iter()
                    .find(|e| e.module == *module && e.name == *field)
                    .ok_or_else(|| anyhow!("failed to find export"))?;
                cx.import_xform(import.id(), &directive.args, directive.ret_externref);
            }
        }
    }
    cx.run(&mut walrus)?;
    walrus::passes::gc::run(&mut walrus);
    let printed = wasmprinter::print_bytes(walrus.emit_wasm())?;
    Ok(printed)
}

#[rstest::rstest]
fn run_test(
    #[base_dir = "src/transforms/externref/tests"]
    #[files("*.wat")]
    test: PathBuf,
) -> Result<()> {
    let expected = Test::from_file(&test)?;
    let actual = runtest(&expected)?;
    expected.check(&actual)
}

struct Test {
    file: PathBuf,
    directives: Vec<Directive>,
    assertion: Option<String>,
}

struct Directive {
    args: Vec<(usize, bool)>,
    ret_externref: bool,
    kind: DirectiveKind,
}

enum DirectiveKind {
    Import(String, String),
    Export(String),
}

impl Test {
    fn from_file(path: &Path) -> Result<Test> {
        let contents = fs::read_to_string(path)?;
        let mut iter = contents.lines();
        let mut assertion = None;
        let mut directives = Vec::new();
        while let Some(line) = iter.next() {
            if line.starts_with("(; CHECK-ALL:") {
                let mut pattern = String::new();
                for line in iter.by_ref() {
                    if line == ";)" {
                        break;
                    }
                    pattern.push_str(line);
                    pattern.push('\n');
                }
                if iter.next().is_some() {
                    bail!("CHECK-ALL must be at the end of the file");
                }
                assertion = Some(pattern);
                continue;
            }

            if !line.starts_with(";; @xform") {
                continue;
            }
            let directive = &line[9..];
            let buf = wast::parser::ParseBuffer::new(directive)?;
            directives.push(wast::parser::parse::<Directive>(&buf)?);
        }
        Ok(Test {
            file: path.to_path_buf(),
            directives,
            assertion,
        })
    }

    fn check(&self, output: &str) -> Result<()> {
        if option_env!("BLESS").is_some() {
            update_output(&self.file, output)
        } else if let Some(pattern) = &self.assertion {
            if output == pattern {
                return Ok(());
            }
            bail!(
                "expected\n    {}\n\nactual\n    {}",
                pattern.replace('\n', "\n    "),
                output.replace('\n', "\n    ")
            );
        } else {
            bail!(
                "no test assertions were found in this file, but you can \
                 rerun tests with `BLESS=1` to automatically add assertions \
                 to this file"
            );
        }
    }
}

fn update_output(path: &Path, output: &str) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let start = contents.find("(; CHECK-ALL:").unwrap_or(contents.len());

    let mut new_output = String::new();
    for line in output.lines() {
        new_output.push_str(line);
        new_output.push('\n');
    }
    let new = format!(
        "{}\n\n(; CHECK-ALL:\n{}\n;)\n",
        contents[..start].trim(),
        new_output.trim_end()
    );
    fs::write(path, new)?;
    Ok(())
}

impl<'a> Parse<'a> for Directive {
    fn parse(parser: Parser<'a>) -> wast::parser::Result<Self> {
        use wast::kw;
        wast::custom_keyword!(externref_owned);
        wast::custom_keyword!(externref_borrowed);
        wast::custom_keyword!(other);

        let kind = if parser.peek::<kw::import>()? {
            parser.parse::<kw::import>()?;
            DirectiveKind::Import(parser.parse()?, parser.parse()?)
        } else {
            parser.parse::<kw::export>()?;
            DirectiveKind::Export(parser.parse()?)
        };
        let mut args = Vec::new();
        parser.parens(|p| {
            let mut i = 0;
            while !p.is_empty() {
                if parser.peek::<externref_owned>()? {
                    parser.parse::<externref_owned>()?;
                    args.push((i, true));
                } else if parser.peek::<externref_borrowed>()? {
                    parser.parse::<externref_borrowed>()?;
                    args.push((i, false));
                } else {
                    parser.parse::<other>()?;
                }
                i += 1;
            }
            Ok(())
        })?;

        let ret_externref = parser.parse::<Option<externref_owned>>()?.is_some();
        Ok(Directive {
            args,
            ret_externref,
            kind,
        })
    }
}
