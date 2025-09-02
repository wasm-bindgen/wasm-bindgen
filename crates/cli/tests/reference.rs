//! A test suite to check the reference JS and Wasm output of the `wasm-bindgen`
//! library.
//!
//! This is intended as an end-to-end integration test where we can track
//! changes to the JS and Wasm output.
//!
//! Tests are located in `reference/*.rs` files and are accompanied with sibling
//! `*.js` files and `*.wat` files with the expected output of the `*.rs`
//! compilation. Use `BLESS=1` in the environment to automatically update all
//! tests.
//!
//! Note: Tests are run sequentially. In CI, tests are run ordered by name and
//! all tests will be run to show all errors. Outside of CI, recently modified
//! tests are run first and the runner will stop on the first failure. This is
//! done to make it faster to iterate on tests.
//!
//! ## Dependencies
//!
//! By default, tests only have access to the `wasm-bindgen` and
//! `wasm-bindgen-futures` crates. Additional crates can be used by declaring
//! them as dependencies using a comment at the top of the test file.
//! For example:
//!
//! ```rust
//! // DEPENDENCY: web-sys = { path = '{root}/crates/web-sys', features = ['console', 'Url', 'MediaSourceReadyState'] }
//! ```
//!
//! This will add the `web-sys` crate as a dependency to the test, allowing the
//! test to use the `console`, `Url`, and `MediaSourceReadyState` features, as
//! well as the `web-sys` crate itself.
//!
//! Note that the `{root}` placeholder will be replaced with the path to the
//! root of the `wasm-bindgen` repository.
//!
//! Multiple dependencies can be declared in a single test file using multiple
//! `DEPENDENCY:` comments.
//!
//! ## Custom CLI flags
//!
//! By default, tests will use the `bundler` target. Custom CLI flags can be
//! passed to the `wasm-bindgen` CLI by declaring them in a comment at the top
//! of the test file. For example:
//!
//! ```rust
//! // FLAGS: --target=web --reference-types
//! ```
//!
//! Multiple comments can be used to run the test multiple times with different
//! flags.
//!
//! ```rust
//! // FLAGS: --target=web
//! // FLAGS: --target=nodejs
//! ```

use anyhow::{bail, Result};
use assert_cmd::prelude::*;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walrus::{ExportItem, FunctionKind, Module, ModuleConfig};

#[rstest::rstest]
fn runtest(
    #[base_dir = "tests/reference"]
    #[files("*.rs")]
    test: PathBuf,
) -> Result<()> {
    let contents = fs::read_to_string(&test)?;
    let td = tempfile::TempDir::new()?;
    let root = repo_root();
    let root = root.display();

    // parse target declarations
    let mut all_flags: Vec<_> = contents
        .lines()
        .filter_map(|l| l.strip_prefix("// FLAGS: "))
        .map(|l| l.trim())
        .collect();
    if all_flags.is_empty() {
        all_flags.push("");
    }

    // parse additional dependency declarations
    let dependencies = contents
        .lines()
        .filter_map(|l| l.strip_prefix("// DEPENDENCY: "))
        .map(|l| "\n            ".to_string() + &l.trim().replace("{root}", &root.to_string()))
        .fold(String::new(), |a, b| a + &b);

    let manifest = format!(
        "
            [package]
            name = \"reference-test\"
            authors = []
            version = \"1.0.0\"
            edition = '2021'

            [dependencies]
            wasm-bindgen = {{ path = '{root}' }}
            wasm-bindgen-futures = {{ path = '{root}/crates/futures' }}
            {dependencies}

            [lib]
            crate-type = ['cdylib']
            path = '{test}'
        ",
        test = test.display(),
    );

    fs::write(td.path().join("Cargo.toml"), manifest)?;
    let target_dir = target_dir(test.file_name().unwrap());
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(td.path())
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &target_dir);
    exec(&mut cargo)?;

    let wasm = target_dir
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("reference_test.wasm");

    for (flags_index, &flags) in all_flags.iter().enumerate() {
        // extract the target from the flags
        let target = flags
            .split_whitespace()
            .find_map(|f| f.strip_prefix("--target="))
            .unwrap_or("bundler");

        let out_dir = &td.path().join(target);
        fs::create_dir(out_dir)?;

        let mut bindgen = Command::cargo_bin("wasm-bindgen")?;
        bindgen
            .arg("--out-dir")
            .arg(out_dir)
            .arg(&wasm)
            .arg("--remove-producers-section");
        for flag in flags.split_whitespace() {
            bindgen.arg(flag);
        }
        if contents.contains("// enable-externref") {
            bindgen.env("WASM_BINDGEN_EXTERNREF", "1");
        }
        exec(&mut bindgen)?;

        // suffix the file name with the target
        let test = if all_flags.len() > 1 {
            let base_file_name = format!(
                "{}-{}.rs",
                test.file_stem().unwrap().to_string_lossy(),
                flags_index
            );
            test.with_file_name(base_file_name)
        } else {
            test.to_owned()
        };

        // bundler uses a different main JS file, because its
        // reference_test.js just imports the reference_test_bg.js
        let main_js_file = match target {
            "bundler" => "reference_test_bg.js",
            _ => "reference_test.js",
        };

        if !contents.contains("async") {
            let js = fs::read_to_string(out_dir.join(main_js_file))?;
            assert_same(&js, &test.with_extension("js"))?;
            let wat = sanitize_wasm(&out_dir.join("reference_test_bg.wasm"))?;
            assert_same(&wat, &test.with_extension("wat"))?;
        }
        let d_ts = fs::read_to_string(out_dir.join("reference_test.d.ts"))?;
        assert_same(&d_ts, &test.with_extension("d.ts"))?;
    }

    Ok(())
}

