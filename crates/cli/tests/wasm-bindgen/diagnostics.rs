//! Tests for cli-support diagnostic messages emitted when a user-supplied
//! name (impl `js_class`, struct `extends` parent, ...) fails to resolve.
//!
//! These exercises live alongside the rest of the CLI test suite rather than
//! in the macro UI-test directory because they assert behaviour of
//! `wasm-bindgen` post-macro-expansion: the user code compiles, the wasm is
//! emitted, and the failure surfaces when `wasm-bindgen` walks the encoded
//! `Aux*` data and tries to wire up class references.

use crate::{Project, REPO_ROOT};
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

/// Two `generic_per_mono` imports that agree on everything else and differ only
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
fn generic_per_mono_js_namespace_does_not_collide_on_the_shim_key() {
    let out_dir = Project::new("generic_per_mono_js_namespace_shim_key")
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
                        #[wasm_bindgen(js_namespace = a, generic_per_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                pub mod two {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = b, generic_per_mono)]
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
    let js = fs::read_to_string(out_dir.join("generic_per_mono_js_namespace_shim_key.js")).unwrap();

    // Both namespaces must actually be bound; if the two imports still shared a
    // key one of them would be silently dropped (or the build would fail).
    assert_contains!(&js, "a.log(");
    assert_contains!(&js, "b.log(");
}

/// The same `generic_per_mono` import declared twice must deduplicate, not
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
fn generic_per_mono_identical_imports_deduplicate() {
    let out_dir = Project::new("generic_per_mono_identical_imports_deduplicate")
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
                        #[wasm_bindgen(js_namespace = console, generic_per_mono)]
                        pub fn log<T>(x: T);
                    }
                }

                pub mod two {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = console, generic_per_mono)]
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
    let js = fs::read_to_string(out_dir.join("generic_per_mono_identical_imports_deduplicate.js"))
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
fn generic_per_mono_genuine_collision_is_reported() {
    let err = Project::new("generic_per_mono_genuine_collision_is_reported")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                pub mod one {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    extern "C" {
                        #[wasm_bindgen(js_namespace = console, generic_per_mono)]
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
                        #[wasm_bindgen(js_namespace = console, generic_per_mono, variadic)]
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

#[test]
fn generic_per_mono_class_requires_type_opt_in() {
    let err = Project::new("generic_per_mono_class_requires_type_opt_in")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(holder: &Holder<JsValue>) -> JsValue {
                    holder.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsPerMonoGenericImport");
    assert_contains!(&err, "Holder<T>");
    assert_contains!(&err, "must use the type-erasure path");
}

#[test]
fn generic_per_mono_type_rejects_erased_generic_method() {
    let err = Project::new("generic_per_mono_type_rejects_erased_generic_method")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono)]
                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(holder: &Holder<JsValue>) -> JsValue {
                    holder.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsErasedGenericImport");
    assert_contains!(&err, "Holder<T>");
    assert_contains!(&err, "must use `generic_per_mono`");
}

#[test]
fn generic_per_mono_policy_uses_rust_type_identity() {
    let err = Project::new("generic_per_mono_policy_uses_rust_type_identity")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono, js_name = Shared)]
                    pub type Marked<T>;

                    #[wasm_bindgen(js_name = Shared)]
                    pub type Unmarked<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, js_class = Shared, generic_per_mono)]
                    pub fn get<T>(this: &Unmarked<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Unmarked<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsPerMonoGenericImport");
    assert_contains!(&err, "Unmarked<T>");
}

#[test]
fn generic_per_mono_disabled_policy_does_not_affect_active_declaration() {
    let err = Project::new("disabled_type_policy_does_not_affect_active_declaration")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[cfg(any())]
                    #[wasm_bindgen(generic_per_mono)]
                    pub type Holder<T>;

                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Holder<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsPerMonoGenericImport");
    assert_contains!(&err, "Holder<T>");
}

#[test]
fn generic_per_mono_inherited_active_class_cfg_rejects_erased_method() {
    let err = Project::new("generic_per_mono_inherited_active_class_cfg_rejects_erased_method")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[cfg(all())]
                    #[wasm_bindgen(generic_per_mono)]
                    pub type Holder<T>;

                    #[wasm_bindgen(method)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Holder<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "marked `generic_per_mono` must also use");
}

#[test]
fn generic_per_mono_class_default_still_requires_type_opt_in() {
    let err = Project::new("generic_class_default_still_requires_type_opt_in")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn get<T>(this: &Holder) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Holder) -> JsValue {
                    value.get()
                }
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsPerMonoGenericImport");
    assert_contains!(&err, "Holder");
}

#[test]
fn generic_per_mono_uninstantiated_method_still_requires_type_opt_in() {
    let err = Project::new("generic_per_mono_uninstantiated_method_still_requires_type_opt_in")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run() {}
            "#,
        )
        .compile_error();

    assert_contains!(&err, "SupportsPerMonoGenericImport");
    assert_contains!(&err, "Holder<T>");
}

#[test]
fn generic_per_mono_unrelated_or_disabled_markers_do_not_reject_erased_imports() {
    Project::new("unrelated_or_disabled_markers_do_not_reject_erased_imports")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono, js_name = Shared)]
                    pub type Marked<T>;

                    #[wasm_bindgen(js_name = Shared)]
                    pub type Unmarked<T>;

                    #[wasm_bindgen(generic_per_mono)]
                    pub type DisabledHolder<T>;

                    pub type DisabledUnmarked<T>;

                    #[cfg(all())]
                    #[wasm_bindgen(generic_per_mono)]
                    pub type GatedMarked<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, js_class = Shared)]
                    pub fn get<T>(this: &Unmarked<T>) -> T;

                    #[cfg(any())]
                    #[wasm_bindgen(method)]
                    pub fn disabled<T>(this: &DisabledHolder<T>) -> T;

                    #[cfg(any())]
                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn disabled_per_mono<T>(this: &DisabledUnmarked<T>) -> T;

                    #[wasm_bindgen(method, generic_per_mono)]
                    pub fn gated_per_mono<T>(this: &GatedMarked<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Unmarked<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn generic_per_mono_policy_is_independent_of_js_class_override() {
    Project::new("generic_per_mono_policy_is_independent_of_js_class_override")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono, js_name = Actual)]
                    pub type Renamed<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method, js_class = Different, generic_per_mono)]
                    pub fn get<T>(this: &Renamed<T>) -> T;
                }

                #[wasm_bindgen]
                pub fn run(value: &Renamed<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn generic_per_mono_cross_crate_policy_is_checked_during_compile() {
    let mut project = Project::new("generic_per_mono_cross_crate_policy_is_checked_during_compile");
    project
        .dep("upstream = { path = 'upstream' }")
        .file(
            "upstream/Cargo.toml",
            &format!(
                r#"
                    [package]
                    name = "upstream"
                    authors = []
                    version = "1.0.0"
                    edition = "2021"

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}
                "#,
                REPO_ROOT.display(),
            ),
        )
        .file(
            "upstream/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(generic_per_mono)]
                    pub type Holder<T>;
                }

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(method)]
                    pub fn get<T>(this: &Holder<T>) -> T;
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn run(value: &upstream::Holder<JsValue>) -> JsValue {
                    value.get()
                }
            "#,
        );

    let err = project.compile_error();
    assert_contains!(&err, "SupportsErasedGenericImport");
    assert_contains!(&err, "Holder<T>");
}
