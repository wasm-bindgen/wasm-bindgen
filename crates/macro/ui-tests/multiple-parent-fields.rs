use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Animal {}

#[wasm_bindgen(extends = Animal)]
pub struct Dog {
    #[wasm_bindgen(parent)]
    animal_a: Animal,
    #[wasm_bindgen(parent)]
    animal_b: Animal,
}

fn main() {}
