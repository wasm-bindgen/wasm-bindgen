use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Animal {}

#[wasm_bindgen(extends = Animal)]
pub struct Dog {
    pub breed: u32,
}

fn main() {}
