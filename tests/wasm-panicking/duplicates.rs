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
#[ignore]
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
#[ignore]
fn same_function_different_modules() {
    assert!(same_function_different_modules_a::foo());
    assert!(!same_function_different_modules_b::foo());
    same_function_different_modules_a::BAR.with(|bar| assert_eq!(*bar, 4));
    same_function_different_modules_b::BAR.with(|bar| assert_eq!(*bar, 5));
}
