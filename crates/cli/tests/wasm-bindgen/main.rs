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
        let wasm = self.build();
        run_wasm_bindgen(&wasm, &self.root.join("pkg"), args)
    }

    fn dep(&mut self, line: &str) -> &mut Project {
        self.deps.push_str(line);
        self.deps.push('\n');
        self
    }

    fn build(&mut self) -> PathBuf {
        if !self.built {
            self.prepare_build();
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

    fn compile_error(&mut self) -> String {
        self.prepare_build();
        let assert = self.cargo_cmd.assert().failure();
        String::from_utf8_lossy(&assert.get_output().stderr).into_owned()
    }

    fn prepare_build(&mut self) {
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
    }
}

/// Run the `wasm-bindgen` CLI (in-process) against an already-built `wasm`
/// artifact, writing its output below `pkg_root` in a directory keyed by a
/// hash of `args` (so distinct flag combinations against the same artifact
/// don't clobber each other's output).
///
/// This is split out from [`Project::wasm_bindgen`] so callers that build
/// their `wasm` artifact some other way (e.g. as part of a batched Cargo
/// workspace shared across many tests, see `reference::REFERENCE_WORKSPACE`)
/// can reuse the exact same post-processing logic.
fn run_wasm_bindgen(wasm: &Path, pkg_root: &Path, args: &str) -> anyhow::Result<PathBuf> {
    let output = pkg_root.join({
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
            wasm.as_os_str(),
        ]
        .into_iter()
        .chain(args.split_whitespace().map(str::as_ref)),
    )?;
    Ok(output)
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

/// A `generic_per_mono` import declared in an upstream crate must still bind
/// when that crate's `extern "C"` block contains *nothing else*.
///
/// The crate's `#[link_section = "__wasm_bindgen_unstable"]` AST metadata lives
/// in an rlib object file, and wasm-ld only pulls an archive member in if
/// something references one of its symbols. The monomorphised shim is
/// instantiated in the downstream crate's CGU, so it references nothing
/// upstream; without the anchoring descriptor export the member is dropped, the
/// AST entry goes missing, and the CLI fails with "generic import
/// monomorphisation references unknown shim".
///
/// This has to be a real two-crate build: a single-crate test cannot reproduce
/// it, since there is no archive member to drop. It also has to be a debug,
/// non-LTO build — `lto = true` merges everything and masks the problem
/// entirely, which is why this regressed without CI noticing.
#[test]
fn cross_crate_generic_per_mono_only_block() {
    let mut project = Project::new("cross_crate_generic_per_mono_only_block");
    project
        .dep("upstream_generic = { path = 'upstream_generic' }")
        .file(
            "upstream_generic/Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"upstream_generic\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = {{ path = '{repo}' }}
                ",
                repo = REPO_ROOT.display(),
            ),
        )
        // Nothing but the generic import: any non-generic import here would
        // emit its own descriptor export and anchor the member by accident,
        // hiding the bug this test exists to catch.
        .file(
            "upstream_generic/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono)]
                    pub fn shared_log<T>(x: T);
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn go() {
                    upstream_generic::shared_log(1u32);
                    upstream_generic::shared_log("two");
                }
            "#,
        );

    let out_dir = project.wasm_bindgen("--target web").unwrap();
    let js =
        fs::read_to_string(out_dir.join("cross_crate_generic_per_mono_only_block.js")).unwrap();

    // One manufactured shim per monomorphisation, each calling the upstream
    // import. `u32` crosses as a number, `&str` as a (ptr, len) pair.
    assert!(
        js.contains("shared_log(arg0 >>> 0)"),
        "missing per-mono binding for the u32 instantiation:\n{js}"
    );
    assert!(
        js.contains("shared_log(getStringFromWasm0(arg0, arg1))"),
        "missing per-mono binding for the &str instantiation:\n{js}"
    );

    // The anchoring descriptor export is interpreted and deleted by
    // `execute_exports`, so none of it may survive into the output.
    assert!(
        !js.contains("__wbindgen_describe"),
        "descriptor export leaked into the generated JS:\n{js}"
    );
}

/// The generated per-monomorphisation shim is expanded with call-site hygiene
/// into the user's own module, so every path it names must be fully qualified.
///
/// An unqualified `core::ptr::read` is shadowed by a user item called `core` in
/// that module, and the expansion then fails with `cannot find function read in
/// module core::ptr` — an error pointing at code the user never wrote, with no
/// hint that their own module is the cause. (A 2015-edition consumer would fail
/// even without the shadowing, since `core::` resolves relative to the crate
/// root there.)
///
/// `core` is the realistic case because the shim reaches for `ptr::read`, but the
/// same applies to any prelude crate name, so `alloc` and `std` are shadowed here
/// too to keep the whole expansion honest.
#[test]
fn generic_per_mono_shim_paths_survive_shadowing() {
    let mut project = Project::new("generic_per_mono_shim_paths_survive_shadowing");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            // User items shadowing the crate names the expansion relies on.
            mod core { pub mod ptr {} pub mod option {} }
            mod alloc {}
            mod std {}

            #[wasm_bindgen]
            extern "C" {
                // Covers the `WasmRet<..>`-returning shim shape.
                #[wasm_bindgen(generic_per_mono)]
                fn echo<T>(x: T) -> T;
                // Covers the unit-returning shim shape, which builds its marker
                // call as a statement rather than a tail expression.
                #[wasm_bindgen(generic_per_mono)]
                fn sink<T>(x: T);
                // Covers the `slice_to_array` rewrite, which names both
                // `alloc::vec::Vec` (in the describe type) and `core::option::Option`
                // (in the ABI conversion for the `Option<&[T]>` shape).
                #[wasm_bindgen(generic_per_mono, slice_to_array)]
                fn take_slice<T>(xs: &[u32], t: T);
                #[wasm_bindgen(generic_per_mono, slice_to_array)]
                fn take_opt_slice<T>(xs: Option<&[u32]>, t: T);
            }

            #[wasm_bindgen]
            pub fn go() {
                let _ = echo(1u32);
                let _ = echo(1.5f64);
                sink(2u32);
                take_slice(&[1u32, 2], 3u32);
                take_opt_slice(None, 4u32);
                take_opt_slice(Some(&[5u32]), 6u32);
            }
        "#,
    );

    // A successful `cargo build` is the assertion: any unqualified path in the
    // generated shim resolves to one of the shadowing modules and fails to
    // compile.
    project.build();
}

