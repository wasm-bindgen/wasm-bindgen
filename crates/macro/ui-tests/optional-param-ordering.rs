use wasm_bindgen::prelude::*;

// Test: non-optional param after optional param should be rejected
#[wasm_bindgen]
pub fn bad_optional_ordering(
    #[wasm_bindgen(unchecked_optional_param_type = "string")] name: JsValue,
    #[wasm_bindgen(unchecked_param_type = "number")] age: JsValue,
) -> JsValue {
    name
}

fn main() {}
