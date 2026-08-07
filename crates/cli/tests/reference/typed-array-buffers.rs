// FLAGS: --ts-typed-array-buffers

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn owned_bytes() -> Vec<u8> {
    Vec::new()
}

#[wasm_bindgen]
pub fn owned_floats() -> Box<[f32]> {
    Box::new([])
}

#[wasm_bindgen]
pub fn optional_bytes() -> Option<Vec<u8>> {
    None
}

#[wasm_bindgen]
pub async fn async_bytes() -> Vec<u8> {
    Vec::new()
}

#[wasm_bindgen]
pub fn roundtrip(borrowed: &[u8], owned: Vec<u8>) -> Vec<u8> {
    let _ = owned;
    borrowed.to_vec()
}

#[wasm_bindgen]
pub fn strings(input: Vec<String>) -> Vec<String> {
    input
}
