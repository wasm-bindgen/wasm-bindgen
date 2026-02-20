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
use anyhow::Result;
use assert_cmd::Command;
use regex::Regex;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walrus::{
    ElementItems, ElementKind, ExportItem, FunctionKind, ImportKind, Module, ModuleConfig,
};

macro_rules! regex {
    ($re:literal) => {{
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new($re).unwrap());
        &*RE
    }};
}

// A helper to remove unstable parts of the output like function indices
// and hash values, while ensuring that the replacement names stay consistent
// between all output files.
#[derive(Default)]
struct Sanitizer {
    prev_replacements: HashMap<String, usize>,
}

impl Sanitizer {
    fn sanitize_one<'s>(
        &mut self,
        s: &'s str,
        regex: &Regex,
        replacement: impl Fn(usize) -> String,
    ) -> Cow<'s, str> {
        regex.replace_all(s, |caps: &regex::Captures| {
            let index = self.prev_replacements.len();

            let index = self
                .prev_replacements
                .entry(caps[0].to_string())
                .or_insert(index);

            replacement(*index)
        })
    }

    fn sanitize(&mut self, s: &str) -> String {
        // Cast descriptors for closures contain unstable function indices, so we need to sanitize the hash.
        let s = self.sanitize_one(s, regex!(r"__wbindgen_cast_[0-9a-f]{16}"), |idx| {
            format!("__wbindgen_cast_{idx:016x}")
        });

        // `h...` are mangled generic function names with unstable type IDs.
        let s = self.sanitize_one(&s, regex!(r"h[0-9a-f]{16}"), |idx| format!("h{idx:016x}"));

        // Cast descriptors contain `Closure`'s `Debug` impl which has unstable function indices.
        let s = self.sanitize_one(&s, regex!(r"_idx: \d+,"), |idx| format!("_idx: {idx},"));

        s.into_owned()
    }

    fn assert_same(&mut self, output: &str, expected: &Path) -> Result<()> {
        let output = self.sanitize(output);
        if env::var("BLESS").is_ok() {
            fs::write(expected, output.as_bytes())?;
        } else {
            let expected = fs::read_to_string(expected)?;
            pretty_assertions::assert_str_eq!(expected, output);
        }
        Ok(())
    }
}

#[rstest::rstest]
fn runtest(
    #[base_dir = "tests/reference"]
    #[files("*.rs")]
    test: PathBuf,
) -> Result<()> {
    runtest_with_opts(test, None, |_| ())
}

#[test]
fn runtest_targets_atomics() -> Result<()> {
    let mut test = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test.push("tests");
    test.push("reference");
    test.push("targets.rs");

    runtest_with_opts(test, Some("atomics"), |command| {
        command
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .env(
                "RUSTFLAGS",
                "-Ctarget-feature=+atomics \
                -Clink-args=--shared-memory \
                -Clink-args=--max-memory=1073741824 \
                -Clink-args=--import-memory \
                -Clink-args=--export=__wasm_init_tls \
                -Clink-args=--export=__tls_size \
                -Clink-args=--export=__tls_align \
                -Clink-args=--export=__tls_base",
            )
            .arg("-Zbuild-std=std,panic_abort");
    })
}

#[test]
fn runtest_targets_mvp() -> Result<()> {
    let mut test = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test.push("tests");
    test.push("reference");
    test.push("targets.rs");

    runtest_with_opts(test, Some("mvp"), |command| {
        command
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .env("RUSTFLAGS", "-C target-cpu=mvp")
            .arg("-Zbuild-std=std,panic_abort");
    })
}

fn runtest_with_opts(
    test: PathBuf,
    suffix: Option<&str>,
    f: impl FnOnce(&mut Command),
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

    let stem = test.file_stem().unwrap().to_str().unwrap();

    let mut name = stem.replace('-', "_") + "_reftest";

    if let Some(suffix) = suffix {
        name.push('_');
        name.push_str(suffix);
    }

    let mut project = Project::new(name);

    // parse additional dependency declarations
    project.dep("wasm-bindgen-futures = { path = '{root}/crates/futures' }");

    f(&mut project.cargo_cmd);

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

        let out_dir = project
            .wasm_bindgen(&format!("{flags} --out-name reference_test"))
            .unwrap();

        // suffix the file name with the sanitized flags
        let test = if all_flags.len() > 1 {
            let mut base_file_name = stem.to_owned();

            for chunk in flags.split(|c: char| !c.is_ascii_alphanumeric()) {
                if !chunk.is_empty() {
                    base_file_name.push('-');
                    base_file_name.push_str(chunk);
                }
            }

            if let Some(suffix) = suffix {
                base_file_name.push('-');
                base_file_name.push_str(suffix);
            }

            test.with_file_name(base_file_name)
        } else {
            test.to_owned()
        };

        let mut sanitizer = Sanitizer::default();

        // These targets care about the `_bg.js` output.
        if target == "bundler" {
            let bg = fs::read_to_string(out_dir.join("reference_test_bg.js"))?;
            sanitizer.assert_same(&bg, &test.with_extension("bg.js"))?;
        }

        let js = fs::read_to_string(out_dir.join("reference_test.js"))?;
        sanitizer.assert_same(&js, &test.with_extension("js"))?;

        let wat = sanitize_wasm(out_dir.join("reference_test_bg.wasm"))?;
        sanitizer.assert_same(&wat, &test.with_extension("wat"))?;

        let d_ts = fs::read_to_string(out_dir.join("reference_test.d.ts"))?;
        sanitizer.assert_same(&d_ts, &test.with_extension("d.ts"))?;
    }

    Ok(())
}

fn sanitize_wasm(wasm: PathBuf) -> Result<String> {
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
        // The function table comes from LLVM and has different size between platforms.
        if table.element_ty == walrus::RefType::FUNCREF {
            table.initial = 0;
            table.maximum = None;
        }
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
