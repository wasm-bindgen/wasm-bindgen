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

mod diagnostics;
mod npm;
mod reference;

use assert_cmd::Command;
use predicates::str;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;
use walrus::{ModuleConfig, RawCustomSection};
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

// Every `Project` must have a unique name: the name keys its build dir, wasm
// artifact, and `pkg` output, all of which live under one shared target dir.
// Two projects sharing a name clobber each other when tests run in parallel.
static PROJECT_NAMES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);

struct Project {
    root: PathBuf,
    name: String,
    deps: String,
    cargo_cmd: Command,
    target: String,
    built: bool,
}

impl Project {
    fn new(name: impl Into<String>) -> Project {
        let name = name.into();
        assert!(
            PROJECT_NAMES.lock().unwrap().insert(name.clone()),
            "duplicate Project name {name:?}: each test must use a unique name so \
             their build dirs and wasm artifacts don't collide in parallel",
        );
        let root = TARGET_DIR.join("cli-tests").join(&name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        let mut cargo_cmd = Command::new("cargo");
        cargo_cmd
            .current_dir(&root)
            .arg("build")
            .env("CARGO_TARGET_DIR", &*TARGET_DIR);
        Project {
            root,
            name,
            deps: "wasm-bindgen = { path = '{root}' }\n".to_owned(),
            cargo_cmd,
            target: "wasm32-unknown-unknown".to_owned(),
            built: false,
        }
    }

    /// Override the build target (defaults to `wasm32-unknown-unknown`).
    fn target(&mut self, target: impl Into<String>) -> &mut Project {
        self.target = target.into();
        self
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

            self.cargo_cmd.arg("--target").arg(&self.target);
            self.cargo_cmd.assert().success();

            self.built = true;
        }

        let mut built = TARGET_DIR.to_path_buf();
        built.push(&self.target);
        built.push("debug");
        built.push(&self.name);
        built.set_extension("wasm");

        built
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

fn assert_no_placeholder_imports(wasm: &Path) {
    let module = ModuleConfig::new().parse_file(wasm).unwrap();
    let placeholder_imports: Vec<_> = module
        .imports
        .iter()
        .filter(|i| i.module == "__wbindgen_placeholder__")
        .map(|i| i.name.clone())
        .collect();
    assert!(
        placeholder_imports.is_empty(),
        "wasm32-wasip1 module has unresolved __wbindgen_placeholder__ imports: {placeholder_imports:?}"
    );
}

/// On WASI targets `wasm-bindgen` must emit panicking stubs rather than
/// `__wbindgen_placeholder__` imports, otherwise the resulting module cannot be
/// linked into a component. Build a crate exercising the codegen paths that
/// would emit those imports (an imported function, an exported function, and
/// `JsValue` traffic) and assert the module is free of any
/// `__wbindgen_placeholder__` imports.
#[test]
fn wasi_target_has_no_placeholder_imports() {
    let mut project = Project::new("wasi_target_has_no_placeholder_imports");
    let wasm = project
        .target("wasm32-wasip1")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    fn alert(s: &str);
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(v: &JsValue);
                }
                #[wasm_bindgen]
                pub fn greet(name: &str) {
                    alert(name);
                    log(&JsValue::from_str(name));
                }
            "#,
        )
        .build()
        .to_owned();

    assert_no_placeholder_imports(&wasm);
}