/// The per-monomorphisation shim body performs unsafe operations (a `ptr::read`
/// of the descriptor tuple, and the call to the imported shim) inside a function
/// the macro declares `unsafe`.
///
/// `codegen.rs` relies on rustc suppressing `unsafe_op_in_unsafe_fn` — like most
/// lints — inside an external macro's expansion, so a downstream crate denying
/// that lint does not see it fire on code it did not write. That is an
/// assumption about rustc behaviour rather than something the expansion
/// controls, so pin it: if it ever stops holding, this fails at the point the
/// toolchain changes rather than in a user's bug report.
#[test]
fn generic_per_mono_unsafe_op_in_unsafe_fn() {
    let mut project = Project::new("generic_per_mono_unsafe_op_in_unsafe_fn");
    project.file(
        "src/lib.rs",
        r#"
            #![deny(unsafe_op_in_unsafe_fn)]

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                // Covers the `WasmRet<..>`-returning shim shape.
                #[wasm_bindgen(generic_per_mono)]
                fn echo<T>(x: T) -> T;
                // Covers the unit-returning shim shape, which builds its marker
                // call as a statement rather than a tail expression.
                #[wasm_bindgen(generic_per_mono)]
                fn sink<T>(x: T);
                // Covers the `slice_to_array` rewrite, whose ABI conversion is
                // spliced into the same unsafe body.
                #[wasm_bindgen(generic_per_mono, slice_to_array)]
                fn take_slice<T>(xs: &[u32], t: T);
            }

            #[wasm_bindgen]
            pub fn go() {
                let _ = echo(1u32);
                let _ = echo(1.5f64);
                sink(2u32);
                take_slice(&[1u32, 2], 3u32);
            }
        "#,
    );

    // `#![deny(..)]` turns the lint into an error, so a successful build is the
    // assertion.
    project.build();
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

    // Only the public namespace roots carry the `__export`/`__force`
    // attributes; the hoisted leaves stay private, reachable through the
    // root's `__deps`.
    let lib = fs::read_to_string(out_dir.join("library_bindgen.js")).unwrap();
    for root in ["app", "foo", "bar"] {
        assert!(
            lib.contains(&format!("${root}__export: true"))
                && lib.contains(&format!("${root}__force: true")),
            "namespace root {root} should carry __export/__force:\n{lib}"
        );
    }
    for leaf in [
        "app__math__Calc",
        "app__math__Op",
        "app__math__pi",
        "foo__Point",
        "bar__Point",
    ] {
        assert!(
            !lib.contains(&format!("${leaf}__export")) && !lib.contains(&format!("${leaf}__force")),
            "namespace leaf {leaf} must not carry __export/__force:\n{lib}"
        );
        assert!(
            lib.contains(&format!("'${leaf}'")),
            "namespace leaf {leaf} must be reachable via __deps:\n{lib}"
        );
    }

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
    // Clean exports are hoisted into top-level `addToLibrary` symbols carrying
    // `__export`/`__force` attributes so emscripten emits them itself.
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
    // Public exports carry the `__export`/`__force` symbol attributes so
    // emscripten includes and exports them itself.
    for name in ["add", "Counter", "Color"] {
        assert!(
            lib.contains(&format!("${name}__export: true")),
            "{name} should carry __export: true:\n{lib}"
        );
        assert!(
            lib.contains(&format!("${name}__force: true")),
            "{name} should carry __force: true:\n{lib}"
        );
    }
    // No EXPORTED_FUNCTIONS mutation and no extraLibraryFuncs force-keep:
    // the symbol attributes replace both mechanisms.
    assert!(
        !lib.contains("EXPORTED_FUNCTIONS"),
        "generated library must not mutate EXPORTED_FUNCTIONS:\n{lib}"
    );
    assert!(
        !lib.contains("extraLibraryFuncs"),
        "generated library must not push to extraLibraryFuncs:\n{lib}"
    );
    // The init closure roots the graph via __force instead.
    assert!(
        lib.contains("$initBindgen__force: true"),
        "$initBindgen must be force-included:\n{lib}"
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
        !lib.contains("$Secret__export") && !lib.contains("$Secret__force"),
        "private class must not carry __export/__force attributes:\n{lib}"
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
fn emscripten_jspi_codegen() {
    // JSPI on the emscripten target uses exactly the same path as the other
    // targets: the in-wasm shadow-stack instrumentation plus
    // `WebAssembly.promising`/`WebAssembly.Suspending` in the JS glue — with
    // no interaction with emscripten's own JSPI machinery. The emscripten
    // specifics are purely about the JS library format: exports hoist as
    // `async function` symbols, the promising cache is a library symbol, and
    // the suspending import is rewrapped via `__postset` (a Suspending
    // instance can't be stringified through the compile-time jsifier).
    let mut project = Project::new("emscripten_jspi_codegen");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(suspending)]
                fn sleep(ms: u32);
            }

            #[wasm_bindgen(jspi)]
            pub fn do_work() {
                sleep(100);
            }

            #[wasm_bindgen(jspi)]
            pub fn compute() -> u32 {
                sleep(1);
                42
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

    // JSPI exports hoist as async library functions awaiting the promising
    // call, with a lazily-initialized promising cache as a library symbol.
    assert!(
        lib.contains("$do_work: async function do_work("),
        "do_work should hoist as an async library function:\n{lib}"
    );
    assert!(
        lib.contains("$compute: async function compute("),
        "compute should hoist as an async library function:\n{lib}"
    );
    assert!(
        lib.contains("WebAssembly.promising(wasmExports['do_work'])"),
        "do_work should call through WebAssembly.promising:\n{lib}"
    );
    assert!(
        lib.contains("$__wbg_jspi_do_work: \"undefined\""),
        "the promising cache should be a library symbol:\n{lib}"
    );

    // The suspending import stays a plain library function and is rewrapped
    // with `WebAssembly.Suspending` via its `__postset`.
    let suspend_postset = lib
        .lines()
        .find(|l| l.contains("__postset") && l.contains("WebAssembly.Suspending"))
        .unwrap_or_else(|| panic!("missing Suspending __postset:\n{lib}"));
    assert!(
        suspend_postset.contains("__wbg_sleep_"),
        "the Suspending postset should target the sleep import:\n{lib}"
    );

    // The in-wasm instrumentation ran: the fiber base global exists and the
    // jspi exports are wired to their wrappers.
    let out_module = ModuleConfig::new()
        .parse_file(out_dir.join("emscripten_input_bg.wasm"))
        .unwrap();
    assert!(
        out_module
            .globals
            .iter()
            .any(|g| g.name.as_deref() == Some("__jspi_stack_base")),
        "output wasm should contain the __jspi_stack_base global"
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

#[test]
fn generated_paths_survive_shadowed_core_alloc_std() {
    let mut project = Project::new("generated_paths_survive_shadowed_core_alloc_std");
    project.dep("wasm-bindgen-futures = { path = '{root}/crates/futures' }");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            // User items shadowing the crate names the expansion relies on. In
            // 2018+ a `mod core` in scope wins over the extern-prelude `core`,
            // so any unqualified `core::`/`alloc::`/`std::` path in generated
            // code resolves in here and fails to compile.
            mod core { pub mod mem {} pub mod option {} pub mod borrow {} pub mod marker {} }
            mod alloc { pub mod vec {} }
            mod std { pub mod vec {} }

            // Imported type: exercises the phantom-data, `to_js` and
            // `RefFromWasmAbi` (`ManuallyDrop`) shapes.
            #[wasm_bindgen]
            extern "C" {
                type Widget;
                #[wasm_bindgen(constructor)]
                fn new() -> Widget;
                #[wasm_bindgen(method)]
                fn tap(this: &Widget);

                // `slice_to_array` names `alloc::vec::Vec` in the describe type
                // and `core::option::Option` in the ABI conversion.
                #[wasm_bindgen(slice_to_array)]
                fn take_slice(xs: &[u32]);
                #[wasm_bindgen(slice_to_array)]
                fn take_opt_slice(xs: Option<&[u32]>);
            }

            // Exported fn taking `&T` in an `async` body: exercises the
            // `borrow::Borrow` anchor shape.
            #[wasm_bindgen]
            pub struct Held { pub v: u32 }

            #[wasm_bindgen]
            pub async fn hold(h: &Held) -> u32 { h.v }

            #[wasm_bindgen]
            pub fn go() {
                let w = Widget::new();
                w.tap();
                take_slice(&[1u32, 2]);
                take_opt_slice(None);
                take_opt_slice(Some(&[3u32]));
            }
        "#,
    );

    // A successful `cargo build` is the assertion.
    project.build();
}

/// `slice_to_array` only changes codegen for *imported* (`extern "C"`)
/// function arguments; on exported free functions and `#[wasm_bindgen]`
/// impl-block methods it is documented as a no-op, since there's no
/// outgoing-argument conversion for those to redirect. In particular, the
/// compile-time rejection of `slice_to_array` on a `&mut [T]` argument (it
/// would silently discard JS's writes, since an owned `Array` has nowhere to
/// write them back to) must not fire here: on an export, the argument is a
/// completely ordinary `&mut [T]` all along -- JS gets the usual writable
/// typed-array view, so there's nothing to warn about, and the attribute
/// being present at all is a copy/paste artifact rather than user intent.
#[test]
fn slice_to_array_is_a_no_op_on_exported_mut_slice_args() {
    let mut project = Project::new("slice_to_array_is_a_no_op_on_exported_mut_slice_args");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn bump(#[wasm_bindgen(slice_to_array)] xs: &mut [u8]) {
                for x in xs {
                    *x += 1;
                }
            }

            #[wasm_bindgen]
            pub struct Doubler;

            #[wasm_bindgen]
            impl Doubler {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Doubler {
                    Doubler
                }

                pub fn double(&self, #[wasm_bindgen(slice_to_array)] xs: &mut [u16]) {
                    for x in xs {
                        *x *= 2;
                    }
                }
            }
        "#,
    );

    // A successful `cargo build` is the assertion: `slice_to_array` on these
    // `&mut` arguments must stay inert rather than tripping the import-only
    // rejection.
    project.build();
}

