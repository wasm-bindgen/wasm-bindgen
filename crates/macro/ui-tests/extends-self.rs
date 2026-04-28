use wasm_bindgen::prelude::*;

#[wasm_bindgen(extends = SelfA)]
pub struct SelfA {}

#[wasm_bindgen(extends = Self)]
pub struct SelfB {}

fn main() {}
