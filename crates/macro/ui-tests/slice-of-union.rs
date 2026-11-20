use wasm_bindgen::prelude::*;

// A union is lent as each of its payloads is, and an exported struct has no
// borrowed ABI to hand JS, so a union that owns one has none either.

#[wasm_bindgen]
pub struct Point {
    x: u32,
}

#[wasm_bindgen]
pub enum Shape {
    Circle = "circle",
    Sides(u32),
    At(Point),
}

#[wasm_bindgen]
extern "C" {
    fn take_shapes(shapes: &[Shape]);
}

fn main() {}
