use wasm_bindgen::prelude::*;

// Define structs in non-alphabetical order to test that their definition order
// is preserved in the Wasm, but the typescript output is sorted alphabetically.

#[wasm_bindgen]
pub struct Zebra {
    z: u32,
}

#[wasm_bindgen]
pub struct Apple {
    a: u32,
}

#[wasm_bindgen]
pub struct Mango {
    m: u32,
}

#[wasm_bindgen]
pub struct Banana {
    b: u32,
}

