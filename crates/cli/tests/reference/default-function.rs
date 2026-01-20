use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = default)]
pub fn default_function(a: i32, b: i32) -> i32 {
    a + b
}
