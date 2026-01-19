//! A small test framework to execute a test function over all files in a
//! directory.
//!
//! Each file in the directory has its own `CHECK-ALL` annotation indicating the
//! expected output of the test. That can be automatically updated with
//! `BLESS=1` in the environment. Otherwise the test are checked against the
//! listed expectation.

use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walrus::ModuleConfig;
use wast::parser::{Parse, Parser};

fn runtest(test: &Test) -> Result<String> {
    let wasm = wat::parse_file(&test.file)?;
    let mut walrus = ModuleConfig::new()
        .generate_producers_section(false)
        .parse(&wasm)?;
    let mut exports = Vec::new();
    let mut xforms = Vec::new();
    for directive in test.directives.iter() {
        let export = walrus
            .exports
            .iter()
            .find(|e| e.name == directive.name)
            .unwrap();
        let id = match export.item {
            walrus::ExportItem::Function(id) => id,
            _ => panic!("must be function export"),
        };
        exports.push(export.id());
        xforms.push((id, 0, directive.tys.clone()));
    }
    let memory = walrus.memories.iter().next().unwrap().id();
    let stack_pointer = walrus.globals.iter().next().unwrap().id();
    let ret = super::run(&mut walrus, memory, stack_pointer, &xforms)?;
    for (export, id) in exports.into_iter().zip(ret) {
        walrus.exports.get_mut(export).item = walrus::ExportItem::Function(id);
    }
    walrus::passes::gc::run(&mut walrus);
    let printed = wasmprinter::print_bytes(walrus.emit_wasm())?;
    Ok(printed)
}

#[rstest::rstest]
fn run_test(
    #[base_dir = "src/transforms/multi_value/tests"]
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
    name: String,
    tys: Vec<walrus::ValType>,
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
        use wast::{core::ValType, kw};

        parser.parse::<kw::export>()?;
        let name = parser.parse()?;
        let mut tys = Vec::new();
        parser.parens(|p| {
            while !p.is_empty() {
                tys.push(match p.parse()? {
                    ValType::I32 => walrus::ValType::I32,
                    ValType::I64 => walrus::ValType::I64,
                    ValType::F32 => walrus::ValType::F32,
                    ValType::F64 => walrus::ValType::F64,
                    _ => panic!(),
                });
            }
            Ok(())
        })?;
        Ok(Directive { name, tys })
    }
}

#[test]
fn round_up_to_alignment_works() {
    for &(n, align, expected) in &[
        (0, 1, 0),
        (1, 1, 1),
        (2, 1, 2),
        (0, 2, 0),
        (1, 2, 2),
        (2, 2, 2),
        (3, 2, 4),
        (0, 4, 0),
        (1, 4, 4),
        (2, 4, 4),
        (3, 4, 4),
        (4, 4, 4),
        (5, 4, 8),
    ] {
        let actual = super::round_up_to_alignment(n, align);
        println!(
            "round_up_to_alignment(n = {n}, align = {align}) = {actual} (expected {expected})"
        );
        assert_eq!(actual, expected);
    }
}
