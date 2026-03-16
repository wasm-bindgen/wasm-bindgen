use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = crate_dir.join("tests");

    // Scan test files for @ts-gen directives.
    let tests = discover_tests(&tests_dir, &crate_dir);

    // Generate a lib.rs that includes all generated modules.
    let mut lib_code = String::new();

    for test in &tests {
        generate_bindings(test, &out_dir);
        lib_code.push_str(&format!(
            "include!(concat!(env!(\"OUT_DIR\"), \"/{}.rs\"));\n\n",
            test.out_name
        ));
    }

    let lib_file = out_dir.join("_lib.rs");
    std::fs::write(&lib_file, &lib_code)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", lib_file.display()));

    // Re-run if any test file changes.
    println!("cargo:rerun-if-changed=tests");
}

struct TestEntry {
    /// Rust module name (snake_case, e.g. `es_module_lexer`)
    mod_name: String,
    /// Library/module name for wasm_bindgen (e.g. `es-module-lexer`, `node:console`)
    lib_name: String,
    /// Filesystem-safe output name (e.g. `node-console` for `node:console`)
    out_name: String,
    /// Resolved .d.ts file paths
    dts_paths: Vec<String>,
    /// External type mappings (e.g. `"node:*=node_sys::*"`)
    externals: Vec<String>,
}

/// Scan all `.rs` files in the tests directory for `//! @ts-gen` directives.
///
/// Each directive line mirrors the CLI interface:
///   `//! @ts-gen --lib-name <name> [--external <mapping>]... <path>...`
///
/// Path resolution:
/// - Relative paths (`./`, `../`) → relative to crate root
/// - Everything else → resolved via `oxc_resolver` with `types` condition
fn discover_tests(tests_dir: &Path, crate_dir: &Path) -> Vec<TestEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(tests_dir) else {
        return entries;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }

        let content = std::fs::read_to_string(&path).unwrap_or_default();

        // Collect all @ts-gen directive lines
        let mut args: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("//! @ts-gen ") {
                // Split respecting quoted strings
                args.extend(shell_split(rest));
            }
        }

        if args.is_empty() {
            continue;
        }

        // Parse CLI-style args
        let mut lib_name: Option<String> = None;
        let mut dts_paths: Vec<String> = Vec::new();
        let mut externals: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--lib-name" | "-l" => {
                    i += 1;
                    if i < args.len() {
                        lib_name = Some(args[i].clone());
                    }
                }
                "--external" | "-e" => {
                    i += 1;
                    if i < args.len() {
                        externals.push(args[i].clone());
                    }
                }
                arg if !arg.starts_with('-') => {
                    dts_paths.push(resolve_dts_path(arg, crate_dir));
                }
                _ => {
                    panic!("Unknown @ts-gen flag: {}", args[i]);
                }
            }
            i += 1;
        }

        let Some(lib_name) = lib_name else {
            panic!(
                "Missing --lib-name in @ts-gen directive in {}",
                path.display()
            );
        };
        if dts_paths.is_empty() {
            panic!("No .d.ts paths in @ts-gen directive in {}", path.display());
        }

        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let out_name = lib_name.replace([':', '/'], "-");

        entries.push(TestEntry {
            mod_name: stem,
            lib_name,
            out_name,
            dts_paths,
            externals,
        });
    }

    entries.sort_by(|a, b| a.mod_name.cmp(&b.mod_name));
    entries
}

/// Simple shell-style splitting that respects double quotes.
fn shell_split(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in s.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    result.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

fn resolve_dts_path(directive: &str, crate_dir: &Path) -> String {
    if directive.starts_with("./") || directive.starts_with("../") {
        // Relative to crate root
        crate_dir.join(directive).to_string_lossy().to_string()
    } else {
        // Resolve via oxc_resolver (handles node_modules, @types, exports conditions)
        ts_gen::parse::resolve::resolve(directive, crate_dir)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to resolve module \"{directive}\" from {}",
                    crate_dir.display()
                )
            })
            .to_string_lossy()
            .to_string()
    }
}

fn generate_bindings(test: &TestEntry, out_dir: &Path) {
    let paths: Vec<PathBuf> = test.dts_paths.iter().map(PathBuf::from).collect();
    let path_refs: Vec<&PathBuf> = paths.iter().collect();

    let (module, mut gctx) = ts_gen::parse(&path_refs, Some(&test.lib_name))
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", test.lib_name));

    // Configure external type mappings before codegen.
    for mapping in &test.externals {
        gctx.external_map.add_mapping(mapping);
    }

    // Print parse diagnostics as cargo warnings so they're visible during build.
    for diag in &gctx.diagnostics.diagnostics {
        let level = match diag.level {
            ts_gen::util::diagnostics::DiagnosticLevel::Error => "ERROR",
            ts_gen::util::diagnostics::DiagnosticLevel::Warning => "warning",
            ts_gen::util::diagnostics::DiagnosticLevel::Info => "info",
        };
        println!(
            "cargo:warning=[ts-gen {level}] {}: {}",
            test.lib_name, diag.message
        );
    }

    let options = ts_gen::codegen::GenerateOptions {
        skip_promise_ext: true,
    };
    let rust_code = ts_gen::codegen::generate_with_options(&module, &gctx, &options)
        .unwrap_or_else(|e| panic!("codegen failed: {e}"));

    // The generated code uses inner attributes (#![allow(...)], //!) which are
    // only valid at the crate root. Since we include! this file, strip them and
    // also strip `use wasm_bindgen::prelude::*;` since the parent module provides it.
    let rust_code = strip_inner_attributes(&rust_code);

    let out_file = out_dir.join(format!("{}.rs", test.out_name));
    std::fs::write(&out_file, &rust_code)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", out_file.display()));

    println!("cargo:warning=Generated bindings: {}", out_file.display());

    // Re-run if the .d.ts file changes.
    for path in &test.dts_paths {
        println!("cargo:rerun-if-changed={path}");
    }
}

/// Strip `#![...]` inner attributes and `//!` inner doc comments from
/// generated code (since these are only valid at the crate root and we
/// include! this file into a module).
///
/// Note: we keep `use wasm_bindgen::prelude::*;` because namespace modules
/// need it in scope.
fn strip_inner_attributes(code: &str) -> String {
    code.lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with("#![") && !trimmed.starts_with("//!")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
