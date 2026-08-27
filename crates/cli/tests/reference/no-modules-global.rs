// FLAGS: --target=no-modules --no-modules-global=custom_global

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
