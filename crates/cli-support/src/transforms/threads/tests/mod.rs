//! A small test framework to execute a test function over all files in a
//! directory.
//!
//! Each file in the directory has its own `CHECK-ALL` annotation indicating the
//! expected output of the test. That can be automatically updated with
//! `BLESS=1` in the environment. Otherwise the test are checked against the
//! listed expectation.

use crate::transforms::unstart_start_function;
use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walrus::ModuleConfig;

fn runtest(test: &Test) -> Result<String> {
    let wasm = wat::parse_file(&test.file)?;
    let mut module = ModuleConfig::new()
        .generate_producers_section(false)
        .parse(&wasm)?;

    super::run(&mut module)?;
    walrus::passes::gc::run(&mut module);

    // We add an extra parameter to the start function, making it invalid for the start section.
    // It's only valid in combination with the "unstart" step.
    unstart_start_function(&mut module);

    let features = wasmparser::WasmFeatures::default() | wasmparser::WasmFeatures::THREADS;

    wasmparser::Validator::new_with_features(features).validate_all(&module.emit_wasm())?;

    let printed = wasmprinter::print_bytes(module.emit_wasm())?;

    Ok(printed)
}

#[rstest::rstest]
fn run_test(
    #[base_dir = "src/transforms/threads/tests"]
    #[files("*.wat")]
    test: PathBuf,
) -> Result<()> {
    let expected = Test::from_file(&test)?;
    let actual = runtest(&expected)?;
    expected.check(&actual)
}

struct Test {
    file: PathBuf,
    assertion: Option<String>,
}

impl Test {
    fn from_file(path: &Path) -> Result<Test> {
        let contents = fs::read_to_string(path)?;
        let mut iter = contents.lines();
        let mut assertion = None;
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
        }
        Ok(Test {
            file: path.to_path_buf(),
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