/// `slice_to_array` used to name `::std::vec::Vec` in the type it describes
/// through. `::std::` is an absolute path to the `std` *crate*, so unlike the
/// shadowing cases above this was not a hygiene problem -- it simply does not
/// resolve in a `#![no_std]` crate, which `wasm-bindgen` explicitly supports.
/// The generated code now goes through the `alloc` re-export instead.
#[test]
fn slice_to_array_works_in_a_no_std_crate() {
    let mut project = Project::new("slice_to_array_works_in_a_no_std_crate");
    // Written out rather than going through `dep`, because the point of the test
    // is `default-features = false` and `Project` seeds a plain `wasm-bindgen`
    // dependency that would collide with a second entry for the same key.
    project.file(
        "Cargo.toml",
        &format!(
            "
            [package]
            name = 'slice_to_array_works_in_a_no_std_crate'
            version = '1.0.0'
            edition = '2021'

            [dependencies]
            wasm-bindgen = {{ path = '{root}', default-features = false }}

            [lib]
            crate-type = ['cdylib']

            [workspace]

            [profile.dev]
            codegen-units = 1
        ",
            root = REPO_ROOT.display(),
        ),
    );
    project.file(
        "src/lib.rs",
        r#"
            #![no_std]

            extern crate alloc;

            use wasm_bindgen::prelude::*;

            // Minimum viable `no_std` scaffolding. Nothing here is exercised --
            // the crate only has to *link* for the test to mean anything.
            #[panic_handler]
            fn panic(_: &core::panic::PanicInfo) -> ! {
                core::arch::wasm32::unreachable()
            }

            struct Bump;
            unsafe impl core::alloc::GlobalAlloc for Bump {
                unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
                    core::ptr::null_mut()
                }
                unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {}
            }
            #[global_allocator]
            static ALLOC: Bump = Bump;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(slice_to_array)]
                fn take_slice(xs: &[u32]);
                #[wasm_bindgen(slice_to_array)]
                fn take_opt_slice(xs: Option<&[u32]>);
            }

            #[wasm_bindgen]
            pub fn go() {
                take_slice(&[1u32, 2]);
                take_opt_slice(None);
                take_opt_slice(Some(&[3u32]));
            }
        "#,
    );

    // A successful `cargo build` is the assertion.
    project.build();
}

