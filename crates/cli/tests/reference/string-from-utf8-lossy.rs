use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(bytes: &[u8]) -> JsValue {
    JsValue::from_utf8_lossy(bytes)
}
