// Data for `fixtures.rs`'s batched workspace. Split into its own file (via
// `include!`) purely to keep `fixtures.rs`'s mechanism separate from the
// (long, mostly-generated-content) fixture bodies themselves.
//
// Keep this list in sync with the `#[test]` functions in `main.rs`,
// `diagnostics.rs`, and `npm.rs` that call `fixture(name)`.

static FIXTURES: &[Fixture] = &[
    // --- diagnostics.rs ---
    fixture_def(
        "missing_js_namespace_on_impl_suggests_struct_namespace",
        &[(
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
        )],
    ),
    fixture_def(
        "missing_js_namespace_on_impl_nested_namespace",
        &[(
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
        )],
    ),
    fixture_def(
        "typo_in_js_class_suggests_nearest_struct",
        &[(
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
        )],
    ),
    // --- npm.rs ---
    fixture_def(
        "no_modules_rejects_npm",
        &[
            (
                "src/lib.rs",
                r#"
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen(module = "foo")]
                    extern {
                        fn foo();
                    }

                    #[wasm_bindgen(start)]
                    fn main() {
                        foo();
                    }
                "#,
            ),
            ("package.json", ""),
        ],
    ),
    fixture_def(
        "more_package_json_fields_ignored",
        &[
            (
                "src/lib.rs",
                r#"
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen(module = "foo")]
                    extern {
                        fn foo();
                    }

                    #[wasm_bindgen(start)]
                    fn main() {
                        foo();
                    }
                "#,
            ),
            (
                "package.json",
                r#"
                    {
                        "name": "foo",
                        "dependencies": {}
                    }
                "#,
            ),
        ],
    ),
    Fixture {
        name: "npm_conflict_rejected",
        extra_deps: &["bar = { path = 'bar' }"],
        files: &[
            (
                "src/lib.rs",
                r#"
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen(module = "bar")]
                    extern {
                        fn foo();
                    }

                    #[wasm_bindgen(start)]
                    fn main() {
                        foo();
                        bar::foo();
                    }
                "#,
            ),
            (
                "package.json",
                r#"
                    {
                        "dependencies": {"bar": "0.0.0"}
                    }
                "#,
            ),
            (
                "bar/Cargo.toml",
                r#"
                [package]
                name = "bar"
                authors = []
                version = "1.0.0"
                edition = '2021'

                [dependencies]
                wasm-bindgen = { path = '{root}' }
            "#,
            ),
            (
                "bar/src/lib.rs",
                r#"
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen(module = "bar")]
                    extern {
                        pub fn foo();
                    }
                "#,
            ),
            (
                "bar/package.json",
                r#"
                    {
                        "dependencies": {"bar": "1.0.0"}
                    }
                "#,
            ),
        ],
    },
    // --- main.rs ---
    fixture_def(
        "works_on_empty_project",
        &[(
            "src/lib.rs",
            r#"
            "#,
        )],
    ),
    fixture_def(
        "namespace_global_and_noglobal_works",
        &[(
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
        )],
    ),
    fixture_def(
        "one_export_works",
        &[(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn foo() {}
            "#,
        )],
    ),
    fixture_def(
        "default_module_path_target_web",
        &[(
            "src/lib.rs",
            r#"
            "#,
        )],
    ),
    fixture_def(
        "default_module_path_target_no_modules",
        &[(
            "src/lib.rs",
            r#"
            "#,
        )],
    ),
    fixture_def(
        "omit_default_module_path_target_web",
        &[(
            "src/lib.rs",
            r#"
            "#,
        )],
    ),
    fixture_def(
        "omit_default_module_path_target_no_modules",
        &[(
            "src/lib.rs",
            r#"
            "#,
        )],
    ),
    fixture_def(
        "function_table_preserved",
        &[(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn bar() {
                    Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
                }
            "#,
        )],
    ),
    fixture_def(
        "function_table_preserved_for_stack_closures",
        &[(
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
        )],
    ),
    fixture_def(
        "constructor_cannot_return_option_struct",
        &[(
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
        )],
    ),
    fixture_def(
        "multiple_start_functions",
        &[(
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
        )],
    ),
    fixture_def(
        "private_start_function",
        &[(
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
        )],
    ),
    fixture_def(
        "private_namespaced_classes_export_actual_ts_identifier",
        &[(
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
        )],
    ),
    fixture_def(
        "bin_crate_works",
        &[
            (
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
            ),
            (
                "Cargo.toml",
                r#"
                    [package]
                    name = "bin_crate_works"
                    authors = []
                    version = "1.0.0"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = { path = '{root}' }
                "#,
            ),
        ],
    ),
    fixture_def(
        "bin_crate_works_without_name_section",
        &[
            (
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
            ),
            (
                "Cargo.toml",
                r#"
                    [package]
                    name = "bin_crate_works_without_name_section"
                    authors = []
                    version = "1.0.0"
                    edition = '2021'

                    [dependencies]
                    wasm-bindgen = { path = '{root}' }
                "#,
            ),
        ],
    ),
    fixture_def(
        "reinit_panic_abort",
        &[(
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
        )],
    ),
    fixture_def(
        "emscripten_namespaced_exports_valid_ts",
        &[(
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
        )],
    ),
    fixture_def(
        "emscripten_exports_hoisted_to_library_symbols",
        &[(
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
        )],
    ),
    fixture_def(
        "emscripten_user_imports_are_prefixed",
        &[(
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
        )],
    ),
    Fixture {
        name: "generated_paths_survive_shadowed_core_alloc_std",
        extra_deps: &["wasm-bindgen-futures = { path = '{root}/crates/futures' }"],
        files: &[(
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
        )],
    },
    fixture_def(
        "slice_to_array_is_a_no_op_on_exported_mut_slice_args",
        &[(
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
        )],
    ),
];
