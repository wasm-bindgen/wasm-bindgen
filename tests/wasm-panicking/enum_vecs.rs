use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/enum_vecs.js")]
extern "C" {
    fn pass_enum_vec();
    fn pass_invalid_enum_vec();
}

#[wasm_bindgen]
pub enum EnumArrayElement {
    Unit,
}

#[wasm_bindgen]
pub fn consume_enum_vec(mut vec: Vec<EnumArrayElement>) -> Vec<EnumArrayElement> {
    vec.push(EnumArrayElement::Unit);
    vec
}

#[wasm_bindgen]
pub fn consume_optional_enum_vec(
    vec: Option<Vec<EnumArrayElement>>,
) -> Option<Vec<EnumArrayElement>> {
    vec.map(consume_enum_vec)
}

#[wasm_bindgen_test]
#[ignore]
fn test_valid() {
    pass_enum_vec();
}

#[wasm_bindgen_test]
#[ignore]
fn test_invalid() {
    pass_invalid_enum_vec();
}
