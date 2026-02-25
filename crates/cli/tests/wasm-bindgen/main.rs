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

mod npm;
mod reference;

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::str;
use std::env;
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use wasmparser::Payload;

static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    dir
});

static REPO_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut repo_root = env::current_dir().unwrap();
    repo_root.pop(); // remove 'cli'
    repo_root.pop(); // remove 'crates'
    repo_root
});

struct Project {
    root: PathBuf,
    name: String,
    deps: String,
    cargo_cmd: Command,
    built: bool,
}

impl Project {
    fn new(name: impl Into<String>) -> Project {
        let name = name.into();
        let root = TARGET_DIR.join("cli-tests").join(&name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        let mut cargo_cmd = Command::new("cargo");
        cargo_cmd
            .current_dir(&root)
            .arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .env("CARGO_TARGET_DIR", &*TARGET_DIR);
        Project {
            root,
            name,
            deps: "wasm-bindgen = { path = '{root}' }\n".to_owned(),
            cargo_cmd,
            built: false,
        }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::write(&dst, contents).unwrap();
        self
    }

    fn file_link(&mut self, name: &str, src: &Path) -> &mut Project {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(src, &dst).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(src, &dst).unwrap();
        self
    }

    fn wasm_bindgen(&mut self, args: &str) -> anyhow::Result<PathBuf> {
        let output = self.root.join("pkg").join({
            let mut hasher = DefaultHasher::new();
            args.hash(&mut hasher);
            hasher.finish().to_string()
        });
        fs::create_dir_all(&output).unwrap();
        wasm_bindgen_cli::wasm_bindgen::run_cli_with_args(
            [
                "wasm-bindgen".as_ref(),
                "--out-dir".as_ref(),
                output.as_os_str(),
                self.build().as_os_str(),
            ]
            .into_iter()
            .chain(args.split_whitespace().map(str::as_ref)),
        )?;
        Ok(output)
    }

    fn dep(&mut self, line: &str) -> &mut Project {
        self.deps.push_str(line);
        self.deps.push('\n');
        self
    }

    fn build(&mut self) -> PathBuf {
        if !self.built {
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
                        {}

                        [lib]
                        crate-type = ['cdylib']

                        [workspace]

                        [profile.dev]
                        codegen-units = 1
                    ",
                        self.name,
                        self.deps.replace("{root}", REPO_ROOT.to_str().unwrap())
                    ),
                );
            }

            self.cargo_cmd.assert().success();

            self.built = true;
        }

        let mut built = TARGET_DIR.to_path_buf();
        built.push("wasm32-unknown-unknown");
        built.push("debug");
        built.push(&self.name);
        built.set_extension("wasm");

        built
    }
}

#[test]
fn version_useful() {
    cargo_bin_cmd!("wasm-bindgen")
        .arg("-V")
        .assert()
        .stdout(str::ends_with("\n"))
        .stdout(str::starts_with("wasm-bindgen "))
        .success();
}

#[test]
fn works_on_empty_project() {
    Project::new("works_on_empty_project")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn namespace_global_and_noglobal_works() {
    Project::new("namespace_global_and_noglobal_works")
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
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn one_export_works() {
    Project::new("one_export_works")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn foo() {}
            "#,
        )
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn bin_crate_works() {
    let out_dir = Project::new("bin_crate_works")
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
                REPO_ROOT.display(),
            ),
        )
        .wasm_bindgen("--target nodejs")
        .unwrap();

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
                REPO_ROOT.display(),
            ),
        );
    let wasm = &*project.build();

    // Remove the name section from the module.
    // This simulates a situation like #3362 where it fails to parse because one of
    // the names is too long.
    // Unfortunately, we can't use `walrus` to do this because it gives the name
    // section special treatment, so instead we use `wasmparser` directly.
    let mut contents = fs::read(wasm).unwrap();
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

    fs::write(wasm, contents).unwrap();

    // Then run wasm-bindgen on the result.
    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    Command::new("node")
        .arg("bin_crate_works_without_name_section.js")
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout("hello, world\n");
}

#[test]
fn default_module_path_target_web() {
    let out_dir = Project::new("default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();

    let contents = fs::read_to_string(out_dir.join("default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('default_module_path_target_web_bg.wasm', import.meta.url);
    }",
    ));
}

#[test]
fn default_module_path_target_no_modules() {
    let out_dir = Project::new("default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules")
        .unwrap();

    let contents =
        fs::read_to_string(out_dir.join("default_module_path_target_no_modules.js")).unwrap();
    assert!(contents
        .contains("script_src = new URL(document.currentScript.src, location.href).toString();",));
    assert!(contents.contains("module_or_path = script_src.replace(",));
}

