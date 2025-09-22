use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "somepackage")]
extern "C" {
    pub type SomeType;
}

#[wasm_bindgen(module = "someotherpackage")]
extern "C" {
    pub type SomeOtherType;
}

#[wasm_bindgen]
pub fn something(a: &SomeType, b: &SomeOtherType) {
    drop(a);
    drop(b);
}
