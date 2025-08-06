use wasm_bindgen::prelude::*;

// Imports with keywords

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type A;

    #[wasm_bindgen(static_method_of = A, js_name = "new")]
    pub fn static_new() -> A;
    #[wasm_bindgen(js_namespace = ["B"], js_name = "new")]
    pub fn namespace_new();

    #[wasm_bindgen(method, js_name = "let")]
    pub fn keyword_let(ptr: &A);

    // await is not a reserved keyword in JS
    pub fn r#await();

    // true & false are reserved keywords in JS, but we allow them anyway
    #[wasm_bindgen(thread_local_v2, js_name = "true")]
    static TRUE: JsValue;
}

// https://github.com/wasm-bindgen/wasm-bindgen/issues/4317
#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "menu"])]
extern "C" {
    #[wasm_bindgen]
    pub type Menu;

    #[wasm_bindgen(static_method_of = Menu)]
    pub fn new() -> Menu;
}

// This function ensures the imported stuff isn't optimized out
#[wasm_bindgen]
pub fn exported() {
    let a = A::static_new();
    let _ = a.keyword_let();
    let _ = namespace_new();
    let _ = r#await();
    std::hint::black_box(&TRUE);

    let _ = Menu::new();
}

// Exports with keywords that we allow and are renamed automatically.

#[wasm_bindgen]
pub fn function() {}

#[wasm_bindgen(js_name = "var")]
pub fn sane_name() {}

#[wasm_bindgen]
pub fn weird_arguments(new: u32, var: u32, r#switch: u32, default: u32, arguments: u32) {}
