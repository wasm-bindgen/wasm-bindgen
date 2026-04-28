use wasm_bindgen::prelude::*;
use wasm_bindgen::Parent;

#[wasm_bindgen]
pub struct Animal {}

#[wasm_bindgen]
pub struct Dog {
    animal: Parent<Animal>,
}

fn main() {}
