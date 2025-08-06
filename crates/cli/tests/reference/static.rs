// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }

use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    static PLAIN: JsValue;
    #[wasm_bindgen(thread_local_v2)]
    static OPTIONAL: Option<Number>;
    #[wasm_bindgen(thread_local_v2, js_namespace = test)]
    static NAMESPACE_PLAIN: JsValue;
    #[wasm_bindgen(thread_local_v2, js_namespace = test)]
    static NAMESPACE_OPTIONAL: Option<Number>;
    #[wasm_bindgen(thread_local_v2, js_namespace = ["test1", "test2"])]
    static NESTED_NAMESPACE_PLAIN: JsValue;
    #[wasm_bindgen(thread_local_v2, js_namespace = ["test1", "test2"])]
    static NESTED_NAMESPACE_OPTIONAL: Option<Number>;
}

#[wasm_bindgen]
pub fn exported() {
    let _ = PLAIN.with(JsValue::clone);
    let _ = OPTIONAL.with(Option::clone);
    let _ = NAMESPACE_PLAIN.with(JsValue::clone);
    let _ = NAMESPACE_OPTIONAL.with(Option::clone);
    let _ = NESTED_NAMESPACE_PLAIN.with(JsValue::clone);
    let _ = NESTED_NAMESPACE_OPTIONAL.with(Option::clone);
}
