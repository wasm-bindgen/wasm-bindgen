use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/struct_vecs.js")]
extern "C" {
    fn pass_struct_vec();
    fn pass_invalid_struct_vec();
}

#[wasm_bindgen]
pub struct ArrayElement;

#[wasm_bindgen]
impl ArrayElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ArrayElement {
        ArrayElement
    }
}

#[wasm_bindgen]
pub fn consume_struct_vec(mut vec: Vec<ArrayElement>) -> Vec<ArrayElement> {
    vec.push(ArrayElement);
    vec
}

#[wasm_bindgen]
pub fn consume_optional_struct_vec(vec: Option<Vec<ArrayElement>>) -> Option<Vec<ArrayElement>> {
    vec.map(consume_struct_vec)
}

#[wasm_bindgen_test]
fn test_valid() {
    pass_struct_vec();
}

#[wasm_bindgen_test]
fn test_invalid() {
    pass_invalid_struct_vec();
}