#[test]
fn omit_default_module_path_target_web() {
    let out_dir = Project::new("omit_default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web --omit-default-module-path")
        .unwrap();

    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
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
    let out_dir = Project::new("omit_default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules --omit-default-module-path")
        .unwrap();

    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_no_modules.js")).unwrap();
    assert!(contents.contains(
        "\
    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (module_or_path !== undefined) {
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
    Project::new("function_table_preserved")
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
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn function_table_preserved_for_stack_closures() {
    Project::new("function_table_preserved_for_stack_closures")
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
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn constructor_cannot_return_option_struct() {
    Project::new("constructor_cannot_return_option_struct")
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
        .wasm_bindgen("--target web")
        .unwrap_err();
}

#[test]
fn abort_reinit() {
    let mut project = Project::new("abort_reinit");
    project
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::throw_str;

                #[wasm_bindgen(inline_js = "export function js_throw_error() { throw new Error('JS import threw'); }")]
                extern "C" {
                    // No `catch` — a JS throw here should unwind, not abort.
                    fn js_throw_error();
                }

                static mut COUNTER: u32 = 0;

                #[wasm_bindgen]
                pub fn simple_add(a: u32, b: u32) -> u32 {
                    a + b
                }

                #[wasm_bindgen]
                pub fn increment_counter() {
                    unsafe { COUNTER += 1; }
                }

                #[wasm_bindgen]
                pub fn get_counter() -> u32 {
                    unsafe { COUNTER }
                }

                #[wasm_bindgen]
                pub fn trigger_unreachable() {
                    #[cfg(target_arch = "wasm32")]
                    unsafe { core::arch::wasm32::unreachable(); }
                }

                #[wasm_bindgen]
                pub fn trigger_panic() {
                    panic!("deliberate panic");
                }

                #[wasm_bindgen]
                pub fn trigger_throw_str() {
                    throw_str("deliberate throw_str");
                }

                #[wasm_bindgen]
                pub fn call_throwing_import() {
                    js_throw_error();
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"abort_reinit\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}', features = ['abort-reinit'] }}

                    [lib]
                    crate-type = ['cdylib']

                    [workspace]

                    [profile.dev]
                    codegen-units = 1
                ",
                REPO_ROOT.display(),
            ),
        );

    // abort-reinit requires panic=unwind and nightly build-std
    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    // Write the Node.js test script into the output directory
    fs::write(
        out_dir.join("test_abort_reinit.js"),
        r#"
const assert = require('node:assert/strict');

// Monkeypatch WebAssembly.Instance to capture the wasm exports (memory and
// __terminated_address) before the generated JS module hides them.
let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};

const wasm = require('./abort_reinit.js');
WebAssembly.Instance = OrigInstance;

// Test 1: Basic functionality works
assert.strictEqual(wasm.simple_add(2, 3), 5);
console.log('Test 1 passed: basic functionality works');

// Test 2: Recoverable exception (panic) does not trigger reinit, state preserved
wasm.increment_counter();
wasm.increment_counter();
assert.strictEqual(wasm.get_counter(), 2);

assert.throws(() => wasm.trigger_panic(), (e) => {
    assert(!(e instanceof WebAssembly.RuntimeError), 'panic should not be WebAssembly.RuntimeError');
    assert.match(e.message || String(e), /deliberate panic/);
    return true;
});

assert.strictEqual(wasm.get_counter(), 2, 'counter should be preserved after panic (no reinit)');
assert.strictEqual(wasm.simple_add(10, 20), 30);
console.log('Test 2 passed: panic is recoverable, state preserved');

// Test 3: Recoverable exception (throw_str) does not trigger reinit, state preserved
assert.strictEqual(wasm.get_counter(), 2);

assert.throws(() => wasm.trigger_throw_str(), (e) => {
    assert(!(e instanceof WebAssembly.RuntimeError), 'throw_str should not be WebAssembly.RuntimeError');
    const msg = (typeof e === 'string') ? e : (e.message || String(e));
    assert.match(msg, /deliberate throw_str/);
    return true;
});

assert.strictEqual(wasm.get_counter(), 2, 'counter should be preserved after throw_str (no reinit)');
assert.strictEqual(wasm.simple_add(7, 8), 15);
console.log('Test 3 passed: throw_str is recoverable, state preserved');

// Test 4: JS throw from non-catch import does not trigger reinit, state preserved
assert.throws(() => wasm.call_throwing_import(), (e) => {
    assert(!(e instanceof WebAssembly.RuntimeError), 'JS throw should not be WebAssembly.RuntimeError');
    assert.match(e.message || String(e), /JS import threw/);
    return true;
});

assert.strictEqual(wasm.get_counter(), 2, 'counter should be preserved after import throw (no reinit)');
console.log('Test 4 passed: import throw is recoverable, state preserved');

// Test 5: Fatal error triggers reinit, state is reset, wasm works after
wasm.increment_counter();
assert.strictEqual(wasm.get_counter(), 3);

// Read __terminated_address from the captured wasm exports before abort
const terminatedAddr = wasmExports.__terminated_address;
const oldMemory = new Int32Array(wasmExports.memory.buffer);
assert.strictEqual(oldMemory[terminatedAddr / 4], 0, '__terminated_address should be 0 before abort');

assert.throws(() => wasm.trigger_unreachable(), (e) => {
    assert(e instanceof WebAssembly.RuntimeError, 'fatal error should be WebAssembly.RuntimeError');
    return true;
});

// The old memory view still points at the pre-reinit buffer where
// __terminated_address was set to 1 by the catch handler.
assert.strictEqual(oldMemory[terminatedAddr / 4], 1, '__terminated_address should be 1 after abort');

assert.strictEqual(wasm.get_counter(), 0, 'counter should be reset to 0 after reinit');
assert.strictEqual(wasm.simple_add(100, 200), 300);
console.log('Test 5 passed: fatal error triggers reinit, state reset, wasm works');

console.log('All abort-reinit tests passed!');
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("test_abort_reinit.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("All abort-reinit tests passed!"));
}
