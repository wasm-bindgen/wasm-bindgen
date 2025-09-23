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

use crate::Project;
use anyhow::{bail, Result};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walrus::{
    ElementItems, ElementKind, ExportItem, FunctionKind, ImportKind, Module, ModuleConfig,
};

#[rstest::rstest]
fn runtest(
    #[base_dir = "tests/reference"]
    #[files("*.rs")]
    test: PathBuf,
) -> Result<()> {
    let contents = fs::read_to_string(&test)?;

    // parse target declarations
    let mut all_flags: Vec<_> = contents
        .lines()
        .filter_map(|l| l.strip_prefix("// FLAGS: "))
        .map(|l| l.trim())
        .collect();
    if all_flags.is_empty() {
        all_flags.push("");
    }

    let mut project = Project::new(
        test.file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace('-', "_")
            + "_reftest",
    );

    // parse additional dependency declarations
    project.dep("wasm-bindgen-futures = { path = '{root}/crates/futures' }");

    contents
        .lines()
        .filter_map(|l| l.strip_prefix("// DEPENDENCY: "))
        .for_each(|dep| {
            project.dep(dep);
        });

    project.file_link("src/lib.rs", &test);

    for &flags in &all_flags {
        // extract the target from the flags
        let target = flags
            .split_whitespace()
            .find_map(|f| f.strip_prefix("--target="))
            .unwrap_or("bundler");

        let (mut cmd, out_dir) =
            project.wasm_bindgen(&format!("{flags} --out-name reference_test"));
        cmd.assert().success();

        // suffix the file name with the sanitized flags
        let test = if all_flags.len() > 1 {
            let mut base_file_name = test.file_stem().unwrap().to_str().unwrap().to_owned();

            for chunk in flags.split(|c: char| !c.is_ascii_alphanumeric()) {
                if !chunk.is_empty() {
                    base_file_name.push('-');
                    base_file_name.push_str(chunk);
                }
            }

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
        .filter(|e| matches!(e.item, ExportItem::Global(_)))
        .map(|d| d.id())
        .collect::<Vec<_>>();
    for id in ids {
        module.exports.delete(id);
    }
    // Prevent imports from being GC'd away as we want to see them in snapshots.
    let temp_element_id = module.elements.add(
        ElementKind::Declared,
        ElementItems::Functions(
            module
                .imports
                .iter()
                .filter_map(|i| match i.kind {
                    ImportKind::Function(f) => {
                        // Preserve but delete name as it's not cross-platform.
                        module.funcs.get_mut(f).name = None;
                        Some(f)
                    }
                    _ => None,
                })
                .collect(),
        ),
    );
    walrus::passes::gc::run(&mut module);
    module.elements.delete(temp_element_id);
    // Sort imports for deterministic snapshot.
    std::mem::take(&mut module.imports)
        .iter()
        .map(|i| ((&i.module, &i.name), i.kind.clone()))
        .collect::<BTreeMap<_, _>>()
        .into_iter()
        .for_each(|((module_name, name), kind)| {
            module.imports.add(module_name, name, kind);
        });
    wasmprinter::print_bytes(module.emit_wasm())
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
