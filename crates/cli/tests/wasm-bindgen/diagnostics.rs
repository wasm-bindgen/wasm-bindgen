//! Tests for cli-support diagnostic messages emitted when a user-supplied
//! name (impl `js_class`, struct `extends` parent, ...) fails to resolve.
//!
//! These exercises live alongside the rest of the CLI test suite rather than
//! in the macro UI-test directory because they assert behaviour of
//! `wasm-bindgen` post-macro-expansion: the user code compiles, the wasm is
//! emitted, and the failure surfaces when `wasm-bindgen` walks the encoded
//! `Aux*` data and tries to wire up class references.

use crate::Project;
use std::fs;

macro_rules! assert_contains {
    ($haystack:expr, $needle:literal) => {
        let haystack = $haystack;
        assert!(
            haystack.contains($needle),
            "Expected\n{haystack:?}\nto contain\n{:?}",
            $needle
        );
    };
}

/// When a struct declares `js_namespace = ns` but the impl block omits it,
/// the impl macro emits methods with no namespace prefix in the wasm shim
/// symbol and `Export.js_namespace = None`. The resulting class identity
/// (`Foo`) doesn't match the struct's registered `qualified_name`
/// (`ns__Foo`), so cli-support emits a targeted hint asking the user to
/// repeat `js_namespace` on the impl. The hint must reference the exact
/// namespace the struct uses so the fix is mechanical.
#[test]
fn missing_js_namespace_on_impl_suggests_struct_namespace() {
    let err = Project::new("missing_js_namespace_on_impl_suggests_struct_namespace")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(js_name = "Foo", js_namespace = ns)]
                pub struct FooImpl;

                // Deliberately missing `js_namespace = ns` on the impl. The
                // impl macro can't see the struct's attrs cross-invocation
                // so the namespace must be repeated here for the emitted
                // wasm symbol to round-trip through cli-support correctly.
                #[wasm_bindgen(js_class = "Foo")]
                impl FooImpl {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> FooImpl { FooImpl }
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "class `Foo` referenced by an impl block");
    assert_contains!(
        &err,
        "a struct with the same `js_name` exists in a different namespace"
    );
    assert_contains!(&err, "js_namespace = ns");
    assert_contains!(&err, "ns__Foo");
}

/// Same shape with a nested namespace (`["a", "b"]`). The hint must list
/// every segment so the user can copy-paste it onto the impl block.
#[test]
fn missing_js_namespace_on_impl_nested_namespace() {
    let err = Project::new("missing_js_namespace_on_impl_nested_namespace")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(js_name = "Foo", js_namespace = ["a", "b"])]
                pub struct FooImpl;

                #[wasm_bindgen(js_class = "Foo")]
                impl FooImpl {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> FooImpl { FooImpl }
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "js_namespace = a, b");
}

/// A typo in `js_class` produces a "did you mean ...?" fuzzy hint sourced
/// from the registered struct names. Ranking is by edit distance so the
/// closest candidate appears first.
#[test]
fn typo_in_js_class_suggests_nearest_struct() {
    let err = Project::new("typo_in_js_class_suggests_nearest_struct")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct Counter { value: i32 }

                #[wasm_bindgen(js_class = "Countr")]
                impl Counter {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> Counter { Counter { value: 0 } }
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "class `Countr` referenced by an impl block");
    assert_contains!(&err, "did you mean `Counter`?");
}

// Note: the `extends = ParentPath` failure-with-suggestion path is wired
// in `write_class` (using the same suggestion helper as
// `validate_impl_class_references`) but it's awkward to trigger from an
// integration test without also tripping a Rust-level compilation error
// (a non-`#[wasm_bindgen]` parent type fails the `Parent<T>` trait bound
// the macro injects). The helper itself has unit coverage in
// `cli-support/src/suggest.rs::tests` and the wider hint-formatting
// behaviour is exercised by `typo_in_js_class_suggests_nearest_struct`
// above, so we deliberately don't duplicate the integration test here.