#[test]
fn split_debug_info() {
    /// Collect the name and data of each custom section in a Wasm module.
    fn custom_sections(wasm: &[u8]) -> Vec<(String, Vec<u8>)> {
        wasmparser::Parser::new(0)
            .parse_all(wasm)
            .filter_map(|payload| match payload.unwrap() {
                Payload::CustomSection(s) => Some((s.name().to_string(), s.data().to_vec())),
                _ => None,
            })
            .collect()
    }

    /// Get the contents of the code section of a Wasm module.
    fn code_section(wasm: &[u8]) -> Vec<u8> {
        wasmparser::Parser::new(0)
            .parse_all(wasm)
            .find_map(|payload| match payload.unwrap() {
                Payload::CodeSectionStart { range, .. } => Some(wasm[range].to_vec()),
                _ => None,
            })
            .expect("no code section")
    }

    /// Encode a string as a Wasm name string: a LEB128 length, then the
    /// UTF-8 bytes.
    fn name_string(s: &str) -> Vec<u8> {
        assert!(s.len() < 128);
        let mut bytes = vec![s.len() as u8];
        bytes.extend_from_slice(s.as_bytes());
        bytes
    }

    /// Get the data of the `external_debug_info` custom section, if the
    /// module has one.
    fn external_debug_info(wasm: &[u8]) -> Option<Vec<u8>> {
        let sections = custom_sections(wasm)
            .into_iter()
            .filter(|(name, _)| name == "external_debug_info")
            .collect::<Vec<_>>();
        assert!(sections.len() <= 1);
        sections.into_iter().next().map(|(_, data)| data)
    }

    let mut project = Project::new("split_debug_info");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn add(a: u32, b: u32) -> u32 {
                a + b
            }
        "#,
    );

    // `--split-debug-info` writes the debug info to a second file and
    // records that file's name in the main module by default.
    let out_dir = project
        .wasm_bindgen("--target web --split-debug-info")
        .unwrap();
    let main = fs::read(out_dir.join("split_debug_info_bg.wasm")).unwrap();
    let debug = fs::read(out_dir.join("split_debug_info_bg.debug.wasm")).unwrap();
    wasmparser::validate(&main).unwrap();

    let main_sections = custom_sections(&main);
    assert!(
        main_sections
            .iter()
            .all(|(name, _)| !name.starts_with(".debug_")),
        "main module must not contain DWARF sections"
    );
    assert_eq!(
        external_debug_info(&main),
        Some(name_string("split_debug_info_bg.debug.wasm"))
    );

    // The debug file keeps the DWARF sections and the code section, and
    // does not point at itself.
    assert_eq!(external_debug_info(&debug), None);
    assert_eq!(code_section(&main), code_section(&debug));

    // `--debug-info-url` changes the recorded URL, not the file name.
    let url = "http://localhost:5173/app_bg.debug.wasm";
    let out_dir = project
        .wasm_bindgen(&format!(
            "--target web --split-debug-info --debug-info-url {url}"
        ))
        .unwrap();
    let main = fs::read(out_dir.join("split_debug_info_bg.wasm")).unwrap();
    assert!(out_dir.join("split_debug_info_bg.debug.wasm").is_file());
    assert_eq!(external_debug_info(&main), Some(name_string(url)));

    // `--debug-info-url` requires `--split-debug-info`.
    assert!(project
        .wasm_bindgen("--target web --debug-info-url http://localhost/app.wasm")
        .is_err());

    // Without `--split-debug-info`, the DWARF stays embedded and no debug
    // file or `external_debug_info` section appears.
    let out_dir = project.wasm_bindgen("--target web --keep-debug").unwrap();
    let main = fs::read(out_dir.join("split_debug_info_bg.wasm")).unwrap();
    assert!(!out_dir.join("split_debug_info_bg.debug.wasm").exists());
    assert_eq!(external_debug_info(&main), None);
}

#[test]
fn experimental_memory_discard() {
    /// Whether a module contains a `memory.discard` instruction in any local
    /// function body.
    fn has_memory_discard(module: &walrus::Module) -> bool {
        use walrus::ir::{dfs_in_order, Instr, Visitor};
        use walrus::FunctionKind;

        struct Scan(bool);
        impl<'instr> Visitor<'instr> for Scan {
            fn visit_instr(&mut self, instr: &Instr, _: &walrus::InstrLocId) {
                if matches!(instr, Instr::MemoryDiscard(_)) {
                    self.0 = true;
                }
            }
        }

        let mut scan = Scan(false);
        for func in module.funcs.iter() {
            if let FunctionKind::Local(local) = &func.kind {
                dfs_in_order(&mut scan, local, local.entry_block());
            }
        }
        scan.0
    }

    let mut project = Project::new("experimental_memory_discard");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;

            #[link(wasm_import_module = "env")]
            extern "C" {
                #[link_name = "__wbindgen_memory_discard"]
                fn __wbindgen_memory_discard(addr: usize, len: usize);
            }

            #[wasm_bindgen]
            pub fn purge(addr: usize, len: usize) {
                unsafe { __wbindgen_memory_discard(addr, len) }
            }
        "#,
    );

    // Without the flag, the import is left dangling and `wasm-bindgen` must
    // hard-error rather than emit a module with an unresolvable import.
    let err = project
        .wasm_bindgen("--target web")
        .expect_err("should fail without --experimental-memory-discard");
    assert!(
        format!("{err:#}").contains("--experimental-memory-discard"),
        "{err:#}"
    );

    // With the flag, the import is replaced by a local `memory.discard`
    // trampoline and no longer appears in the import section.
    let out_dir = project
        .wasm_bindgen("--target web --experimental-memory-discard")
        .unwrap_or_else(|e| panic!("{e:#}"));
    let wasm = out_dir.join("experimental_memory_discard_bg.wasm");
    let module = ModuleConfig::new().parse_file(&wasm).unwrap();

    assert!(
        module
            .imports
            .iter()
            .all(|i| !(i.module == "env" && i.name == "__wbindgen_memory_discard")),
        "`__wbindgen_memory_discard` import should have been removed"
    );
    assert!(
        has_memory_discard(&module),
        "output module should contain a `memory.discard` instruction"
    );

    // The emitted `memory.discard` instruction only validates against an
    // engine/parser that understands the memory-control proposal.
    let bytes = fs::read(&wasm).unwrap();
    assert!(
        wasmparser::validate(&bytes).is_err(),
        "plain validation should reject `memory.discard` without the feature"
    );
    let features = wasmparser::WasmFeatures::default() | wasmparser::WasmFeatures::MEMORY_CONTROL;
    wasmparser::Validator::new_with_features(features)
        .validate_all(&bytes)
        .expect("module should validate once memory-control is enabled");
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

// ─── JSPI (JS Promise Integration) runtime tests ─────────────────────────────
//
// These build a JSPI module and run it under a JSPI-capable Node
// (Node ≥ 24; CI's `test_native` job pins 24.15). They exercise the interaction
// between JSPI stack switching and the in-wasm exception-handling / abort-handler
// machinery, which is reasoned about statically elsewhere but only proven here.