/// Same as above but for `panic = "unwind"`, which is supported on WASI and
/// pulls in `wasm-bindgen-futures`' unwinding `future_to_promise` and the
/// `__wbindgen_panic_error` placeholder import. These gates must also stub out
/// on WASI, otherwise the module either fails to link or fails to compile.
#[test]
fn wasi_target_has_no_placeholder_imports_panic_unwind() {
    let mut project = Project::new("wasi_target_has_no_placeholder_imports_panic_unwind");
    project
        .target("wasm32-wasip1")
        .dep("js-sys = { path = '{root}/crates/js-sys' }")
        .dep("wasm-bindgen-futures = { path = '{root}/crates/futures' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                use js_sys::Promise;
                #[wasm_bindgen]
                pub fn make_promise() -> Promise {
                    wasm_bindgen_futures::future_to_promise(async { Ok(JsValue::UNDEFINED) })
                }
            "#,
        );
    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");
    let wasm = project.build().to_owned();

    assert_no_placeholder_imports(&wasm);
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

/// Shared Rust source for termination / reset-state tests.
const TERMINATION_LIB_RS: &str = r#"
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::throw_str;

                #[wasm_bindgen(inline_js = "
                    export function js_throw_error() { throw new Error('JS import threw'); }
                    export function set_was_dropped(val) { globalThis.was_dropped = val; }
                    let _callback = null;
                    export function register_callback(f) { _callback = f; }
                    export function js_call_callback_with_catch() {
                        try { _callback(); } catch(e) {}
                    }
                ")]
                extern "C" {
                    fn js_throw_error();
                    fn set_was_dropped(val: bool);
                    fn register_callback(f: &JsValue);
                    fn js_call_callback_with_catch();
                }

                #[wasm_bindgen]
                pub fn setup_nested_unreachable() {
                    let closure: Closure<dyn Fn()> = Closure::own_assert_unwind_safe(|| {
                        trigger_unreachable();
                    });
                    register_callback(closure.as_ref());
                    closure.forget();
                }

                struct DropGuard;

                impl DropGuard {
                    fn new() -> Self {
                        set_was_dropped(false);
                        DropGuard
                    }
                }

                impl Drop for DropGuard {
                    fn drop(&mut self) {
                        set_was_dropped(true);
                    }
                }

                static mut COUNTER: u32 = 0;

                #[wasm_bindgen]
                pub fn increment_counter() -> u32 {
                    unsafe {
                        COUNTER += 1;
                        COUNTER
                    }
                }

                #[wasm_bindgen]
                pub fn get_counter() -> u32 {
                    unsafe { COUNTER }
                }

                #[wasm_bindgen]
                pub fn simple_add(a: u32, b: u32) -> u32 {
                    a + b
                }

                #[wasm_bindgen]
                pub fn trigger_unreachable() {
                    let _guard = DropGuard::new();
                    #[cfg(target_arch = "wasm32")]
                    unsafe { core::arch::wasm32::unreachable(); }
                }

                #[wasm_bindgen]
                pub fn trigger_panic() {
                    let _guard = DropGuard::new();
                    panic!("deliberate panic");
                }

                #[wasm_bindgen]
                pub fn trigger_throw_str() {
                    let _guard = DropGuard::new();
                    throw_str("deliberate throw_str");
                }

                #[wasm_bindgen]
                pub fn call_throwing_import() {
                    let _guard = DropGuard::new();
                    js_throw_error();
                }

                #[wasm_bindgen]
                pub fn call_throwing_import_indirect() {
                    let _guard = DropGuard::new();
                    let f = std::hint::black_box(js_throw_error as fn());
                    f();
                }

                #[wasm_bindgen]
                pub fn call_nested_unreachable() {
                    let _guard = DropGuard::new();
                    js_call_callback_with_catch();
                }
            "#;

#[test]
fn termination() {
    let mut project = Project::new("termination");
    project.file("src/lib.rs", TERMINATION_LIB_RS).file(
        "Cargo.toml",
        &format!(
            "
                    [package]
                    name = \"termination\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [lib]
                    crate-type = ['cdylib']

                    [workspace]

                    [profile.dev]
                    codegen-units = 1
                ",
            REPO_ROOT.display(),
        ),
    );

    // termination detection requires panic=unwind and nightly build-std
    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    // Write the Node.js test script into the output directory
    fs::write(
        out_dir.join("test_termination.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

// Monkeypatch WebAssembly.Instance to capture the wasm exports (memory and
// __instance_terminated) before the generated JS module hides them.
let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};

const wasm = require('./termination.js');
WebAssembly.Instance = OrigInstance;
function isTerminated() {
    const memory = new Int32Array(wasmExports.memory.buffer);
    const terminatedAddr = wasmExports.__instance_terminated.value;
    return memory[terminatedAddr / 4];
}

describe('termination', () => {
    it('basic functionality works', () => {
        assert.strictEqual(wasm.simple_add(2, 3), 5);
        assert.strictEqual(isTerminated(), 0);
    });

    it('panic is recoverable and drops locals', () => {
        assert.throws(() => wasm.trigger_panic(), (e) => {
            assert.match(e.message, /deliberate panic/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
    });

    it('throw_str is recoverable and drops locals', () => {
        assert.throws(() => wasm.trigger_throw_str(), (e) => {
            assert.match(e.message, /deliberate throw_str/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
    });

    it('JS import throw is recoverable and drops locals', () => {
        assert.throws(() => wasm.call_throwing_import(), (e) => {
            assert.match(e.message, /JS import threw/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
    });

    it('JS import throw via indirect call is recoverable and drops locals', () => {
        assert.throws(() => wasm.call_throwing_import_indirect(), (e) => {
            assert.match(e.message, /JS import threw/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
    });

    it('fatal error triggers termination without dropping locals', () => {
        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert(e instanceof WebAssembly.RuntimeError, 'fatal error should be WebAssembly.RuntimeError');
            return true;
        });
        assert.strictEqual(isTerminated(), 1);
        assert.strictEqual(globalThis.was_dropped, false);
    });

    it('exports throw Module terminated after fatal error', () => {
        assert.strictEqual(isTerminated(), 1);
        assert.throws(() => wasm.simple_add(1, 2), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_termination.js")
        .current_dir(&out_dir)
        .assert()
        .success();

    // Test that JS can write to terminatedAddr from top-level code to
    // terminate the instance, and that exports then throw "Module terminated".
    fs::write(
        out_dir.join("test_js_terminate_toplevel.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};

const wasm = require('./termination.js');
WebAssembly.Instance = OrigInstance;

describe('JS-initiated termination from top-level', () => {
    it('writing to terminatedAddr from JS makes exports throw Module terminated', () => {
        // Sanity: exports work before termination.
        assert.strictEqual(wasm.simple_add(2, 3), 5);

        // Terminate from JS by writing to the flag.
        const memory = new Int32Array(wasmExports.memory.buffer);
        const terminatedAddr = wasmExports.__instance_terminated.value;
        memory[terminatedAddr / 4] = 1;

        // Now every export should throw "Module terminated".
        assert.throws(() => wasm.simple_add(1, 2), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
        assert.throws(() => wasm.trigger_panic(), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_js_terminate_toplevel.js")
        .current_dir(&out_dir)
        .assert()
        .success();

    // Test that setting the terminated flag from a JS import (inside a wasm
    // frame) prevents drop guards from running on the outer Rust frame.
    fs::write(
        out_dir.join("test_js_terminate_in_wasm.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};

const wasm = require('./termination.js');
WebAssembly.Instance = OrigInstance;

function terminate() {
    const memory = new Int32Array(wasmExports.memory.buffer);
    const terminatedAddr = wasmExports.__instance_terminated.value;
    memory[terminatedAddr / 4] = 1;
}

describe('JS-initiated termination inside wasm frame', () => {
    it('setting terminated flag from JS import callback skips drop', () => {
        // Register a callback that sets the terminated flag and throws,
        // simulating a fatal condition detected from the JS side.
        wasm.setup_nested_unreachable();

        // call_nested_unreachable creates a DropGuard (sets was_dropped=false),
        // then calls js_call_callback_with_catch. The registered callback calls
        // trigger_unreachable which hits wasm unreachable — this sets the
        // terminated flag via the runtime. The outer DropGuard must NOT run.
        globalThis.was_dropped = undefined;
        assert.throws(() => wasm.call_nested_unreachable());
        assert.strictEqual(globalThis.was_dropped, false,
            'outer DropGuard must not have been dropped');

        // Verify the instance is now terminated.
        const memory = new Int32Array(wasmExports.memory.buffer);
        const terminatedAddr = wasmExports.__instance_terminated.value;
        assert.strictEqual(memory[terminatedAddr / 4], 1);

        // Further exports should throw Module terminated.
        assert.throws(() => wasm.simple_add(1, 2), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_js_terminate_in_wasm.js")
        .current_dir(&out_dir)
        .assert()
        .success();

    // Separate test file for nested unreachable: a Rust export calls a JS
    // import that calls back into wasm's trigger_unreachable inside a
    // try/catch. The outer Rust export's DropGuard must NOT be dropped.
    fs::write(
        out_dir.join("test_nested_unreachable.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

const wasm = require('./termination.js');

describe('nested unreachable', () => {
    it('outer drop guard is not executed when inner call hits unreachable', () => {
        wasm.setup_nested_unreachable();
        assert.throws(() => wasm.call_nested_unreachable(), (e) => {
            assert(e instanceof WebAssembly.RuntimeError);
            assert.match(e.message, /unreachable/);
            return true;
        });
        assert.strictEqual(globalThis.was_dropped, false);
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_nested_unreachable.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

/// Regression test for a `Task` leak in the futures executor: when a spawned
/// future panics during `poll`, the executor must drop the future (and
/// everything it owns) rather than leaving it alive via the
/// `Task -> Inner -> future -> Waker -> Rc<Task>` reference cycle. The panic
/// propagates out of the microtask that drives the first poll, so the Node
/// script installs an `uncaughtException` handler to observe it while checking
/// that the future's `DropGuard` ran.
#[test]
fn spawn_local_panic_frees_future() {
    let mut project = Project::new("spawn_local_panic_frees_future");
    project
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                use wasm_bindgen_futures::spawn_local;
                use std::future::Future;
                use std::pin::Pin;
                use std::task::{Context, Poll, Waker};

                #[wasm_bindgen(inline_js = "
                    export function set_was_dropped(v) { globalThis.was_dropped = v; }
                ")]
                extern "C" {
                    fn set_was_dropped(v: bool);
                }

                struct DropGuard;

                impl Drop for DropGuard {
                    fn drop(&mut self) {
                        set_was_dropped(true);
                    }
                }

                // Retains its own waker before panicking, forming the
                // `future -> Waker -> Rc<Task>` cycle that the single-threaded
                // executor would otherwise leak on a panicking poll.
                struct PanicFuture {
                    _guard: DropGuard,
                    waker: Option<Waker>,
                }

                impl Future for PanicFuture {
                    type Output = ();
                    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
                        self.waker = Some(cx.waker().clone());
                        panic!("spawned future panic");
                    }
                }

                #[wasm_bindgen]
                pub fn spawn_panicking_future() {
                    set_was_dropped(false);
                    spawn_local(PanicFuture {
                        _guard: DropGuard,
                        waker: None,
                    });
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"spawn_local_panic_frees_future\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{root}' }}
                    wasm-bindgen-futures = {{ path = '{root}/crates/futures' }}

                    [lib]
                    crate-type = ['cdylib']

                    [workspace]

                    [profile.dev]
                    codegen-units = 1
                ",
                root = REPO_ROOT.display(),
            ),
        );

    // The leak fix matters when a poll unwinds, which requires building with
    // nightly build-std under `panic = "unwind"`.
    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    fs::write(
        out_dir.join("test_spawn_panic.js"),
        r#"
const assert = require('node:assert/strict');
const wasm = require('./spawn_local_panic_frees_future.js');

// The spawned future's panic propagates out of the microtask that drives its
// first poll, surfacing as an uncaught exception. That's expected here; record
// it and swallow only that specific panic so the process keeps running.
let sawPanic = false;
process.on('uncaughtException', (e) => {
    const msg = (e && e.message) || String(e);
    if (/spawned future panic/.test(msg)) {
        sawPanic = true;
    } else {
        throw e;
    }
});

wasm.spawn_panicking_future();

// A macrotask runs after the microtask queue, so by now the spawned task has
// been polled (and panicked).
setTimeout(() => {
    try {
        assert.ok(sawPanic, 'expected the spawned future to panic');
        assert.strictEqual(
            globalThis.was_dropped,
            true,
            'spawned future must be dropped after a panicking poll, not leaked',
        );
        console.log('PASS');
        process.exit(0);
    } catch (e) {
        console.error(e);
        process.exit(1);
    }
}, 50);
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("test_spawn_panic.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(str::contains("PASS"));
}

#[test]
fn termination_reset_state() {
    let mut project = Project::new("termination_reset_state");
    project.file("src/lib.rs", TERMINATION_LIB_RS).file(
        "Cargo.toml",
        &format!(
            "
                    [package]
                    name = \"termination_reset_state\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [lib]
                    crate-type = ['cdylib']

                    [workspace]

                    [profile.dev]
                    codegen-units = 1
                ",
            REPO_ROOT.display(),
        ),
    );

    // termination detection requires panic=unwind and nightly build-std
    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-reset-state-function")
        .unwrap();

    fs::write(
        out_dir.join("test_reset_state.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

// Monkeypatch WebAssembly.Instance to capture the wasm exports (memory and
// __instance_terminated) before the generated JS module hides them.
let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};

const wasm = require('./termination_reset_state.js');
function isTerminated() {
    const memory = new Int32Array(wasmExports.memory.buffer);
    const terminatedAddr = wasmExports.__instance_terminated.value;
    return memory[terminatedAddr / 4];
}

describe('termination with reset state', () => {
    it('basic functionality works', () => {
        assert.strictEqual(wasm.simple_add(2, 3), 5);
        assert.strictEqual(isTerminated(), 0);
    });

    it('counter state is preserved across normal calls', () => {
        assert.strictEqual(wasm.get_counter(), 0);
        assert.strictEqual(wasm.increment_counter(), 1);
        assert.strictEqual(wasm.increment_counter(), 2);
        assert.strictEqual(wasm.get_counter(), 2);
    });

    it('panic is recoverable and preserves counter state', () => {
        assert.throws(() => wasm.trigger_panic(), (e) => {
            assert.match(e.message, /deliberate panic/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
        // Counter preserved across recoverable error.
        assert.strictEqual(wasm.get_counter(), 2);
    });

    it('throw_str is recoverable and preserves counter state', () => {
        assert.strictEqual(wasm.increment_counter(), 3);
        assert.throws(() => wasm.trigger_throw_str(), (e) => {
            assert.match(e.message, /deliberate throw_str/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
        assert.strictEqual(wasm.get_counter(), 3);
    });

    it('JS import throw is recoverable and preserves counter state', () => {
        assert.throws(() => wasm.call_throwing_import(), (e) => {
            assert.match(e.message, /JS import threw/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
        assert.strictEqual(wasm.get_counter(), 3);
    });

    it('JS import throw via indirect call is recoverable and preserves counter state', () => {
        assert.throws(() => wasm.call_throwing_import_indirect(), (e) => {
            assert.match(e.message, /JS import threw/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
        assert.strictEqual(wasm.get_counter(), 3);
    });

    it('fatal error triggers termination without dropping locals', () => {
        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert(e instanceof WebAssembly.RuntimeError, 'fatal error should be WebAssembly.RuntimeError');
            return true;
        });
        assert.strictEqual(isTerminated(), 1);
        assert.strictEqual(globalThis.was_dropped, false);
    });

    it('after fatal error, next call throws "Module terminated"', () => {
        assert.strictEqual(isTerminated(), 1);
        // Without --abort-reinit, calling an export after termination should throw.
        assert.throws(() => wasm.get_counter(), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });

    it('manual reset via __wbg_reset_state() works', () => {
        assert.strictEqual(isTerminated(), 1);
        wasm.__wbg_reset_state();
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(wasm.get_counter(), 0, 'counter should be reset to zero');
        assert.strictEqual(wasm.simple_add(2, 3), 5);
    });

    it('counter works from scratch after reset', () => {
        assert.strictEqual(wasm.increment_counter(), 1);
        assert.strictEqual(wasm.increment_counter(), 2);
        assert.strictEqual(wasm.get_counter(), 2);
    });

    it('recoverable errors still work after a reset', () => {
        assert.throws(() => wasm.trigger_panic(), (e) => {
            assert.match(e.message, /deliberate panic/);
            return true;
        });
        assert.strictEqual(isTerminated(), 0);
        assert.strictEqual(globalThis.was_dropped, true);
        // Counter preserved across recoverable error after reset.
        assert.strictEqual(wasm.get_counter(), 2);
    });

    it('JS-initiated termination throws "Module terminated" on next call', () => {
        assert.strictEqual(wasm.increment_counter(), 3);

        // Terminate from JS by writing to the flag.
        const memory = new Int32Array(wasmExports.memory.buffer);
        const terminatedAddr = wasmExports.__instance_terminated.value;
        memory[terminatedAddr / 4] = 1;

        // Next call should throw — needs explicit reset.
        assert.throws(() => wasm.get_counter(), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });

    it('nested unreachable terminates and throws on next call', () => {
        // Ensure module is not terminated from previous tests
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        wasm.setup_nested_unreachable();
        assert.strictEqual(wasm.increment_counter(), 1);

        globalThis.was_dropped = undefined;
        assert.throws(() => wasm.call_nested_unreachable());
        assert.strictEqual(globalThis.was_dropped, false,
            'outer DropGuard must not have been dropped');
        assert.strictEqual(isTerminated(), 1);

        // Next call should throw — needs explicit reset.
        assert.throws(() => wasm.get_counter(), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
    });

    it('multiple fatal errors require explicit reset each time', () => {
        // Ensure module is not terminated from previous tests
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        for (let i = 0; i < 3; i++) {
            // Build up counter state.
            assert.strictEqual(wasm.increment_counter(), 1);
            assert.strictEqual(wasm.increment_counter(), 2);

            assert.throws(() => wasm.trigger_unreachable(), (e) => {
                assert(e instanceof WebAssembly.RuntimeError);
                return true;
            });
            assert.strictEqual(isTerminated(), 1);

            // Next call throws without explicit reset.
            assert.throws(() => wasm.get_counter(), (e) => {
                assert.match(e.message, /Module terminated/);
                return true;
            });

            // Explicitly reset.
            wasm.__wbg_reset_state();
            assert.strictEqual(isTerminated(), 0);
            assert.strictEqual(wasm.get_counter(), 0, `cycle ${i}: counter should be reset`);
        }
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_reset_state.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

// Lib used for the abort handler and reinit tests — extends TERMINATION_LIB_RS
// with handler setup exports.
const HANDLER_LIB_RS: &str = r#"
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::throw_str;

    #[wasm_bindgen(inline_js = "
        export function js_throw_error() { throw new Error('JS import threw'); }
        export function set_was_dropped(val) { globalThis.was_dropped = val; }
        export function get_js_error() { return new Error('A JS error!'); }
        let _callback = null;
        export function register_callback(f) { _callback = f; }
        export function js_call_callback_with_catch() {
            try { _callback(); } catch(e) {}
        }
    ")]
    extern "C" {
        fn js_throw_error();
        fn set_was_dropped(val: bool);
        fn register_callback(f: &JsValue);
        fn js_call_callback_with_catch();
        fn get_js_error() -> JsValue;
    }

    #[wasm_bindgen(js_namespace = console)]
    extern "C" {
        fn log(data: &str);
    }

    struct DropGuard;
    impl DropGuard {
        fn new() -> Self { set_was_dropped(false); DropGuard }
    }
    impl Drop for DropGuard {
        fn drop(&mut self) { set_was_dropped(true); }
    }

    static mut COUNTER: u32 = 0;

    #[wasm_bindgen]
    pub fn get_counter() -> u32 { unsafe { COUNTER } }

    #[wasm_bindgen]
    pub fn increment_counter() -> u32 {
        unsafe { COUNTER += 1; COUNTER }
    }

    #[wasm_bindgen]
    pub fn simple_add(a: u32, b: u32) -> u32 { a + b }

    #[wasm_bindgen]
    pub fn trigger_unreachable() {
        let _guard = DropGuard::new();
        #[cfg(target_arch = "wasm32")]
        unsafe { core::arch::wasm32::unreachable(); }
    }

    #[wasm_bindgen]
    pub fn trigger_nested_unreachable() {
        let closure: Closure<dyn Fn()> = Closure::own_assert_unwind_safe(|| {
            trigger_unreachable();
        });
        register_callback(closure.as_ref());
        closure.forget();
        // log("This should  happen");
        js_call_callback_with_catch();
        // log("This shouldn't happen");
    }

    #[wasm_bindgen]
    pub fn trigger_panic() {
        let _guard = DropGuard::new();
        panic!("deliberate panic");
    }

    #[wasm_bindgen]
    pub fn call_throwing_import() {
        let _guard = DropGuard::new();
        js_throw_error();
    }

    #[wasm_bindgen]
    pub fn call_throw_val() {
        let val = get_js_error();
        wasm_bindgen::throw_val(val);
    }

    // --- abort handler ---

    #[no_mangle]
    pub static mut __abort_called: u32 = 0;

    fn on_abort() {
        unsafe { __abort_called = 1; }
    }

    fn on_abort_with_reinit() {
        unsafe { __abort_called = 1; }
        wasm_bindgen::handler::schedule_reinit();
    }

    /// Returns true if no previous handler was registered (first registration),
    /// false if one was already set (returned Some).
    #[wasm_bindgen]
    pub fn setup_abort_handler() -> bool {
        wasm_bindgen::handler::set_on_abort(on_abort).is_none()
    }

    /// Sets an abort handler that also calls schedule_reinit().
    #[wasm_bindgen]
    pub fn setup_abort_reinit_handler() -> bool {
        wasm_bindgen::handler::set_on_abort(on_abort_with_reinit).is_none()
    }

    #[wasm_bindgen]
    pub fn signal_reinit() {
        wasm_bindgen::handler::schedule_reinit();
    }
"#;

/// Builds the abort-handler test project, writes a `node:test` harness wrapping
/// `describe_body`, and asserts it passes. Each test must pass a unique `name`
/// so their build dirs, wasm artifacts, and `pkg` output don't collide when run
/// in parallel.
fn run_abort_handler_test(
    name: &str,
    wasm_bindgen_args: &str,
    build_std_unwind: bool,
    describe_body: &str,
) {
    let mut project = Project::new(name);
    project.file("src/lib.rs", HANDLER_LIB_RS).file(
        "Cargo.toml",
        &format!(
            "
                [package]
                name = \"{name}\"
                authors = []
                version = \"1.0.0\"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{repo}' }}

                [lib]
                crate-type = ['cdylib']

                [workspace]

                [profile.dev]
                codegen-units = 1
            ",
            name = name,
            repo = REPO_ROOT.display(),
        ),
    );

    if build_std_unwind {
        // panic=unwind + nightly build-std required for EH catch wrappers
        project
            .cargo_cmd
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .env("RUSTFLAGS", "-Cpanic=unwind")
            .arg("-Zbuild-std=std,panic_unwind");
    }

    let out_dir = project.wasm_bindgen(wasm_bindgen_args).unwrap();

    // Read __abort_called flag directly from linear memory after termination —
    // JS-level exports are blocked but the buffer is still readable.
    let preamble = format!(
        r#"
const {{ describe, it }} = require('node:test');
const assert = require('node:assert/strict');

let wasmExports = null;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {{
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
}};
const wasm = require('./{name}.js');
WebAssembly.Instance = OrigInstance;

function abortCalled() {{
    const addr = wasmExports.__abort_called.value;
    return new Int32Array(wasmExports.memory.buffer)[addr / 4] !== 0;
}}
function isTerminated() {{
    const addr = wasmExports.__instance_terminated.value;
    return new Int32Array(wasmExports.memory.buffer)[addr / 4] !== 0;
}}
"#
    );
    fs::write(
        out_dir.join("test_abort_handler.js"),
        format!("{preamble}{describe_body}"),
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_abort_handler.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

#[test]
fn termination_abort_handler_unwind_panic() {
    run_abort_handler_test(
        "termination_abort_handler_unwind_panic",
        "--target nodejs",
        true,
        r#"
describe('abort handler', () => {
    it('set_on_abort returns true with panic=unwind', () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
    });

    it('handler not called before any fatal error', () => {
        assert.strictEqual(abortCalled(), false);
    });

    it('recoverable panic does not fire the handler', () => {
        assert.throws(() => wasm.trigger_panic(), /deliberate panic/);
        assert.strictEqual(abortCalled(), false);
    });

    it('recoverable JS import throw does not fire the handler', () => {
        assert.throws(() => wasm.call_throwing_import(), /JS import threw/);
        assert.strictEqual(abortCalled(), false);
    });

    it('unreachable fires the handler and terminates the instance', () => {
        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert.ok(e instanceof WebAssembly.RuntimeError);
            return true;
        });
        assert.strictEqual(abortCalled(), true);
        assert.strictEqual(isTerminated(), true);
    });

    it('all exports blocked after termination', () => {
        assert.throws(() => wasm.simple_add(1, 2), /Module terminated/);
    });
});
"#,
    );
}

#[test]
fn termination_abort_handler_unwind_abort1() {
    run_abort_handler_test(
        "termination_abort_handler_unwind_abort1",
        "--target nodejs --force-enable-abort-handler",
        false,
        r#"
describe('abort handler', () => {
    it('set_on_abort returns true with panic=unwind', () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
    });

    it('handler not called before any fatal error', () => {
        assert.strictEqual(abortCalled(), false);
    });

    it('throw_val doesnt fire the handler', () => {
        assert.throws(() => wasm.call_throw_val(), /A JS error/);
        assert.strictEqual(abortCalled(), false);
        assert.strictEqual(isTerminated(), false);
    });

    it('unreachable fires the handler and terminates the instance', () => {
        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert.ok(e instanceof WebAssembly.RuntimeError);
            return true;
        });
        assert.strictEqual(abortCalled(), true);
        assert.strictEqual(isTerminated(), true);
    });

    it('all exports blocked after termination', () => {
        assert.throws(() => wasm.simple_add(1, 2), /Module terminated/);
    });
});
"#,
    );
}

#[test]
fn termination_abort_handler_unwind_abort2() {
    run_abort_handler_test(
        "termination_abort_handler_unwind_abort2",
        "--target nodejs --force-enable-abort-handler",
        false,
        r#"
describe('abort handler', () => {
    it('set_on_abort returns true with panic=unwind', () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
    });

    it('handler not called before any fatal error', () => {
        assert.strictEqual(abortCalled(), false);
    });

    it('JS import throw fires the handler and terminates the instance', () => {
        // Under panic=abort, every JS import throw is a critical error: the
        // catch wrapper calls out to `__wbindgen_rethrow_critical`, which
        // rethrows the original error wrapped in
        // `new Error("Critical error", { cause })`.
        assert.throws(() => wasm.call_throwing_import(), (e) => {
            assert.match(e.message, /Critical error/);
            assert.match(e.cause.message, /JS import threw/);
            return true;
        });
        assert.strictEqual(abortCalled(), true);
        assert.strictEqual(isTerminated(), true);
    });

    it('all exports blocked after termination', () => {
        assert.throws(() => wasm.simple_add(1, 2), /Module terminated/);
    });
});
"#,
    );
}

#[test]
fn termination_abort_handler_unwind_abort3() {
    run_abort_handler_test(
        "termination_abort_handler_unwind_abort3",
        "--target nodejs --force-enable-abort-handler",
        false,
        r#"
describe('abort handler', () => {
    it('set_on_abort returns true with panic=unwind', () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
    });

    it('Trigger nested unreachable', () => {
        assert.throws(() => wasm.trigger_nested_unreachable(), /unreachable/);
        assert.strictEqual(abortCalled(), true);
        assert.strictEqual(isTerminated(), true);
    });

    it('all exports blocked after termination', () => {
        assert.throws(() => wasm.simple_add(1, 2), /Module terminated/);
    });
});
"#,
    );
}

#[test]
fn termination_reinit() {
    let mut project = Project::new("termination_reinit");
    project.file("src/lib.rs", HANDLER_LIB_RS).file(
        "Cargo.toml",
        &format!(
            "
                [package]
                name = \"termination_reinit\"
                authors = []
                version = \"1.0.0\"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}

                [lib]
                crate-type = ['cdylib']

                [workspace]

                [profile.dev]
                codegen-units = 1
            ",
            REPO_ROOT.display(),
        ),
    );

    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-reset-state-function")
        .unwrap();

    fs::write(
        out_dir.join("test_reinit.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

// Monkeypatch WebAssembly.Instance to capture the wasm exports (memory and
// __instance_terminated) before the generated JS module hides them.
let wasmExports = null;
let instanceCount = 0;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    instanceCount++;
    const instance = new OrigInstance(module, imports);
    wasmExports = instance.exports;
    return instance;
};
const wasm = require('./termination_reinit.js');
assert.strictEqual(instanceCount, 1, 'one instance on load');

function isTerminated() {
    const memory = new Int32Array(wasmExports.memory.buffer);
    const terminatedAddr = wasmExports.__instance_terminated.value;
    return memory[terminatedAddr / 4];
}

describe('reinit handler', () => {
    it('signal_reinit then export call creates a new instance', () => {
        wasm.signal_reinit();
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(instanceCount, 2);
    });

    it('reinit resets statics — counter resets to 0', () => {
        // Bump counter so we can prove it resets on reinit.
        wasm.increment_counter();
        wasm.increment_counter();
        // Counter is now > 1 on old instance.
        assert.ok(wasm.get_counter() > 1);
        wasm.signal_reinit();
        wasm.simple_add(0, 0); // __wbg_reset_state -> new instance
        // New instance: statics reset to 0.
        assert.strictEqual(wasm.get_counter(), 0, 'fresh instance: counter reset to 0');
        assert.strictEqual(instanceCount, 3);
    });

    it('counter persists without reinit signal', () => {
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        wasm.increment_counter();
        wasm.increment_counter();
        wasm.increment_counter();
        assert.strictEqual(wasm.get_counter(), 3);
        // No reinit — counter stays at 3.
        wasm.simple_add(0, 0);
        assert.strictEqual(wasm.get_counter(), 3);
    });

    it('multiple reinit cycles each produce a fresh instance with counter=0', () => {
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        const startInstances = instanceCount;
        for (let i = 0; i < 3; i++) {
            // Bump counter to prove it resets.
            wasm.increment_counter();
            wasm.increment_counter();
            wasm.signal_reinit();
            wasm.simple_add(0, 0);
            assert.strictEqual(instanceCount, startInstances + i + 1);
            // Each new instance: counter reset to 0.
            assert.strictEqual(wasm.get_counter(), 0);
        }
    });

    it('hard abort terminates instance and requires explicit reset', () => {
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert.ok(e instanceof WebAssembly.RuntimeError);
            return true;
        });
        assert.throws(() => wasm.simple_add(1, 2), (e) => {
            assert.match(e.message, /Module terminated/);
            return true;
        });
        wasm.__wbg_reset_state();
        assert.strictEqual(wasm.simple_add(1, 2), 3);
    });

    it('host-initiated termination with abort-reinit handler auto-reinits', () => {
        if (isTerminated()) {
            wasm.__wbg_reset_state();
        }
        // Set up an abort handler that calls schedule_reinit().
        wasm.setup_abort_reinit_handler();
        wasm.increment_counter();
        wasm.increment_counter();
        assert.ok(wasm.get_counter() > 1);

        const prevInstances = instanceCount;

        // Terminate from JS by writing to the flag.
        const memory = new Int32Array(wasmExports.memory.buffer);
        const terminatedAddr = wasmExports.__instance_terminated.value;
        memory[terminatedAddr / 4] = 1;

        // Next call should trigger: abort hook -> schedule_reinit() -> reset_state.
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(instanceCount, prevInstances + 1, 'new instance created');
        // Counter reset to 0.
        assert.strictEqual(wasm.get_counter(), 0, 'fresh instance after host-initiated reinit');
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_reinit.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

/// Tests that schedule_reinit() auto-detects without --experimental-reset-state-function.
/// Uses the same HANDLER_LIB_RS which calls schedule_reinit() via signal_reinit(), so
/// the __wbindgen_reinit intrinsic is linked, triggering auto-emission of the
/// private __wbg_reset_state function.
#[test]
fn termination_reinit_auto_detect() {
    let mut project = Project::new("termination_reinit_auto_detect");
    project.file("src/lib.rs", HANDLER_LIB_RS);
    project.file(
        ".cargo/config.toml",
        &format!(
            "
            [patch.crates-io]
            wasm-bindgen = {{ path = '{}' }}

            [profile.dev]
            panic = 'unwind'
            codegen-units = 1
            ",
            REPO_ROOT.display(),
        ),
    );

    project
        .cargo_cmd
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .arg("-Zbuild-std=std,panic_unwind");

    // No --experimental-reset-state-function — reinit is auto-detected.
    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    fs::write(
        out_dir.join("test_reinit_auto.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

let instanceCount = 0;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    instanceCount++;
    return new OrigInstance(module, imports);
};
const wasm = require('./termination_reinit_auto_detect.js');
assert.strictEqual(instanceCount, 1, 'one instance on load');

describe('reinit auto-detection (no --experimental-reset-state-function)', () => {
    it('signal_reinit + call creates a new instance', () => {
        wasm.signal_reinit();
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(instanceCount, 2);
    });

    it('reinit resets counter to 0', () => {
        wasm.increment_counter();
        wasm.increment_counter();
        assert.ok(wasm.get_counter() > 1);
        wasm.signal_reinit();
        wasm.simple_add(0, 0);
        assert.strictEqual(wasm.get_counter(), 0, 'counter reset to 0');
        assert.strictEqual(instanceCount, 3);
    });

    it('abort handler calling schedule_reinit() auto-recovers on next call', () => {
        wasm.setup_abort_reinit_handler();
        wasm.increment_counter();
        const prevInstances = instanceCount;

        assert.throws(() => wasm.trigger_unreachable(), (e) => {
            assert.ok(e instanceof WebAssembly.RuntimeError);
            return true;
        });
        // Abort hook called schedule_reinit(), so next call auto-reinits.
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(instanceCount, prevInstances + 1, 'new instance created');
        assert.strictEqual(wasm.get_counter(), 0, 'fresh instance');
    });

    it('__wbg_reset_state is NOT publicly exported', () => {
        assert.strictEqual(wasm.__wbg_reset_state, undefined);
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_reinit_auto.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

/// Tests that schedule_reinit() works under panic=abort builds.
#[test]
fn reinit_panic_abort() {
    let mut project = Project::new("reinit_panic_abort");
    project
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                static mut COUNTER: u32 = 0;

                #[wasm_bindgen]
                pub fn get_counter() -> u32 { unsafe { COUNTER } }

                #[wasm_bindgen]
                pub fn increment_counter() -> u32 {
                    unsafe { COUNTER += 1; COUNTER }
                }

                #[wasm_bindgen]
                pub fn simple_add(a: u32, b: u32) -> u32 { a + b }

                #[wasm_bindgen]
                pub fn signal_reinit() {
                    wasm_bindgen::handler::schedule_reinit();
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                [package]
                name = \"reinit_panic_abort\"
                authors = []
                version = \"1.0.0\"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}

                [lib]
                crate-type = ['cdylib']

                [workspace]
            ",
                REPO_ROOT.display(),
            ),
        );

    let out_dir = project.wasm_bindgen("--target nodejs").unwrap();

    fs::write(
        out_dir.join("test_reinit_abort.js"),
        r#"
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');

let instanceCount = 0;
const OrigInstance = WebAssembly.Instance;
WebAssembly.Instance = function(module, imports) {
    instanceCount++;
    return new OrigInstance(module, imports);
};
const wasm = require('./reinit_panic_abort.js');
assert.strictEqual(instanceCount, 1, 'one instance on load');

describe('schedule_reinit under panic=abort', () => {
    it('signal_reinit then export call creates a new instance', () => {
        wasm.signal_reinit();
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(instanceCount, 2);
    });

    it('reinit resets statics', () => {
        wasm.increment_counter();
        wasm.increment_counter();
        assert.ok(wasm.get_counter() > 1);
        wasm.signal_reinit();
        wasm.simple_add(0, 0);
        assert.strictEqual(wasm.get_counter(), 0, 'counter reset to 0');
        assert.strictEqual(instanceCount, 3);
    });

    it('counter persists without reinit signal', () => {
        wasm.increment_counter();
        wasm.increment_counter();
        wasm.increment_counter();
        assert.strictEqual(wasm.get_counter(), 3);
        wasm.simple_add(0, 0);
        assert.strictEqual(wasm.get_counter(), 3);
    });

    it('multiple reinit cycles each produce a fresh instance', () => {
        const startInstances = instanceCount;
        for (let i = 0; i < 3; i++) {
            wasm.increment_counter();
            wasm.increment_counter();
            wasm.signal_reinit();
            wasm.simple_add(0, 0);
            assert.strictEqual(instanceCount, startInstances + i + 1);
            assert.strictEqual(wasm.get_counter(), 0);
        }
    });
});
"#,
    )
    .unwrap();

    Command::new("node")
        .arg("--test")
        .arg("test_reinit_abort.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

#[test]
fn multiple_start_functions() {
    let out_dir = Project::new("multiple_start_functions")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(data: &str);
                }

                #[wasm_bindgen(start)]
                fn start1() {
                    log("start1");
                }

                #[wasm_bindgen(start)]
                fn start2() {
                    log("start2");
                }
            "#,
        )
        .wasm_bindgen("--target nodejs")
        .unwrap();

    Command::new("node")
        .arg("-e")
        .arg("require('./multiple_start_functions.js')")
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout(str::contains("start1"))
        .stdout(str::contains("start2"));
}

#[test]
fn private_start_function() {
    let out_dir = Project::new("private_start_function")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(data: &str);
                }

                #[wasm_bindgen(start, private)]
                fn my_start() {
                    log("started");
                }

                #[wasm_bindgen]
                pub fn greet() -> String {
                    "hello".to_string()
                }
            "#,
        )
        .wasm_bindgen("--target nodejs")
        .unwrap();

    // The start function should run but not be exported
    Command::new("node")
        .arg("-e")
        .arg(
            "const m = require('./private_start_function.js'); \
              console.log(typeof m.my_start); \
              console.log(m.greet());",
        )
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout("started\nundefined\nhello\n");
}

#[test]
fn private_namespaced_classes_export_actual_ts_identifier() {
    let mut project = Project::new("private_namespaced_classes_export_actual_ts_identifier");
    let out_dir = project
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(private, js_namespace = foo, js_name = "Point")]
                pub struct FooPoint {
                    pub x: i32,
                }

                #[wasm_bindgen(private, js_namespace = bar, js_name = "Point")]
                pub struct BarPoint {
                    pub y: i32,
                }

                #[wasm_bindgen(js_namespace = foo)]
                pub fn make_foo() -> FooPoint {
                    FooPoint { x: 1 }
                }

                #[wasm_bindgen(js_namespace = bar)]
                pub fn make_bar() -> BarPoint {
                    BarPoint { y: 2 }
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap();

    let d_ts = fs::read_to_string(
        out_dir.join("private_namespaced_classes_export_actual_ts_identifier.d.ts"),
    )
    .unwrap();

    assert!(d_ts.contains("export type { foo__Point };"));
    assert!(d_ts.contains("export type { bar__Point };"));
}

#[test]
fn emscripten_namespaced_exports_valid_ts() {
    // Covers all three TS-emission bugs for namespaced (`js_namespace`)
    // exports in emscripten output:
    //   * mangled identifier (`app__math__Calc`) leaking as a public type,
    //   * spurious unqualified `Calc:` property on BindgenModule,
    //   * `export let app: { ... }` ending up inside the interface body
    //     (TS1131 — invalid syntax).
    //
    // Includes the original repro shape (deep `["app", "math"]`
    // namespace with a struct + impl carrying a constructor and a
    // method), plus same-`js_name` collisions across namespaces (which
    // require the mangled identifier as the disambiguator inside the
    // namespace shape), plus a namespaced enum and free function (same
    // emission path).
    let mut project = Project::new("emscripten_namespaced_exports_valid_ts");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            // Original repro: deep namespace, struct + impl, constructor + method.
            #[wasm_bindgen(js_namespace = ["app", "math"])]
            pub struct Calc {
                value: i32,
            }

            #[wasm_bindgen(js_namespace = ["app", "math"])]
            impl Calc {
                #[wasm_bindgen(constructor)]
                pub fn new(initial: i32) -> Calc {
                    Calc { value: initial }
                }
                pub fn double(&self) -> i32 {
                    self.value * 2
                }
            }

            // Same-`js_name` across namespaces must not collide.
            #[wasm_bindgen(js_namespace = foo, js_name = "Point")]
            pub struct FooPoint {
                pub x: i32,
            }

            #[wasm_bindgen(js_namespace = bar, js_name = "Point")]
            pub struct BarPoint {
                pub y: i32,
            }

            // Namespaced enum + free function share the namespaced-export
            // emission path; cover them in the same fixture.
            #[wasm_bindgen(js_namespace = ["app", "math"])]
            pub enum Op {
                Add = 0,
                Sub = 1,
            }

            #[wasm_bindgen(js_namespace = ["app", "math"])]
            pub fn pi() -> f64 {
                3.14
            }
        "#,
    );

    let built = project.build();
    let mut module = ModuleConfig::new().parse_file(&built).unwrap();
    module.customs.add(RawCustomSection {
        name: "__wasm_bindgen_emscripten_marker".into(),
        data: vec![1],
    });

    let emscripten_wasm = project.root.join("emscripten_input.wasm");
    module.emit_wasm_file(&emscripten_wasm).unwrap();

    let out_dir = project.root.join("pkg-emscripten");
    fs::create_dir_all(&out_dir).unwrap();
    wasm_bindgen_cli::wasm_bindgen::run_cli_with_args([
        "wasm-bindgen".as_ref(),
        "--out-dir".as_ref(),
        out_dir.as_os_str(),
        emscripten_wasm.as_os_str(),
    ])
    .unwrap();

    let d_ts_path = out_dir.join("emscripten_input.d.ts");
    let d_ts = fs::read_to_string(&d_ts_path).unwrap();

    // --- Bug 1: mangled identifier must stay module-internal. ---
    // `declare class` keeps the type reachable inside the .d.ts (so the
    // namespace shape can write `typeof app__math__Calc`) without
    // exporting the mangled name to consumers.
    for mangled in [
        "app__math__Calc",
        "foo__Point",
        "bar__Point",
        "app__math__Op",
    ] {
        let declare_class = format!("declare class {mangled}");
        let declare_enum = format!("declare enum {mangled}");
        assert!(
            d_ts.contains(&declare_class) || d_ts.contains(&declare_enum),
            "expected `declare class|enum {mangled}` in .d.ts, got:\n{d_ts}"
        );
        let export_class = format!("export class {mangled}");
        let export_enum = format!("export enum {mangled}");
        assert!(
            !d_ts.contains(&export_class),
            "mangled identifier `{mangled}` must not be `export`-ed"
        );
        assert!(
            !d_ts.contains(&export_enum),
            "mangled identifier `{mangled}` must not be `export`-ed"
        );
    }

    // --- Bug 2a: no direct unqualified entries on BindgenModule for
    // namespaced items. They are only reachable via the namespace.
    // Check the *top-level* of the interface body (4-space indent only)
    // — nested occurrences inside `app: { math: { ... } }` are fine
    // and required. ---
    let interface_body = d_ts
        .split_once("interface BindgenModule {")
        .and_then(|(_, rest)| rest.split_once("\n}"))
        .map(|(body, _)| body)
        .expect("BindgenModule interface body");
    let top_level: String = interface_body
        .lines()
        .filter(|line| {
            // Top-level entries inside the interface are indented by
            // exactly 4 spaces (the `for line in self.typescript.lines()`
            // splicer adds 2 spaces, on top of any indent the line
            // already carried). Nested-namespace entries indent deeper.
            line.starts_with("    ") && !line.starts_with("        ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in [
        "Calc: typeof",
        "Point: typeof",
        "Op: typeof",
        "pi: typeof",
        "pi(): number",
    ] {
        assert!(
            !top_level.contains(forbidden),
            "BindgenModule top-level must not carry `{forbidden}`; \
             namespaced items belong inside the namespace shape.\n\
             top-level lines:\n{top_level}\n\nfull body:\n{interface_body}"
        );
    }

    // --- Bug 2b: namespace shape must be an interface member, not an
    // `export let` statement (TS1131: Property or signature expected). ---
    assert!(
        !interface_body.contains("export let"),
        "`export let` inside an interface body is invalid TS:\n{interface_body}"
    );
    assert!(
        !interface_body.contains(" let "),
        "`let` declarations are invalid inside an interface body:\n{interface_body}"
    );
    // The top-level namespaces must appear as interface members.
    assert!(interface_body.contains("app: {"));
    assert!(interface_body.contains("foo: {"));
    assert!(interface_body.contains("bar: {"));

    // The nested namespace shape must preserve depth: `app: { math: { Calc, Op, pi } }`.
    // Use the mangled identifier in the `typeof` reference (the
    // disambiguator that survives namespace collisions).
    assert!(
        d_ts.contains("Calc: typeof app__math__Calc"),
        ".d.ts is missing nested `Calc: typeof app__math__Calc`:\n{d_ts}"
    );
    assert!(
        d_ts.contains("Op: typeof app__math__Op"),
        ".d.ts is missing nested `Op: typeof app__math__Op`:\n{d_ts}"
    );
    assert!(
        d_ts.contains("pi: typeof app__math__pi"),
        ".d.ts is missing nested `pi: typeof app__math__pi`:\n{d_ts}"
    );

    // The constructor + method on the namespaced class must reach the
    // mangled `declare class` body unchanged.
    assert!(
        d_ts.contains("constructor(initial: number)"),
        ".d.ts is missing constructor signature:\n{d_ts}"
    );
    assert!(
        d_ts.contains("double(): number"),
        ".d.ts is missing method signature:\n{d_ts}"
    );

    // Same-`js_name` classes in different namespaces must coexist via
    // their mangled identifiers under each namespace shape.
    assert!(d_ts.contains("Point: typeof foo__Point"));
    assert!(d_ts.contains("Point: typeof bar__Point"));
    assert!(!d_ts.contains("Point: typeof Point"));

    // --- End-to-end TS validity: parse the .d.ts with `tsc --noEmit
    // --strict`. Substring assertions can't catch every shape of
    // TS-invalid emission; this is the canonical check. CI installs
    // `typescript` globally before `cargo test` runs; locally the test
    // skips gracefully when tsc isn't on PATH. ---
    if let Some(tsc) = which("tsc") {
        let status = std::process::Command::new(tsc)
            .args([
                "--noEmit",
                "--strict",
                "--skipLibCheck",
                "--lib",
                "esnext,dom",
            ])
            .arg(&d_ts_path)
            .status()
            .expect("failed to invoke tsc");
        assert!(
            status.success(),
            "`tsc --noEmit --strict` rejected the generated .d.ts at {}",
            d_ts_path.display()
        );
    } else {
        eprintln!(
            "skipping tsc validation of {} (tsc not on PATH)",
            d_ts_path.display()
        );
    }
}

#[test]
fn emscripten_exports_hoisted_to_library_symbols() {
    // Clean exports are hoisted into top-level `addToLibrary` symbols and
    // self-registered so emscripten emits them itself.
    let mut project = Project::new("emscripten_exports_hoisted_to_library_symbols");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            #[wasm_bindgen]
            pub enum Color {
                Red = 0,
                Green = 1,
            }

            // A private class must stay module-internal: hoisted, but not
            // attached to Module nor self-registered as a public export.
            #[wasm_bindgen(private)]
            pub struct Secret {
                value: i32,
            }

            #[wasm_bindgen]
            impl Secret {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Secret {
                    Secret { value: 0 }
                }
            }

            #[wasm_bindgen]
            pub struct Counter {
                value: i32,
            }

            #[wasm_bindgen]
            impl Counter {
                #[wasm_bindgen(constructor)]
                pub fn new(start: i32) -> Counter {
                    Counter { value: start }
                }
                pub fn inc(&mut self) -> i32 {
                    self.value += 1;
                    self.value
                }
            }
        "#,
    );

    let built = project.build();
    let mut module = ModuleConfig::new().parse_file(&built).unwrap();
    module.customs.add(RawCustomSection {
        name: "__wasm_bindgen_emscripten_marker".into(),
        data: vec![1],
    });
    let emscripten_wasm = project.root.join("emscripten_input.wasm");
    module.emit_wasm_file(&emscripten_wasm).unwrap();

    let out_dir = project.root.join("pkg-emscripten");
    fs::create_dir_all(&out_dir).unwrap();
    wasm_bindgen_cli::wasm_bindgen::run_cli_with_args([
        "wasm-bindgen".as_ref(),
        "--out-dir".as_ref(),
        out_dir.as_os_str(),
        emscripten_wasm.as_os_str(),
    ])
    .unwrap();

    let lib = fs::read_to_string(out_dir.join("library_bindgen.js")).unwrap();

    // Free function hoisted to its own library symbol with Module attachment.
    assert!(
        lib.contains("$add: function add("),
        "add should be a hoisted library function:\n{lib}"
    );
    assert!(
        lib.contains("$add__postset: \"Module['add'] = add;\""),
        "add should attach to Module via __postset:\n{lib}"
    );
    // Class + its finalization registry hoisted to library symbols.
    assert!(
        lib.contains("$Counter: class Counter"),
        "Counter should be a hoisted library class:\n{lib}"
    );
    assert!(
        lib.contains("$CounterFinalization:"),
        "Counter's finalization registry should be a sibling library symbol:\n{lib}"
    );
    assert!(
        lib.contains("$Counter__deps: ['$initBindgen', '$CounterFinalization']"),
        "Counter should depend on $initBindgen + its finalizer:\n{lib}"
    );
    // Enum hoisted as a string-valued symbol so the freeze is emitted verbatim.
    assert!(
        lib.contains(r#"$Color: "Object.freeze("#),
        "Color enum should be a hoisted string-valued library symbol:\n{lib}"
    );
    // Finalization registry built in a __postset to avoid re-serialization.
    assert!(
        lib.contains("$CounterFinalization: undefined,")
            && lib.contains(
                r#"$CounterFinalization__postset: "CounterFinalization = (typeof FinalizationRegistry"#
            ),
        "CounterFinalization should be constructed in a __postset:\n{lib}"
    );
    // Self-registration so emscripten exports them itself.
    for name in ["add", "Counter", "Color"] {
        assert!(
            lib.contains(&format!("EXPORTED_FUNCTIONS.add('{name}');")),
            "{name} should be self-registered into EXPORTED_FUNCTIONS:\n{lib}"
        );
    }
    // No force-keep: EXPORTED_FUNCTIONS membership already retains each symbol.
    assert!(
        !lib.contains("extraLibraryFuncs.push('$add'"),
        "public exports should rely on EXPORTED_FUNCTIONS, not a redundant force-keep:\n{lib}"
    );
    assert!(
        lib.contains("$Counter__deps: ['$initBindgen', '$CounterFinalization']"),
        "Counter must keep its finalization registry via __deps:\n{lib}"
    );
    // A private class is hoisted but must NOT be exposed as a public export.
    assert!(
        lib.contains("$Secret: class Secret"),
        "private class should still be hoisted as a library symbol:\n{lib}"
    );
    assert!(
        !lib.contains("EXPORTED_FUNCTIONS.add('Secret')"),
        "private class must not self-register as a named export:\n{lib}"
    );
    assert!(
        !lib.contains("Module['Secret']"),
        "private class must not attach to Module:\n{lib}"
    );
    // They must no longer be inlined inside the $initBindgen closure.
    let init = lib
        .split_once("$initBindgen: () =>")
        .map(|(_, rest)| rest)
        .expect("$initBindgen present");
    let init_body = init
        .split_once("\n            });")
        .map(|(b, _)| b)
        .unwrap_or(init);
    assert!(
        !init_body.contains("class Counter") && !init_body.contains("function add("),
        "exports must not be inlined in $initBindgen:\n{init_body}"
    );
}

#[test]
fn emscripten_user_imports_are_prefixed() {
    // User module imports land in the `--extern-pre-js` sidecar at module top
    // level alongside emcc's runtime, and the imported names come verbatim from
    // the user's JS. They're prefixed with `__wbg_` so arbitrary names (e.g. a
    // function literally called `Module`) can't collide with emcc globals. The
    // public export (`run`) stays unprefixed.
    let mut project = Project::new("emscripten_user_imports_are_prefixed");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            // A name that would collide with emscripten's runtime if unprefixed.
            #[wasm_bindgen(module = "imports")]
            extern "C" {
                fn Module() -> i32;
            }

            #[wasm_bindgen(inline_js = "export function snippet_value() { return 7; }")]
            extern "C" {
                fn snippet_value() -> i32;
            }

            #[wasm_bindgen]
            pub fn run() -> i32 {
                Module() + snippet_value()
            }
        "#,
    );

    let built = project.build();
    let mut module = ModuleConfig::new().parse_file(&built).unwrap();
    module.customs.add(RawCustomSection {
        name: "__wasm_bindgen_emscripten_marker".into(),
        data: vec![1],
    });
    let emscripten_wasm = project.root.join("emscripten_input.wasm");
    module.emit_wasm_file(&emscripten_wasm).unwrap();

    let out_dir = project.root.join("pkg-emscripten");
    fs::create_dir_all(&out_dir).unwrap();
    wasm_bindgen_cli::wasm_bindgen::run_cli_with_args([
        "wasm-bindgen".as_ref(),
        "--out-dir".as_ref(),
        out_dir.as_os_str(),
        emscripten_wasm.as_os_str(),
    ])
    .unwrap();

    // ESM imports live in the extern-pre sidecar, aliased to a `__wbg_` local.
    let extern_pre = fs::read_to_string(out_dir.join("library_bindgen.extern-pre.js")).unwrap();
    assert!(
        extern_pre.contains("Module as __wbg_Module"),
        "module import should be aliased to a __wbg_-prefixed local:\n{extern_pre}"
    );
    assert!(
        extern_pre.contains("snippet_value as __wbg_snippet_value"),
        "inline-js snippet import should be aliased to a __wbg_-prefixed local:\n{extern_pre}"
    );
    // The bare user name must NOT bind at module scope (no collision with emcc).
    assert!(
        !extern_pre.contains("import { Module }")
            && !extern_pre.contains("import { Module,")
            && !extern_pre.contains(", Module }"),
        "the unprefixed `Module` must not be imported into module scope:\n{extern_pre}"
    );

    // The library shims reference the prefixed locals.
    let lib = fs::read_to_string(out_dir.join("library_bindgen.js")).unwrap();
    assert!(
        lib.contains("__wbg_Module(") && lib.contains("__wbg_snippet_value("),
        "shims should call the prefixed import locals:\n{lib}"
    );
    // The public export keeps its clean name.
    assert!(
        lib.contains("$run: function run("),
        "the public export `run` must stay unprefixed:\n{lib}"
    );
}

/// Look up an executable on `PATH`. Used so the test can opportunistically
/// validate the generated .d.ts with `tsc` without hard-requiring it.
fn which(name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
