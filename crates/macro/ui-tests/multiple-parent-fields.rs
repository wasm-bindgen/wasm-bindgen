use wasm_bindgen::prelude::*;
use wasm_bindgen::Parent;

#[wasm_bindgen]
pub struct Animal {}

#[wasm_bindgen(extends = Animal)]
pub struct Dog {
    animal_a: Parent<Animal>,
    animal_b: Parent<Animal>,
}

fn main() {}
