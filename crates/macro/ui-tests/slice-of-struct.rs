use wasm_bindgen::prelude::*;

// A borrowed slice crosses as each element's ABI, and an exported struct has
// no borrowed ABI to hand JS (only an owned `Vec<T>` moves its elements in).

#[wasm_bindgen]
pub struct Point {
    x: u32,
}

#[wasm_bindgen]
extern "C" {
    fn take_points(points: &[Point]);
}

fn main() {}
