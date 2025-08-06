use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./foo.js")]
extern "C" {
    fn wut();
}

#[wasm_bindgen(module = "../foo.js")]
extern "C" {
    fn wut2();
}

fn main() {}
