use wasm_bindgen::prelude::*;

// An owned `JsValue` is put in the externref table, or on the JS heap, as it
// is passed. The later numeric arguments are converted ahead of the call, so
// one that Wasm rejects throws before the slot is allocated.

#[wasm_bindgen]
pub fn owned_then_u64(a: JsValue, n: u64) -> bool {
    a.is_truthy() && n > 0
}

#[wasm_bindgen]
pub fn option_then_u64(a: Option<JsValue>, n: u64) -> bool {
    a.is_some() && n > 0
}

#[wasm_bindgen]
pub fn option_then_numbers(n: u32, a: Option<JsValue>, x: f64, m: Option<i32>) -> bool {
    a.is_some() && n > 0 && x > 0.0 && m.is_some()
}

#[wasm_bindgen]
pub fn option_without_numbers(a: Option<JsValue>, b: &JsValue) -> bool {
    a.is_some() && b.is_truthy()
}