/// Shared library for the JSPI runtime tests. Mirrors the `jspi` example but
/// trimmed to the functions the Node harness drives directly.
const JSPI_LIB_RS: &str = r#"
    #![allow(deprecated)]
    use wasm_bindgen::prelude::*;
    use js_sys::Promise;
    use js_sys::futures::jspi_block_on_promise as block_on_promise;

    #[wasm_bindgen(inline_js = "
        export function note_drop() { globalThis.__jspi_drops = (globalThis.__jspi_drops | 0) + 1; }
        export function note_spawn(v) { (globalThis.__jspi_spawns ??= []).push(v); }
    ")]
    extern "C" {
        fn note_drop();
        fn note_spawn(v: u32);
    }

    // Increments a JS-side counter when dropped, so tests can prove destructors
    // run on a resumed-then-unwound fiber stack.
    struct DropGuard;
    impl Drop for DropGuard {
        fn drop(&mut self) { note_drop(); }
    }

    #[no_mangle]
    pub static mut __abort_called: u32 = 0;

    fn on_abort() { unsafe { __abort_called = 1; } }

    #[wasm_bindgen]
    pub fn setup_abort_handler() -> bool {
        wasm_bindgen::handler::set_on_abort(on_abort).is_none()
    }

    #[wasm_bindgen]
    pub fn simple_add(a: u32, b: u32) -> u32 { a + b }

    // Plain (NON-jspi) export that calls a suspending import with no
    // `WebAssembly.promising` frame on the stack — fails with `SuspendError`
    // at the import boundary.
    #[wasm_bindgen]
    pub fn misuse_suspend() {
        let p = Promise::resolve(&JsValue::UNDEFINED);
        let _ = block_on_promise(&p);
    }

    // Happy path: suspend on an already-resolved promise, then resume.
    // Returns 42.
    #[wasm_bindgen(jspi)]
    pub fn do_sleep() -> u32 {
        let p = Promise::resolve(&JsValue::from(41u32));
        let v = block_on_promise(&p).unwrap_throw();
        v.as_f64().unwrap_or(0.0) as u32 + 1
    }

    // Deep recursion, suspend at the bottom, then allocate a `Vec` at every
    // level on the way back up. `deep_alloc(N) == 1000 + N*(N+1)/2`, so
    // `deep_alloc(20) == 1210`. Run concurrently this also proves per-fiber
    // shadow-stack isolation.
    #[wasm_bindgen(jspi)]
    pub fn deep_alloc(depth: u32) -> u32 { deep_alloc_inner(depth) }

    #[inline(never)]
    fn deep_alloc_inner(depth: u32) -> u32 {
        let buf = [depth as u8; 1024];
        let _ = core::hint::black_box(&buf);
        if depth == 0 {
            let p = Promise::resolve(&JsValue::UNDEFINED);
            block_on_promise(&p).unwrap_throw();
            let v: Vec<u32> = vec![1000];
            v[0]
        } else {
            let child = deep_alloc_inner(depth - 1);
            let v: Vec<u32> = vec![depth];
            child + v[0]
        }
    }

    // Panic AFTER a suspend/resume, with a `DropGuard` live across the suspend.
    // Exercises unwind starting from a post-switch native stack with the shadow
    // stack freshly restored by the in-wasm suspending wrapper.
    #[wasm_bindgen(jspi)]
    pub fn panic_after_resume() {
        let _g = DropGuard;
        let p = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&p).unwrap_throw();
        panic!("boom after resume");
    }

    // A rejected non-`catch` suspending import: the exception unwinds through
    // Rust frames — running destructors under panic=unwind — and rejects the
    // promising call's promise with the original reason.
    #[wasm_bindgen(jspi)]
    pub fn reject_no_catch() -> u32 {
        let _g = DropGuard;
        always_rejects()
    }

    // The async-form analog: the rejection unwinds out of a
    // promising-entered poll; the trampoline's dropped promise surfaces it
    // as an unhandled rejection and the task is abandoned.
    #[wasm_bindgen(jspi)]
    pub async fn async_reject_no_catch() -> u32 {
        let _g = DropGuard;
        always_rejects()
    }

    // The async-form analog of `panic_after_resume`: a panic in a poll after
    // a suspend/resume, with a `DropGuard` live across the suspension.
    #[wasm_bindgen(jspi)]
    pub async fn async_panic_after_resume() {
        let _g = DropGuard;
        let p = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&p).unwrap_throw();
        panic!("boom in poll");
    }

    // Rejection path: the suspend wrapper catches the JSTag exception thrown
    // at the resume point and reports it as `Err` data via the
    // `__wbindgen_jspi_rejected` flag. Returns the rejection reason (13).
    #[wasm_bindgen(jspi)]
    pub fn check_rejection() -> u32 {
        let p = Promise::reject(&JsValue::from(13u32));
        match block_on_promise(&p) {
            Err(v) => v.as_f64().unwrap_or(0.0) as u32,
            Ok(_) => 0,
        }
    }

    #[wasm_bindgen(inline_js = "
        export function get_text() { return new Promise(r => setTimeout(() => r('hello world'), 1)); }
        export function flaky(ok) { return ok ? Promise.resolve(41) : Promise.reject(new Error('nope')); }
        export function throws_sync() { throw new Error('sync boom'); }
        export function plain_throw(msg) { throw new Error(msg); }
        export function plain_ok(v) { return v + 1; }
        export function always_rejects() { return Promise.reject(new Error('nocatch nope')); }
    ")]
    extern "C" {
        // Non-externref return: marshalled to String in Rust post-resume.
        #[wasm_bindgen(suspending)]
        fn get_text() -> String;
        // catch + suspending: rejections surface as `Err` data.
        #[wasm_bindgen(catch, suspending)]
        fn flaky(ok: bool) -> Result<u32, JsValue>;
        // A synchronous throw is converted to a rejection by the Suspending
        // shim, so it ticks and surfaces as `Err` like a rejection.
        #[wasm_bindgen(catch, suspending)]
        fn throws_sync() -> Result<u32, JsValue>;
        // Plain (non-suspending) `catch` imports: the ordinary exception
        // machinery (handleError, or wasm catch wrappers under the abort
        // handler), exercised under fibers.
        #[wasm_bindgen(catch)]
        fn plain_throw(msg: &str) -> Result<u32, JsValue>;
        #[wasm_bindgen(catch)]
        fn plain_ok(v: u32) -> Result<u32, JsValue>;
        // Non-`catch` suspending import whose promise always rejects: the
        // JSTag exception is rethrown at the resume point (over a restored
        // shadow stack) and unwinds through the Rust frames.
        #[wasm_bindgen(suspending)]
        fn always_rejects() -> u32;
    }

    #[wasm_bindgen(jspi)]
    pub fn fetch_text_len() -> u32 {
        get_text().len() as u32
    }

    #[wasm_bindgen(jspi)]
    pub fn try_flaky(ok: bool) -> u32 {
        match flaky(ok) {
            Ok(v) => v + 1,
            Err(_) => 13,
        }
    }

    #[wasm_bindgen(jspi)]
    pub fn try_sync_throw() -> u32 {
        match throws_sync() {
            Ok(v) => v,
            Err(_) => 27,
        }
    }

    // Plain-`catch` (non-suspending) imports under a fiber: the exception
    // machinery must work both before and after a suspension.
    #[wasm_bindgen(jspi)]
    pub fn catch_plain_around_suspend() -> u32 {
        let a = match plain_throw("before suspend") {
            Err(_) => 1,
            Ok(_) => 0,
        };
        let p = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&p).unwrap_throw();
        let b = match plain_throw("after suspend") {
            Err(_) => 2,
            Ok(_) => 0,
        };
        a + b + plain_ok(1).unwrap_throw()
    }

    // jspi methods on exported classes emit `async` class methods.
    #[wasm_bindgen]
    pub struct Counter {
        n: u32,
    }

    #[wasm_bindgen]
    impl Counter {
        #[wasm_bindgen(constructor)]
        pub fn new(n: u32) -> Counter {
            Counter { n }
        }

        #[wasm_bindgen(jspi)]
        pub fn add_slept(&self, v: u32) -> u32 {
            let p = Promise::resolve(&JsValue::from(v));
            let got = block_on_promise(&p).unwrap_throw();
            self.n + got.as_f64().unwrap_or(0.0) as u32
        }
    }

    // A Result-returning jspi export: `Err` is thrown at the JS boundary
    // inside the async glue wrapper, rejecting the returned Promise.
    #[wasm_bindgen(jspi)]
    pub fn fallible_fiber(ok: bool) -> Result<u32, JsValue> {
        let p = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&p).unwrap_throw();
        if ok {
            Ok(7)
        } else {
            Err(JsValue::from_str("fiber says no"))
        }
    }

    // A future that wakes itself synchronously *during* poll (exercising the
    // pre-created resolver) and returns Pending a few times before resolving.
    struct SelfWaking { remaining: u32 }
    impl core::future::Future for SelfWaking {
        type Output = u32;
        fn poll(
            mut self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
        ) -> core::task::Poll<u32> {
            if self.remaining == 0 {
                core::task::Poll::Ready(7)
            } else {
                self.remaining -= 1;
                cx.waker().wake_by_ref();
                core::task::Poll::Pending
            }
        }
    }

    // A Rust future awaited from sync jspi code: scheduled on the ordinary
    // microtask executor via `future_to_promise`, with the fiber suspending
    // on the completion promise.
    #[wasm_bindgen(jspi)]
    pub fn drive_future() -> u32 {
        let promise = wasm_bindgen_futures::future_to_promise(async {
            Ok(JsValue::from(SelfWaking { remaining: 3 }.await))
        });
        block_on_promise(&promise).unwrap().as_f64().unwrap() as u32
    }

    // Sync helper that suspends: only callable where the poll runs on a
    // fiber — i.e. from a task spawned within a JSPI context.
    fn sync_add_one(p: &js_sys::Promise) -> u32 {
        block_on_promise(p).unwrap().as_f64().unwrap() as u32 + 1
    }

    // Context inheritance: a *plain* `spawn_local` from inside a jspi
    // export spawns a promising-entered task, whose poll may park
    // mid-frame on a suspending sync call until `gate` settles.
    #[wasm_bindgen(jspi)]
    pub fn spawn_suspending(gate: js_sys::Promise) {
        wasm_bindgen_futures::spawn_local(async move {
            note_spawn(sync_add_one(&gate));
        });
    }

    // Transitive inheritance: a task spawned from a promising-entered
    // poll is itself promising-entered, two levels deep.
    #[wasm_bindgen(jspi)]
    pub fn spawn_nested(gate: js_sys::Promise) {
        wasm_bindgen_futures::spawn_local(async move {
            wasm_bindgen_futures::spawn_local(async move {
                note_spawn(sync_add_one(&gate) + 100);
            });
        });
    }

    // A jspi async export: the same JS contract as a plain async export
    // (the caller receives a Promise), but the body's polls are
    // promising-entered, so sync callees may suspend mid-poll.
    #[wasm_bindgen(jspi)]
    pub async fn async_mixed(a: js_sys::Promise, gate: js_sys::Promise) -> u32 {
        let x = js_sys::futures::JsFuture::from(a).await.unwrap()
            .as_f64().unwrap() as u32;
        x + sync_add_one(&gate)
    }

    // The same `catch` matrix inside a promising-entered poll: a suspending
    // import's rejection and a plain-catch throw both surface as `Err` data
    // mid-poll.
    #[wasm_bindgen(jspi)]
    pub async fn async_try_flaky(ok: bool) -> u32 {
        let plain = match plain_throw("in poll") {
            Err(_) => 100,
            Ok(_) => 0,
        };
        plain + match flaky(ok) {
            Ok(v) => v + 1,
            Err(_) => 13,
        }
    }

    // `Err` from a jspi async export rejects the returned promise, through
    // the internal promising `future_to_promise` reject path.
    #[wasm_bindgen(jspi)]
    pub async fn async_fallible(ok: bool) -> Result<u32, JsValue> {
        let p = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&p).unwrap_throw();
        if ok {
            Ok(9)
        } else {
            Err(JsValue::from_str("async fiber says no"))
        }
    }

    // An ordinary `spawn_local` future on the plain microtask executor:
    // `steps` awaits (one queue poll each), then invokes `done`. Its
    // progress while jspi tasks sit suspended is the proof that a
    // suspended poll parks only its own task, not the executor.
    #[wasm_bindgen]
    pub fn spawn_ordinary(steps: u32, done: js_sys::Function) {
        wasm_bindgen_futures::spawn_local(async move {
            for i in 0..steps {
                let p = js_sys::Promise::resolve(&JsValue::from(i));
                js_sys::futures::JsFuture::from(p).await.unwrap();
                note_spawn(1000 + i);
            }
            done.call0(&JsValue::NULL).unwrap();
        });
    }

    // Returns the address of a shadow-stack local, approximating the empty-
    // stack SP. Used to detect shadow-stack leaks across fiber completions.
    #[wasm_bindgen]
    pub fn sp_probe() -> u32 {
        let buf = [0u8; 16];
        core::hint::black_box(&buf) as *const _ as u32
    }

    // Plain sync export holding a live 4 KiB shadow-stack frame while calling
    // back into JS, so a promising export started by `f` enters with a shadow-
    // stack offset below the true stack top. When that fiber suspends, this
    // frame unwinds; on the fiber's completion the SP must be reset to the
    // stack top, not the entry offset, or the 4 KiB is leaked forever.
    #[wasm_bindgen]
    pub fn call_nested(f: &js_sys::Function) {
        let buf = [0u8; 4096];
        let _ = core::hint::black_box(&buf);
        f.call0(&JsValue::NULL).unwrap_throw();
        let _ = core::hint::black_box(&buf);
    }