/// Two `experimental_generic_mono` imports that agree on everything else and differ only
/// in `js_namespace` must get distinct shim keys, and both must bind.
///
/// The key is `__wbg_<wasm.name>_<ShortHash(ns, sig tokens, module, cfg attrs)>`
/// (see `parser.rs`). `js_namespace` used to contribute to *neither* the prefix
/// nor the hash, so the pair below collided and `wit/mod.rs` had to refuse the
/// whole build. `js_namespace` is now folded into the hashed namespace element,
/// so the collision class is gone: the two imports below live in separate modules
/// so their `sig` token streams are byte-identical (`fn log<T>(x: T)`) and every
/// other hashed input agrees, which is exactly the case that used to fail.
#[test]
fn experimental_generic_mono_js_namespace_does_not_collide_on_the_shim_key() {
    let out_dir = Project::new("experimental_generic_mono_js_namespace_shim_key")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                // Separate modules so both can declare an identically-spelled
                // `fn log<T>(x: T)`; the shim key hashes the signature tokens,
                // so everything but `js_namespace` agrees here.
                pub mod one {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = a, experimental_generic_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                pub mod two {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = b, experimental_generic_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                #[wasm_bindgen]
                pub fn run() {
                    one::log(1u32);
                    two::log(2u32);
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();
    let js = fs::read_to_string(out_dir.join("experimental_generic_mono_js_namespace_shim_key.js"))
        .unwrap();

    // Both namespaces must actually be bound; if the two imports still shared a
    // key one of them would be silently dropped (or the build would fail).
    assert_contains!(&js, "a.log(");
    assert_contains!(&js, "b.log(");
}

/// The same `experimental_generic_mono` import declared twice must deduplicate, not
/// abort the build.
///
/// Two byte-identical declarations in two modules of one crate necessarily
/// hash to one shim key -- every input to the hash agrees. That is benign:
/// both resolve to the same JS value and want the same binding, which is why
/// the ordinary (non-generic) import path deduplicates the same case silently
/// via `function_imports`. `wit/mod.rs` used to treat *any* key clash as a
/// collision and refuse, which made a legitimate pattern (per-module
/// `extern "C"` blocks re-declaring a common import) unbuildable, and reported
/// it with a message stating the two "differ only in an attribute" when they
/// are identical.
///
/// Note `ShortHash` mixes in `CARGO_PKG_NAME`/`CARGO_PKG_VERSION`, so the
/// cross-crate form of this never collided; it has to be one crate to bite.
#[test]
fn experimental_generic_mono_identical_imports_deduplicate() {
    let out_dir = Project::new("experimental_generic_mono_identical_imports_deduplicate")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                // Byte-identical declarations. Same fn name, same signature
                // tokens, same (absent) namespace and module, so the shim key
                // is necessarily the same for both.
                pub mod one {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = console, experimental_generic_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                pub mod two {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = console, experimental_generic_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                #[wasm_bindgen]
                pub fn run() {
                    // Distinct monomorphisations through each declaration, so
                    // both groups have to resolve through the shared key.
                    one::log(1u32);
                    two::log("two");
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();
    let js = fs::read_to_string(
        out_dir.join("experimental_generic_mono_identical_imports_deduplicate.js"),
    )
    .unwrap();

    // Both monomorphisations bind, through the one shared entry.
    assert_contains!(&js, "console.log(");
}

/// A *genuine* shim-key collision must still be reported, naming both imports.
///
/// `variadic` is one of the attributes that does not contribute to the shim
/// key and, unlike `catch`, does not perturb the signature tokens either (it
/// spreads the trailing sequence argument rather than retyping it). So these
/// two agree on every hashed input yet want materially different bindings:
/// one calls `console.log(xs)`, the other `console.log(...xs)`. Binding only
/// one would silently mis-bind every monomorphisation of the other, so this
/// has to stay an error -- the dedup above must not swallow it.
#[test]
fn experimental_generic_mono_genuine_collision_is_reported() {
    let err = Project::new("experimental_generic_mono_genuine_collision_is_reported")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                pub mod one {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = console, experimental_generic_mono)]
                        pub fn log<T>(xs: Vec<T>);
                    }
                }

                pub mod two {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        // Differs from `one::log` only in `variadic`, which is
                        // not hashed into the shim key and leaves the signature
                        // tokens byte-identical.
                        #[wasm_bindgen(js_namespace = console, experimental_generic_mono, variadic)]
                        pub fn log<T>(xs: Vec<T>);
                    }
                }

                #[wasm_bindgen]
                pub fn run() {
                    one::log(vec![1u32]);
                    two::log(vec![2u32]);
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "collided on the shim key");
    // The shim key is a hash, so the message has to name the imports in terms
    // the user recognises.
    assert_contains!(&err, "console.log");
}

/// Same-named exports from different crates no longer collide at link time
/// (their wasm shims carry a per-crate hash), so a genuine JS-level name
/// collision is reported by cli-support instead of surfacing as a cryptic
/// wasm-ld duplicate-symbol failure. Structs/enums are checked up front and
/// the hint offers `#[wasm_bindgen(private)]` as an out.
#[test]
fn duplicate_public_class_across_crates_errors() {
    let err = Project::new("duplicate_public_class_across_crates_errors")
        .file("dupe-a/Cargo.toml", &dep_crate_toml("dupe-a"))
        .file(
            "dupe-a/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub struct Widget { pub id: u32 }
                pub fn touch() -> u32 { Widget { id: 1 }.id }
            "#,
        )
        .file("dupe-b/Cargo.toml", &dep_crate_toml("dupe-b"))
        .file(
            "dupe-b/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub struct Widget { pub id: u32 }
                pub fn touch() -> u32 { Widget { id: 2 }.id }
            "#,
        )
        .dep("dupe-a = { path = 'dupe-a' }")
        .dep("dupe-b = { path = 'dupe-b' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn touch_both() -> u32 { dupe_a::touch() + dupe_b::touch() }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "the name `Widget` is exported multiple times");
    assert_contains!(&err, "#[wasm_bindgen(private)]");
}

/// Functions have no `private` form, so the collision is caught when the
/// second export's canonical name is restored.
#[test]
fn duplicate_public_function_across_crates_errors() {
    let err = Project::new("duplicate_public_function_across_crates_errors")
        .file("fn-dupe-a/Cargo.toml", &dep_crate_toml("fn-dupe-a"))
        .file(
            "fn-dupe-a/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn overlap() -> u32 { 1 }
            "#,
        )
        .file("fn-dupe-b/Cargo.toml", &dep_crate_toml("fn-dupe-b"))
        .file(
            "fn-dupe-b/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn overlap() -> u32 { 2 }
            "#,
        )
        .dep("fn-dupe-a = { path = 'fn-dupe-a' }")
        .dep("fn-dupe-b = { path = 'fn-dupe-b' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn touch_both() -> u32 { fn_dupe_a::overlap() + fn_dupe_b::overlap() }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "the name `overlap` is exported by multiple crates");
}

fn dep_crate_toml(name: &str) -> String {
    format!(
        r#"
            [package]
            name = "{name}"
            version = "0.0.0"
            edition = "2021"

            [dependencies]
            wasm-bindgen = {{ path = '{repo}' }}
        "#,
        repo = crate::REPO_ROOT.display(),
    )
}
