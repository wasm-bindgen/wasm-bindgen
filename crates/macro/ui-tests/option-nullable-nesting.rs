use wasm_bindgen::prelude::*;
use wasm_bindgen::Nullable;

#[wasm_bindgen]
pub fn nested(a: Option<Nullable<u32>>) -> Option<Nullable<u32>> {
    a
}

fn main() {}