"#;

/// Returns the node flags under which the `node` on `PATH` exposes JSPI
/// (`WebAssembly.Suspending`): none on Node 25+ (JSPI on by default; newer
/// Node removes the flag entirely), `--experimental-wasm-jspi` on Node 24.
/// `None` (skip) on older Node, which either rejects the flag (≤ 20) or
/// lacks the API (22).
fn node_jspi_flags() -> Option<&'static [&'static str]> {
    const PROBE: &str = "process.exit(typeof WebAssembly.Suspending === 'function' ? 0 : 1)";
    const FLAGGED: &[&str] = &["--experimental-wasm-jspi"];
    for flags in [&[] as &[&str], FLAGGED] {
        let supported = std::process::Command::new("node")
            .args(flags)
            .args(["-e", PROBE])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if supported {
            return Some(flags);
        }
    }
    None
}

/// Builds the JSPI test project, writes a `node:test` harness wrapping
/// `describe_body`, and asserts it passes under a JSPI-capable `node`.
/// Each test must pass a unique `name`. Pass `panic_unwind` to build with
/// nightly `-Cpanic=unwind -Zbuild-std`, which emits modern EH instructions.
fn run_jspi_test(
    name: &str,
    lib_rs: &str,
    wasm_bindgen_args: &str,
    panic_unwind: bool,
    describe_body: &str,
) {
    let Some(node_flags) = node_jspi_flags() else {
        eprintln!("skipping {name}: the `node` on PATH lacks JSPI (needs Node >= 24)");
        return;
    };

    let mut project = Project::new(name);
    project.file("src/lib.rs", lib_rs).file(
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
                js-sys = {{ path = '{repo}/crates/js-sys' }}
                # Async jspi exports expand to the internal executor via the
                # `wasm_bindgen_futures` path, like all async exports.
                wasm-bindgen-futures = {{ path = '{repo}/crates/futures' }}

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

    if panic_unwind {
        project
            .cargo_cmd
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .env("RUSTFLAGS", "-Cpanic=unwind")
            .arg("-Zbuild-std=std,panic_unwind");
    }

    let out_dir = project.wasm_bindgen(wasm_bindgen_args).unwrap();

    let preamble = format!(
        r#"
const {{ describe, it }} = require('node:test');
const assert = require('node:assert/strict');

// Capture the wasm exports before the generated JS module hides them, so the
// abort/termination flags can be read straight out of linear memory.
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
        out_dir.join("test_jspi.js"),
        format!("{preamble}{describe_body}"),
    )
    .unwrap();

    Command::new("node")
        .args(node_flags)
        .arg("--test")
        .arg("test_jspi.js")
        .current_dir(&out_dir)
        .assert()
        .success();
}

#[test]
fn jspi_runtime_basics() {
    run_jspi_test(
        "jspi_runtime_basics",
        JSPI_LIB_RS,
        "--target nodejs",
        false,
        r#"
describe('jspi runtime', () => {
    it('suspends on a resolved promise and resumes', async () => {
        assert.strictEqual(await wasm.do_sleep(), 42);
    });

    it('restores the shadow SP across deep recursion + post-resume alloc', async () => {
        assert.strictEqual(await wasm.deep_alloc(20), 1210);
    });

    it('keeps per-fiber shadow stacks isolated across concurrent fibers', async () => {
        const r = await Promise.all([wasm.deep_alloc(20), wasm.deep_alloc(20), wasm.deep_alloc(20)]);
        assert.deepStrictEqual(r, [1210, 1210, 1210]);
    });

    it('fails with SuspendError when no promising frame is on the stack', async () => {
        let threw = false;
        try { await wasm.misuse_suspend(); } catch (e) {
            threw = true;
            assert.match(String(e), /SuspendError|promising/);
        }
        assert.ok(threw, 'misuse_suspend should have thrown');
    });

    it('returns promise rejections as data', async () => {
        assert.strictEqual(await wasm.check_rejection(), 13);
    });

    it('marshals non-externref suspending returns post-resume', async () => {
        assert.strictEqual(await wasm.fetch_text_len(), 11);
    });

    it('catch + suspending surfaces fulfillment and rejection as Result', async () => {
        assert.strictEqual(await wasm.try_flaky(true), 42);
        assert.strictEqual(await wasm.try_flaky(false), 13);
    });

    it('catch + suspending converts synchronous throws into Err', async () => {
        assert.strictEqual(await wasm.try_sync_throw(), 27);
    });

    it('plain catch imports work before and after a suspension', async () => {
        assert.strictEqual(await wasm.catch_plain_around_suspend(), 5);
    });

    it('catch works for suspending and plain imports inside a promising-entered poll', async () => {
        assert.strictEqual(await wasm.async_try_flaky(true), 142);
        assert.strictEqual(await wasm.async_try_flaky(false), 113);
    });

    it('Result-returning jspi async exports reject the promise with Err', async () => {
        assert.strictEqual(await wasm.async_fallible(true), 9);
        await assert.rejects(() => wasm.async_fallible(false), /async fiber says no/);
    });

    it('supports jspi methods on classes', async () => {
        const c = new wasm.Counter(10);
        assert.strictEqual(await c.add_slept(5), 15);
    });

    it('Result-returning jspi exports reject the promise with Err', async () => {
        assert.strictEqual(await wasm.fallible_fiber(true), 7);
        await assert.rejects(() => wasm.fallible_fiber(false), /fiber says no/);
    });

    it('mixed fulfillment and rejection across concurrent fibers', async () => {
        const r = await Promise.all([
            wasm.try_flaky(true),
            wasm.check_rejection(),
            wasm.try_flaky(false),
            wasm.deep_alloc(20),
        ]);
        assert.deepStrictEqual(r, [42, 13, 13, 1210]);
    });

    it('awaits a Rust future from sync jspi code via future_to_promise', async () => {
        assert.strictEqual(await wasm.drive_future(), 7);
    });

    it('a jspi async export mixes .await with sync suspension', async () => {
        let resolveGate;
        const gate = new Promise(r => { resolveGate = r; });
        const p = wasm.async_mixed(Promise.resolve(30), gate);
        setTimeout(() => resolveGate(11), 0);
        assert.strictEqual(await p, 42);
    });

    it('context inheritance is transitive across nested spawns', async () => {
        globalThis.__jspi_spawns = [];
        let resolve3;
        const gate3 = new Promise(r => { resolve3 = r; });
        wasm.spawn_nested(gate3);
        setTimeout(() => resolve3(7), 0);
        await new Promise(r => setTimeout(r, 10));
        assert.deepStrictEqual(globalThis.__jspi_spawns, [108]);
    });

    it('suspended polls park only their task: the executor keeps polling', async () => {
        globalThis.__jspi_spawns = [];
        let resolve1, resolve2;
        const gate1 = new Promise(r => { resolve1 = r; });
        const gate2 = new Promise(r => { resolve2 = r; });
        // Two tasks REALLY suspended mid-poll, in parallel — spawned with
        // plain spawn_local from inside a jspi export (context inheritance).
        wasm.spawn_suspending(gate1);
        wasm.spawn_suspending(gate2);
        // An ordinary future on the plain executor whose multi-poll
        // progress is the ONLY thing that unblocks them — resolved out of
        // order. If a suspended poll stalled the executor, this deadlocks.
        wasm.spawn_ordinary(2, () => { resolve2(20); resolve1(10); });
        await new Promise(r => setTimeout(r, 0));
        assert.deepStrictEqual(globalThis.__jspi_spawns, [1000, 1001, 21, 11]);
    });

    it('resets the SP to the stack top when a fiber entered over live frames completes', async () => {
        const before = wasm.sp_probe();
        // Start a fiber from inside a sync export holding a live 4 KiB shadow
        // frame. The fiber suspends; call_nested's frame unwinds while it is
        // pending; on completion the SP must return to the true stack top.
        let pending;
        wasm.call_nested(() => { pending = wasm.do_sleep(); });
        assert.strictEqual(await pending, 42);
        assert.strictEqual(wasm.sp_probe(), before, 'shadow stack leaked across fiber completion');
    });
});
"#,
    );
}

#[test]
fn jspi_abort_handler_suspend_misuse() {
    run_jspi_test(
        "jspi_abort_handler_suspend_misuse",
        JSPI_LIB_RS,
        "--target nodejs --force-enable-abort-handler",
        false,
        r#"
describe('jspi misuse under --force-enable-abort-handler', () => {
    it('routes the import-boundary SuspendError through the abort handler', () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
        assert.strictEqual(abortCalled(), false);
        // The suspending import is wrapped in an `Aborting` try_table; calling
        // it without a promising frame throws SuspendError in-wasm, which the
        // wrapper routes to a clean abort via __wbindgen_rethrow_critical.
        assert.throws(() => wasm.misuse_suspend(), (e) => {
            assert.match(e.message, /Critical error/);
            return true;
        });
        assert.strictEqual(abortCalled(), true);
        assert.strictEqual(isTerminated(), true);
    });

    it('blocks all exports after termination', () => {
        assert.throws(() => wasm.simple_add(1, 2), /Module terminated/);
    });
});
"#,
    );
}

