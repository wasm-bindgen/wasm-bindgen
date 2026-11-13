use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn takes_values(v: Vec<JsValue>);
    fn takes_optional_values(v: Option<Vec<JsValue>>);
}

#[wasm_bindgen]
pub fn give() {
    takes_values(vec![JsValue::NULL]);
    takes_optional_values(None);
}
