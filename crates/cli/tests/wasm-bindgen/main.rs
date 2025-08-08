//! A small test suite for the `wasm-bindgen` CLI command itself
//!
//! This test suite is intended to exercise functionality of the CLI in terms of
//! errors and such. It is not intended for comprehensive behavior testing, as
//! that should all be placed in the top-level `tests` directory for the
//! `wasm-bindgen` crate itself.
//!
//! Assertions about errors in `wasm-bindgen` or assertions about the output of
//! `wasm-bindgen` should all be placed in this test suite, however. Currently
//! it is largely based off actually running `cargo build` at test time which is
//! quite expensive, so it's recommended that this test suite doesn't become too
//! large!

use assert_cmd::prelude::*;
use predicates::str;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use wasmparser::Payload;

fn target_dir() -> PathBuf {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    dir
}

fn repo_root() -> PathBuf {
    let mut repo_root = env::current_dir().unwrap();
    repo_root.pop(); // remove 'cli'
    repo_root.pop(); // remove 'crates'
    repo_root
}

struct Project {
    root: PathBuf,
    name: &'static str,
}

impl Project {
    fn new(name: &'static str) -> Project {
        let root = target_dir().join("cli-tests").join(name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        Project { root, name }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::write(&dst, contents).unwrap();
        self
    }

    fn wasm_bindgen(&mut self, args: &str) -> (Command, PathBuf) {
        let wasm = self.build();
        let output = self.root.join("pkg");
        fs::create_dir_all(&output).unwrap();
        let mut cmd = Command::cargo_bin("wasm-bindgen").unwrap();
        cmd.arg("--out-dir").arg(&output);
        cmd.arg(&wasm);
        for arg in args.split_whitespace() {
            cmd.arg(arg);
        }
        (cmd, output)
    }

    fn build(&mut self) -> PathBuf {
        if !self.root.join("Cargo.toml").is_file() {
            self.file(
                "Cargo.toml",
                &format!(
                    "
                        [package]
                        name = \"{}\"
                        authors = []
                        version = \"1.0.0\"
                        edition = '2021'

                        [dependencies]
                        wasm-bindgen = {{ path = '{}' }}

                        [lib]
                        crate-type = ['cdylib']

                        [workspace]
                    ",
                    self.name,
                    repo_root().display(),
                ),
            );
        }

        let target_dir = target_dir();
        Command::new("cargo")
            .current_dir(&self.root)
            .arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .env("CARGO_TARGET_DIR", &target_dir)
            .assert()
            .success();

        target_dir
            .join("wasm32-unknown-unknown")
            .join("debug")
            .join(self.name)
            .with_extension("wasm")
    }
}

#[test]
fn version_useful() {
    Command::cargo_bin("wasm-bindgen")
        .unwrap()
        .arg("-V")
        .assert()
        .stdout(str::ends_with("\n"))
        .stdout(str::starts_with("wasm-bindgen "))
        .success();
}

#[test]
fn works_on_empty_project() {
    let (mut cmd, _out_dir) = Project::new("works_on_empty_project")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

#[test]
fn namespace_global_and_noglobal_works() {
    let (mut cmd, _out_dir) = Project::new("namespace_global_and_noglobal_works")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen(module = "fs")]
                extern "C" {
                    #[wasm_bindgen(js_namespace = window)]
                    fn t1();
                }
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = window)]
                    fn t2();
                }
                #[wasm_bindgen]
                pub fn test() {
                    t1();
                    t2();
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

mod npm;

#[test]
fn one_export_works() {
    let (mut cmd, _out_dir) = Project::new("one_export_works")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn foo() {}
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

#[test]
fn bin_crate_works() {
    let (mut cmd, out_dir) = Project::new("bin_crate_works")
        .file(
            "src/main.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(data: &str);
                }

                fn main() {
                    log("hello, world");
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"bin_crate_works\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [workspace]
                ",
                repo_root().display(),
            ),
        )
        .wasm_bindgen("--target nodejs");
    cmd.assert().success();
    Command::new("node")
        .arg("bin_crate_works.js")
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout("hello, world\n");
}

#[test]
fn bin_crate_works_without_name_section() {
    let mut project = Project::new("bin_crate_works_without_name_section");
    project
        .file(
            "src/main.rs",
            r#"
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = console)]
                fn log(data: &str);
            }

            fn main() {
                log("hello, world");
            }
        "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"bin_crate_works_without_name_section\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [workspace]
                ",
                repo_root().display(),
            ),
        );
    let wasm = project.build();

    // Remove the name section from the module.
    // This simulates a situation like #3362 where it fails to parse because one of
    // the names is too long.
    // Unfortunately, we can't use `walrus` to do this because it gives the name
    // section special treatment, so instead we use `wasmparser` directly.
    let mut contents = fs::read(&wasm).unwrap();
    for payload in wasmparser::Parser::new(0).parse_all(&contents.clone()) {
        match payload.unwrap() {
            Payload::CustomSection(reader) if reader.name() == "name" => {
                /// Figures out how many bytes `x` will take up when encoded in
                /// unsigned LEB128.
                fn leb128_len(x: u32) -> usize {
                    match x {
                        0..=0x07f => 1,
                        0x80..=0x3fff => 2,
                        0x4000..=0x1fffff => 3,
                        0x200000..=0xfffffff => 4,
                        0x10000000..=0xffffffff => 5,
                    }
                }

                // Figure out the length of the section header.
                let header_len = 1 + leb128_len(reader.data().len() as u32);

                // Remove the section.
                contents.drain(reader.range().start - header_len..reader.range().end);
            }
            // Ignore everything else.
            _ => {}
        }
    }

    fs::write(&wasm, contents).unwrap();

    // Then run wasm-bindgen on the result.
    let out_dir = project.root.join("pkg");
    fs::create_dir_all(&out_dir).unwrap();
    let mut cmd = Command::cargo_bin("wasm-bindgen").unwrap();
    cmd.arg("--out-dir")
        .arg(&out_dir)
        .arg(&wasm)
        .arg("--target")
        .arg("nodejs");
    cmd.assert().success();

    Command::new("node")
        .arg("bin_crate_works_without_name_section.js")
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout("hello, world\n");
}

