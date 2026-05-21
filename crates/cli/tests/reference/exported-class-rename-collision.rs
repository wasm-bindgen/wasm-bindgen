use wasm_bindgen::prelude::*;

// Two structs sharing the same Rust ident in different modules, disambiguated
// purely via `js_name` (no `js_namespace`). This is the case `qualified_name`
// keying of `exported_classes` (#5154) was introduced to support, and is the
// minimal one that does not also depend on namespace handling.
//
// Pre-#5154, `exported_classes` was keyed by `rust_name`, so both `Point`
// structs collapsed onto the same entry and codegen hard-errored with
// "found duplicate constructor for class FooPoint". Under `qualified_name`
// keying — which without a namespace is just the `js_name` — the two entries
// are distinct and both classes are emitted independently.
//
// Pinning this case prevents a future refactor from regressing the keying
// back to a `rust_name`-equivalent scheme. The namespace-based variant is
// covered by `js-namespace-export-same-name`; this fixture covers the
// no-namespace half.

mod foo {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = "FooPoint")]
    pub struct Point {
        x: i32,
    }

    #[wasm_bindgen(js_class = "FooPoint")]
    impl Point {
        #[wasm_bindgen(constructor)]
        pub fn new(x: i32) -> Point {
            Point { x }
        }
        #[wasm_bindgen(getter)]
        pub fn x(&self) -> i32 {
            self.x
        }
    }
}

mod bar {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = "BarPoint")]
    pub struct Point {
        y: i32,
    }

    #[wasm_bindgen(js_class = "BarPoint")]
    impl Point {
        #[wasm_bindgen(constructor)]
        pub fn new(y: i32) -> Point {
            Point { y }
        }
        #[wasm_bindgen(getter)]
        pub fn y(&self) -> i32 {
            self.y
        }
    }
}

pub use bar::Point as _Bar;
pub use foo::Point as _Foo;