fn assert_same(output: &str, expected: &Path) -> Result<()> {
    if env::var("BLESS").is_ok() {
        fs::write(expected, output)?;
    } else {
        let expected = fs::read_to_string(expected)?;
        diff(&expected, output)?;
    }
    Ok(())
}

fn sanitize_wasm(wasm: &Path) -> Result<String> {
    // Clean up the Wasm module by removing all function
    // implementations/instructions, data sections, etc. This'll help us largely
    // only deal with exports/imports which is all we're really interested in.
    let mut module = ModuleConfig::new()
        .generate_producers_section(false)
        .parse_file(wasm)?;

    sanitize_local_funcs(&mut module);

    let ids = module.data.iter().map(|d| d.id()).collect::<Vec<_>>();
    for id in ids {
        module.data.delete(id);
    }
    for mem in module.memories.iter_mut() {
        mem.data_segments.drain();
    }
    let ids = module.elements.iter().map(|d| d.id()).collect::<Vec<_>>();
    for id in ids {
        module.elements.delete(id);
    }
    for table in module.tables.iter_mut() {
        table.elem_segments.drain();
    }
    let ids = module
        .exports
        .iter()
        .filter(|e| matches!(e.item, walrus::ExportItem::Global(_)))
        .map(|d| d.id())
        .collect::<Vec<_>>();
    for id in ids {
        module.exports.delete(id);
    }
    walrus::passes::gc::run(&mut module);
    let mut wat = wasmprinter::print_bytes(module.emit_wasm())?;
    wat.push('\n');
    Ok(wat)
}

/// Sort all exported local functions by export order, and remove their bodies.
///
/// This removes inconsistency between toolchains on different OS producing
/// local functions in different order, even though exports are consistent.
fn sanitize_local_funcs(module: &mut Module) {
    let func_ids: Vec<_> = module
        .exports
        .iter()
        .filter_map(|e| match e.item {
            ExportItem::Function(f)
                if matches!(module.funcs.get(f).kind, FunctionKind::Local(_)) =>
            {
                Some(f)
            }
            _ => None,
        })
        .collect();

    for id in func_ids {
        let old_name = module.funcs.get_mut(id).name.take();
        // Replace with an empty function. This ensures two things:
        // 1. Because we replace in export order, the new local functions are sorted in the same way.
        // 2. New functions don't have any instructions, which is what we want for comparisons anyway.
        let new_id = module.replace_exported_func(id, |_| {}).unwrap();
        module.funcs.get_mut(new_id).name = old_name;
    }
}

fn diff(a: &str, b: &str) -> Result<()> {
    if a == b {
        return Ok(());
    }
    let mut s = String::new();
    for result in diff::lines(a, b) {
        match result {
            diff::Result::Both(l, _) => {
                s.push(' ');
                s.push_str(l);
            }
            diff::Result::Left(l) => {
                s.push('-');
                s.push_str(l);
            }
            diff::Result::Right(l) => {
                s.push('+');
                s.push_str(l);
            }
        }
        s.push('\n');
    }
    bail!("found a difference:\n\n{}", s);
}

fn target_dir(name: &OsStr) -> PathBuf {
    repo_root().join("target/tests/reference").join(name)
}

fn repo_root() -> PathBuf {
    let mut repo_root = env::current_dir().unwrap();
    if repo_root.file_name() == Some("cli".as_ref()) {
        repo_root.pop(); // remove 'cli'
        repo_root.pop(); // remove 'crates'
    }
    repo_root
}

fn exec(cmd: &mut Command) -> Result<()> {
    let output = cmd.output()?;
    if output.status.success() {
        return Ok(());
    }
    let mut err = format!("command failed {cmd:?}");
    err.push_str(&format!("\nstatus: {}", output.status));
    err.push_str(&format!(
        "\nstderr:\n{}",
        tab(&String::from_utf8_lossy(&output.stderr))
    ));
    err.push_str(&format!(
        "\nstdout:\n{}",
        tab(&String::from_utf8_lossy(&output.stdout))
    ));
    bail!("{}", err);
}

fn tab(s: &str) -> String {
    format!("    {}", s.replace('\n', "\n    "))
}