#[test]
fn jspi_abort_handler_happy_and_panic() {
    run_jspi_test(
        "jspi_abort_handler_happy_and_panic",
        JSPI_LIB_RS,
        "--target nodejs --force-enable-abort-handler",
        false,
        r#"
describe('jspi under --force-enable-abort-handler', () => {
    // Proves a try_table EH frame survives a JSPI stack switch on the happy
    // path: the catch-wrapper transform predates JSPI and wraps a call that
    // now suspends mid-flight.
    it('still suspends, resumes, and returns correct values with the handler armed', async () => {
        assert.strictEqual(wasm.setup_abort_handler(), true);
        assert.strictEqual(await wasm.do_sleep(), 42);
        const r = await Promise.all([wasm.deep_alloc(20), wasm.deep_alloc(20), wasm.deep_alloc(20)]);
        assert.deepStrictEqual(r, [1210, 1210, 1210]);
    });

    // Under the abort handler, plain catch imports take the wasm-catch-wrapper
    // path (rather than handleError); the wrapper's EH frame must behave across
    // the fiber's suspension.
    it('plain catch imports use wasm catch wrappers correctly under fibers', async () => {
        assert.strictEqual(await wasm.catch_plain_around_suspend(), 5);
        assert.strictEqual(await wasm.async_try_flaky(false), 113);
    });

    it('rejects (rather than hangs or corrupts) when a fiber panics after resume', async () => {
        await assert.rejects(() => wasm.panic_after_resume());
    });
});
"#,
    );
}