#[test]
fn default_module_path_target_web() {
    let (mut cmd, out_dir) = Project::new("default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web");
    cmd.assert().success();
    let contents = fs::read_to_string(out_dir.join("default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('default_module_path_target_web_bg.wasm', import.meta.url);
    }",
    ));
}

#[test]
fn default_module_path_target_no_modules() {
    let (mut cmd, out_dir) = Project::new("default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("default_module_path_target_no_modules.js")).unwrap();
    assert!(contents.contains(
        "\
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }",
    ));
    assert!(contents.contains(
        "\
    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (typeof module_or_path !== 'undefined') {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }

        if (typeof module_or_path === 'undefined' && typeof script_src !== 'undefined') {
            module_or_path = script_src.replace(/\\.js$/, '_bg.wasm');
        }",
    ));
}

#[test]
fn omit_default_module_path_target_web() {
    let (mut cmd, out_dir) = Project::new("omit_default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web --omit-default-module-path");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }


    const imports = __wbg_get_imports();",
    ));
}

#[test]
fn omit_default_module_path_target_no_modules() {
    let (mut cmd, out_dir) = Project::new("omit_default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules --omit-default-module-path");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_no_modules.js")).unwrap();
    assert!(contents.contains(
        "\
    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (typeof module_or_path !== 'undefined') {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }


        const imports = __wbg_get_imports();",
    ));
}

#[test]
fn function_table_preserved() {
    let (mut cmd, _out_dir) = Project::new("function_table_preserved")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn bar() {
                    Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

#[test]
fn function_table_preserved_for_stack_closures() {
    let (mut cmd, _out_dir) = Project::new("function_table_preserved_for_stack_closures")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    fn take_closure(closure: &dyn Fn());
                }

                #[wasm_bindgen]
                pub extern fn pass_closure() {
                    take_closure(&|| {
                        // Noop, just ensure that the compilation succeeds.
                        // See https://github.com/wasm-bindgen/wasm-bindgen/issues/4119.
                    });
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

#[test]
fn constructor_cannot_return_option_struct() {
    let (mut cmd, _out_dir) = Project::new("constructor_cannot_return_option_struct")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct Foo(());

                #[wasm_bindgen]
                impl Foo {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> Option<Foo> {
                        Some(Foo(()))
                    }
                }
            "#,
        )
        .wasm_bindgen("--target web");
    cmd.assert().failure();
}
