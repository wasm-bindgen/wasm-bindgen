use wasm_bindgen::prelude::*;

// A union's variant may own an exported struct, which has no borrowed ABI
// to hand JS, so a union has none either.

#[wasm_bindgen]
pub enum Shape {
    Circle = "circle",
    Sides(u32),
}

#[wasm_bindgen]
extern "C" {
    fn take_shapes(shapes: &[Shape]);
}

fn main() {}
