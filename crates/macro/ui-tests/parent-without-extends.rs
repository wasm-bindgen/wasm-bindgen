use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Animal {}

#[wasm_bindgen]
pub struct Dog {
    #[wasm_bindgen(parent)]
    animal: Animal,
}

fn main() {}
