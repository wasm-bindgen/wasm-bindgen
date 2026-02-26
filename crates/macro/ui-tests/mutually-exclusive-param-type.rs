use wasm_bindgen::prelude::*;

// Test: unchecked_optional_param_type after unchecked_param_type
#[wasm_bindgen]
pub fn fn_with_both_param_types1(
    #[wasm_bindgen(unchecked_param_type = "string", unchecked_optional_param_type = "string")]
    arg1: JsValue,
) -> JsValue {
    arg1
}

// Test: unchecked_param_type after unchecked_optional_param_type
#[wasm_bindgen]
pub fn fn_with_both_param_types2(
    #[wasm_bindgen(unchecked_optional_param_type = "string", unchecked_param_type = "string")]
    arg2: JsValue,
) -> JsValue {
    arg2
}

fn main() {}
