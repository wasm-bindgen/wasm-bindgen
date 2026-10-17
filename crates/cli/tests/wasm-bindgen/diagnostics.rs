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

/// Two `experimental_generic_mono` imports that agree on everything else and differ only
/// in `variadic` must get distinct shim keys, and both must bind.
///
/// Unlike `catch`, `variadic` does not perturb the signature tokens (it spreads
/// the trailing sequence argument rather than retyping it), so it is hashed into
/// the key as part of the call style (`ImportCallStyle` in `parser.rs`). One
/// import calls `console.log(xs)`, the other `console.log(...xs)`.
#[test]
fn experimental_generic_mono_variadic_does_not_collide_on_the_shim_key() {
    let out_dir = Project::new("experimental_generic_mono_variadic_shim_key")
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
                        // Differs from `one::log` only in `variadic`, which
                        // leaves the signature tokens byte-identical.
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
        .unwrap();
    let js =
        fs::read_to_string(out_dir.join("experimental_generic_mono_variadic_shim_key.js")).unwrap();

    // Both calls must be bound; if the two imports still shared a key one of
    // them would be silently dropped (or the build would fail).
    assert_contains!(&js, "console.log(v0)");
    assert_contains!(&js, "console.log(...(v0))");
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

#[test]
fn external_private_type_ref_binds_to_public_collision() {
    let out_dir = Project::new("external_private_type_ref_binds_to_public_collision")
        .file("public-widget/Cargo.toml", &dep_crate_toml("public-widget"))
        .file(
            "public-widget/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct Widget { pub public_value: u32 }

                #[wasm_bindgen]
                pub enum Mode { Public = 1 }

                pub fn make() -> Widget { Widget { public_value: 1 } }
                pub fn mode() -> Mode { Mode::Public }
            "#,
        )
        .file(
            "private-widget/Cargo.toml",
            &dep_crate_toml("private-widget"),
        )
        .file(
            "private-widget/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(private)]
                pub struct Widget { pub private_value: u32 }

                #[wasm_bindgen(private)]
                pub enum Mode { Private = 2 }

                pub fn make() -> Widget { Widget { private_value: 2 } }
                pub fn mode() -> Mode { Mode::Private }
            "#,
        )
        .dep("public-widget = { path = 'public-widget' }")
        .dep("private-widget = { path = 'private-widget' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn make_public_widget() -> public_widget::Widget {
                    public_widget::make()
                }

                #[wasm_bindgen]
                pub fn make_private_widget() -> private_widget::Widget {
                    private_widget::make()
                }

                #[wasm_bindgen]
                pub fn public_mode() -> public_widget::Mode {
                    public_widget::mode()
                }

                #[wasm_bindgen]
                pub fn private_mode() -> private_widget::Mode {
                    private_widget::mode()
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();
    let js =
        fs::read_to_string(out_dir.join("external_private_type_ref_binds_to_public_collision.js"))
            .unwrap();
    let dts = fs::read_to_string(
        out_dir.join("external_private_type_ref_binds_to_public_collision.d.ts"),
    )
    .unwrap();

    assert_contains!(&js, "return Widget2.__wrap(ret);");
    assert_contains!(&dts, "export function make_private_widget(): Widget2;");
    assert_contains!(&dts, "export function private_mode(): Mode2;");
    assert_contains!(&dts, "export function make_public_widget(): Widget;");
    assert_contains!(&dts, "export function public_mode(): Mode;");
}

/// When every colliding definition is `private`, references from outside the
/// defining crates used to hit the ambiguity error; with the defining crate
/// embedded in type descriptors they now resolve to the numbered identities.
#[test]
fn external_all_private_type_collision_resolves() {
    let out_dir = Project::new("external_all_private_type_collision_resolves")
        .file("widget-a/Cargo.toml", &dep_crate_toml("widget-a"))
        .file(
            "widget-a/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(private)]
                pub struct Widget { pub a_value: u32 }

                #[wasm_bindgen(private)]
                pub enum Mode { A = 1 }

                #[wasm_bindgen(private)]
                pub enum Shade { Light = "light" }

                #[wasm_bindgen(private)]
                pub enum ShadeOrNumber { Shade(Shade), Number(f64) }

                pub fn make() -> Widget { Widget { a_value: 1 } }
                pub fn mode() -> Mode { Mode::A }
                pub fn shade() -> ShadeOrNumber { ShadeOrNumber::Shade(Shade::Light) }
            "#,
        )
        .file("widget-b/Cargo.toml", &dep_crate_toml("widget-b"))
        .file(
            "widget-b/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(private)]
                pub struct Widget { pub b_value: u32 }

                #[wasm_bindgen(private)]
                pub enum Mode { B = 2 }

                #[wasm_bindgen(private)]
                pub enum Shade { Dark = "dark" }

                #[wasm_bindgen(private)]
                pub enum ShadeOrNumber { Shade(Shade), Number(f64) }

                pub fn make() -> Widget { Widget { b_value: 2 } }
                pub fn mode() -> Mode { Mode::B }
                pub fn shade() -> ShadeOrNumber { ShadeOrNumber::Shade(Shade::Dark) }
            "#,
        )
        .dep("widget-a = { path = 'widget-a' }")
        .dep("widget-b = { path = 'widget-b' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn make_a_widget() -> widget_a::Widget {
                    widget_a::make()
                }

                #[wasm_bindgen]
                pub fn make_b_widget() -> widget_b::Widget {
                    widget_b::make()
                }

                #[wasm_bindgen]
                pub fn a_mode() -> widget_a::Mode {
                    widget_a::mode()
                }

                #[wasm_bindgen]
                pub fn b_mode() -> widget_b::Mode {
                    widget_b::mode()
                }

                #[wasm_bindgen]
                pub fn a_shade() -> widget_a::ShadeOrNumber {
                    widget_a::shade()
                }

                #[wasm_bindgen]
                pub fn b_shade() -> widget_b::ShadeOrNumber {
                    widget_b::shade()
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();
    let dts = fs::read_to_string(out_dir.join("external_all_private_type_collision_resolves.d.ts"))
        .unwrap();

    // Identity assignment orders definitions by unique crate identifier, so
    // `widget-a` keeps the canonical names and `widget-b` gets numbered ones.
    assert_contains!(&dts, "export function make_a_widget(): Widget;");
    assert_contains!(&dts, "export function make_b_widget(): Widget2;");
    assert_contains!(&dts, "export function a_mode(): Mode;");
    assert_contains!(&dts, "export function b_mode(): Mode2;");
    assert_contains!(&dts, "export function a_shade(): ShadeOrNumber;");
    assert_contains!(&dts, "export function b_shade(): ShadeOrNumber2;");
    assert_contains!(&dts, "type ShadeOrNumber = Shade | number;");
    assert_contains!(&dts, "type ShadeOrNumber2 = Shade2 | number;");
    assert_contains!(&dts, "type Shade = \"light\";");
    assert_contains!(&dts, "type Shade2 = \"dark\";");
}

/// Two public definitions of one name in different modules of one crate are
/// the same collision as across crates, including byte-identical copies.
fn duplicate_public_item_in_one_crate_errors(project: &str, item: &str, name: &str) {
    let err = Project::new(project)
        .file(
            "src/lib.rs",
            &format!(
                r#"
                    macro_rules! items {{
                        ($m:ident) => {{
                            pub mod $m {{
                                use wasm_bindgen::prelude::*;
                                #[wasm_bindgen]
                                {item}
                                #[wasm_bindgen]
                                pub fn $m(value: {name}) -> {name} {{ value }}
                            }}
                        }};
                    }}
                    items!(a);
                    items!(b);
                "#
            ),
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert!(
        err.contains(&format!("the name `{name}` is exported multiple times")),
        "unexpected error for `{name}`:\n{err}"
    );
}

/// Structs, numeric enums, string enums and dynamic unions share one
/// TypeScript scope, so same-named items of different kinds are numbered
/// together.
#[test]
fn same_name_across_item_kinds_resolves() {
    let out_dir = Project::new("same_name_across_item_kinds_resolves")
        .file(
            "src/lib.rs",
            r#"
                macro_rules! levels {
                    ($m:ident) => {
                        pub mod $m {
                            use wasm_bindgen::prelude::*;
                            #[wasm_bindgen(private)]
                            pub enum Level { Low = 1 }
                            #[wasm_bindgen]
                            pub fn $m(level: Level) -> Level { level }
                        }
                    };
                }
                levels!(a);
                levels!(b);

                pub mod c {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen]
                    pub enum Level { Low = "low" }
                    #[wasm_bindgen]
                    pub fn c(level: Level) -> Level { level }
                }
            "#,
        )
        .wasm_bindgen("--target web")
        .unwrap();
    let dts =
        fs::read_to_string(out_dir.join("same_name_across_item_kinds_resolves.d.ts")).unwrap();

    assert_contains!(&dts, "export function a(level: Level2): Level2;");
    assert_contains!(&dts, "export function b(level: Level3): Level3;");
    assert_contains!(&dts, "export function c(level: Level): Level;");
    assert_contains!(&dts, "type Level = \"low\";");
    assert_contains!(&dts, "declare enum Level2 {");
    assert_contains!(&dts, "declare enum Level3 {");
}

#[test]
fn duplicate_public_string_enum_in_one_crate_errors() {
    duplicate_public_item_in_one_crate_errors(
        "duplicate_public_string_enum_in_one_crate_errors",
        r#"pub enum Mode { Fast = "fast", Slow = "slow" }"#,
        "Mode",
    );
}

#[test]
fn duplicate_public_numeric_enum_in_one_crate_errors() {
    duplicate_public_item_in_one_crate_errors(
        "duplicate_public_numeric_enum_in_one_crate_errors",
        "#[derive(Clone, Copy)] pub enum Level { Low = 1, High = 2 }",
        "Level",
    );
}

#[test]
fn duplicate_public_dynamic_union_in_one_crate_errors() {
    duplicate_public_item_in_one_crate_errors(
        "duplicate_public_dynamic_union_in_one_crate_errors",
        "pub enum TextOrNumber { Text(String), Number(f64) }",
        "TextOrNumber",
    );
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

/// A `slice` / `vec` mode written on an argument is validated against the
/// argument's resolved type, which only the CLI sees through aliases and
/// generics. Inherited, the same modes are a no-op wherever they have nothing
/// to do, except on a `&mut` slice, where they can't be honoured.
#[test]
fn seq_modes_are_validated_when_explicit() {
    let bind = |name: &str, block: &str, run: &str| {
        Project::new(name)
            .file(
                "src/lib.rs",
                &format!(
                    r#"
                        use wasm_bindgen::prelude::*;
                        type Bytes = Vec<u8>;
                        type Buf = &'static mut [u8];
                        {block}
                        #[wasm_bindgen]
                        pub fn run() {{ {run} }}
                    "#
                ),
            )
            .wasm_bindgen("")
    };
    for (name, decl, value, expected) in [
        (
            "seq_mode_vec_on_borrowed",
            "fn f(#[wasm_bindgen(vec = copy)] xs: &[u8]);",
            "&[1]",
            "`vec = copy` can't apply to this argument: `vec` applies to owned vectors; for a \
             borrowed `&[T]` use `slice = view | copy | array`",
        ),
        (
            "seq_mode_slice_on_owned",
            "fn f(#[wasm_bindgen(slice = copy)] xs: Vec<u8>);",
            "vec![1]",
            "`slice = copy` can't apply to this argument: `slice` applies to borrowed slices; \
             for an owned `Vec<T>` use `vec = copy | array`",
        ),
        (
            "seq_mode_slice_on_aliased_owned",
            "fn f(#[wasm_bindgen(slice = array)] xs: Bytes);",
            "vec![1]",
            "`slice = array` can't apply to this argument: `slice` applies to borrowed slices",
        ),
        (
            "seq_mode_inherited_copy_on_aliased_mut_slice",
            "#[wasm_bindgen(slice = copy)] fn f(xs: Buf);",
            "Box::leak(vec![1u8].into_boxed_slice())",
            "`slice = copy` can't apply to this argument: JS would receive a copy, whose writes \
             never reach the `&mut [T]`; only `slice = view` applies",
        ),
        (
            "seq_mode_view_on_untyped_elements",
            "fn f(#[wasm_bindgen(slice = view)] xs: &[String]);",
            r#"&["a".to_string()]"#,
            "`slice = view` can't apply to this argument: it has no typed-array form, so it \
             always reaches JS as a fresh `Array`",
        ),
        (
            "seq_mode_slice_on_scalar",
            "fn f(#[wasm_bindgen(slice = copy)] x: u32);",
            "1",
            "`slice = copy` can't apply to this argument: `slice` applies to borrowed slice \
             arguments (`&[T]`)",
        ),
    ] {
        let block = format!(r#"#[wasm_bindgen] extern "C" {{ {decl} }}"#);
        let err = format!(
            "{:#}",
            bind(name, &block, &format!("f({value});")).unwrap_err()
        );
        assert!(
            err.contains("failed to generate a binding for argument 1 of `__wbg_f_"),
            "{err}"
        );
        assert!(err.contains(expected), "{err}");
    }
    // `array` fits a sequence that arrives as a fresh `Array` anyway.
    bind(
        "seq_mode_no_op_elsewhere",
        r#"
            #[wasm_bindgen(slice = copy, vec = array)]
            extern "C" {
                fn f(xs: &[u8], ys: Vec<u8>, aliased: Bytes, strings: &[String]);
                fn g(
                    #[wasm_bindgen(slice = array)] strings: &[String],
                    #[wasm_bindgen(vec = array)] owned: Vec<String>,
                );
            }
        "#,
        r#"
            f(&[1], vec![2], vec![3], &["a".to_string()]);
            g(&["a".to_string()], vec!["b".to_string()]);
        "#,
    )
    .unwrap();
}
