use wasm_bindgen_test::*;

pub mod same_function_different_locations_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_a.js")]
    extern "C" {
        pub fn foo();
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

pub mod same_function_different_locations_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_a.js")]
    extern "C" {
        pub fn foo();
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_locations() {
    same_function_different_locations_a::foo();
    same_function_different_locations_b::foo();
    same_function_different_locations_a::BAR.with(|bar| assert_eq!(*bar, 3));
    same_function_different_locations_a::BAR.with(|bar| assert_eq!(*bar, 3));
}

pub mod same_function_different_modules_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_b.js")]
    extern "C" {
        pub fn foo() -> bool;
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

pub mod same_function_different_modules_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "tests/wasm/duplicates_c.js")]
    extern "C" {
        pub fn foo() -> bool;
        #[wasm_bindgen(thread_local_v2, js_name = bar)]
        pub static BAR: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_modules() {
    assert!(same_function_different_modules_a::foo());
    assert!(!same_function_different_modules_b::foo());
    same_function_different_modules_a::BAR.with(|bar| assert_eq!(*bar, 4));
    same_function_different_modules_b::BAR.with(|bar| assert_eq!(*bar, 5));
}

// Every `inline_js` snippet is snippet 0 of its own macro invocation, so the
// snippet contents, not just its index, must keep these two apart.
pub mod same_function_different_inline_js_a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const foo = () => 6; export const X = 6;")]
    extern "C" {
        pub fn foo() -> u32;
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
    }
}

pub mod same_function_different_inline_js_b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const foo = () => 7; export const X = 7;")]
    extern "C" {
        pub fn foo() -> u32;
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
    }
}

#[wasm_bindgen_test]
fn same_function_different_inline_js() {
    assert_eq!(same_function_different_inline_js_a::foo(), 6);
    assert_eq!(same_function_different_inline_js_b::foo(), 7);
    same_function_different_inline_js_a::X.with(|x| assert_eq!(*x, 6));
    same_function_different_inline_js_b::X.with(|x| assert_eq!(*x, 7));
}