#[test]
fn jspi_panic_unwind() {
    run_jspi_test(
        "jspi_panic_unwind",
        JSPI_LIB_RS,
        "--target nodejs",
        true,
        r#"
describe('jspi with panic=unwind', () => {
    // Under panic=unwind the catch wrappers are generated with no flag, so every
    // suspending-import call is wrapped in an EH frame that is live across the
    // JSPI stack switch on the happy path.
    it('concurrent fibers suspend, resume, and return correct values', async () => {
        const r = await Promise.all([wasm.deep_alloc(20), wasm.deep_alloc(20), wasm.deep_alloc(20)]);
        assert.deepStrictEqual(r, [1210, 1210, 1210]);
    });

    it('panic after resume rejects and runs destructors across the suspend', async () => {
        globalThis.__jspi_drops = 0;
        await assert.rejects(() => wasm.panic_after_resume());
        assert.ok((globalThis.__jspi_drops | 0) >= 1, 'DropGuard must run on unwind');
    });

    it('fiber state stays clean after an unwound fiber', async () => {
        // The export wrapper's exceptional path must reset the fiber globals;
        // stale state would corrupt subsequent fibers or misroute the
        // suspending-import guard.
        await assert.rejects(() => wasm.panic_after_resume());
        assert.strictEqual(await wasm.deep_alloc(20), 1210);
        let threw = false;
        try { await wasm.misuse_suspend(); } catch (e) { threw = true; }
        assert.ok(threw, 'misuse_suspend should still throw after an unwound fiber');
    });

    it('a rejected non-catch suspending import unwinds destructors and rejects the promise', async () => {
        globalThis.__jspi_drops = 0;
        await assert.rejects(() => wasm.reject_no_catch(), /nocatch nope/);
        assert.ok((globalThis.__jspi_drops | 0) >= 1, 'DropGuard must run on foreign-exception unwind');
        assert.strictEqual(await wasm.deep_alloc(20), 1210);
    });

    it('catch still surfaces Err with unwind EH frames live, in fibers and polls', async () => {
        assert.strictEqual(await wasm.try_flaky(false), 13);
        assert.strictEqual(await wasm.async_try_flaky(false), 113);
    });

    // A spawned poll's promise is deliberately dropped, so an unwind out of
    // it is *specified* to surface as an unhandled rejection. Observe it
    // with the runner's own listeners parked, or node:test would attribute
    // the rejection to the running test and fail it.
    async function expectUnhandled(trigger) {
        const saved = process.rawListeners('unhandledRejection');
        process.removeAllListeners('unhandledRejection');
        try {
            const seen = new Promise(r => process.once('unhandledRejection', r));
            trigger();
            return await seen;
        } finally {
            for (const l of saved) process.on('unhandledRejection', l);
        }
    }

    it('a rejected non-catch suspending import mid-poll surfaces as an unhandled rejection', async () => {
        globalThis.__jspi_drops = 0;
        const err = await expectUnhandled(() => wasm.async_reject_no_catch());
        assert.match(String(err), /nocatch nope/);
        assert.ok((globalThis.__jspi_drops | 0) >= 1, 'DropGuard must run when the poll unwinds');
        // Only that task is abandoned: fibers and polls still work.
        assert.strictEqual(await wasm.async_try_flaky(true), 142);
    });

    it('a panic mid-poll after resume runs destructors and abandons only that task', async () => {
        globalThis.__jspi_drops = 0;
        await expectUnhandled(() => wasm.async_panic_after_resume());
        assert.ok((globalThis.__jspi_drops | 0) >= 1, 'DropGuard must run across the suspend on unwind');
        assert.strictEqual(await wasm.deep_alloc(20), 1210);
        assert.strictEqual(await wasm.async_fallible(true), 9);
    });
});
"#,
    );
}

#[test]
fn jspi_async_only_module() {
    // Regression test: a module whose ONLY jspi usage is `async fn` exports
    // must still keep the spawn machinery (the trampoline signal is "any
    // jspi export", sync or async — async jspi exports root the context for
    // their internal spawn, so stubbing the intrinsics would trap on the
    // first call).
    run_jspi_test(
        "jspi_async_only_module",
        r#"
    #![allow(deprecated)]
    use wasm_bindgen::prelude::*;
    use js_sys::futures::jspi_block_on_promise;

    fn sync_double(p: &js_sys::Promise) -> u32 {
        jspi_block_on_promise(p).unwrap().as_f64().unwrap() as u32 * 2
    }

    #[wasm_bindgen(jspi)]
    pub async fn async_double(p: js_sys::Promise) -> u32 {
        sync_double(&p)
    }
"#,
        "--target nodejs",
        false,
        r#"
describe('jspi async-only module', () => {
    it('an async jspi export roots the context for its own spawn', async () => {
        assert.strictEqual(await wasm.async_double(Promise.resolve(21)), 42);
    });
});
"#,
    );
}
