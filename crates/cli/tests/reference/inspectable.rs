// FLAGS: --target=nodejs
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

#[wasm_bindgen(inspectable)]
pub struct Pair(pub u32, pub i32);

#[wasm_bindgen]
impl Pair {
    #[wasm_bindgen(constructor)]
    pub fn new(a: u32, b: i32) -> Pair {
        Pair(a, b)
    }
}
