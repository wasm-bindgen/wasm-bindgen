use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, PartialEq)]
    pub type PartialOnly;

    #[derive(Clone, Debug)]
    pub type NoEq;
}

fn assert_eq<T: Eq>() {}
fn assert_partial_eq<T: PartialEq>() {}

fn main() {
    assert_eq::<PartialOnly>();
    assert_partial_eq::<NoEq>();
}
