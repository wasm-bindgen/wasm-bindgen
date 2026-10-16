//! `derive(PartialEq, Eq)` on an imported type compares JS handle identity
//! regardless of its type parameters or its `extends` parent.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsGeneric;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/import_type_eq.js")]
extern "C" {
    #[derive(Clone, Debug, PartialEq, Eq)]
    type G<T: JsGeneric>;

    #[wasm_bindgen(extends = G)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    type H;

    #[wasm_bindgen(constructor)]
    fn new() -> H;

    #[wasm_bindgen(js_name = sameH)]
    fn same_h(h: &H) -> H;

    #[wasm_bindgen(js_name = newG)]
    fn new_g() -> G<JsValue>;
}

fn assert_eq<T: Eq>() {}

#[wasm_bindgen_test]
fn import_type_eq_is_handle_identity() {
    assert_eq::<G<JsValue>>();
    assert_eq::<H>();

    let a = H::new();
    let b = H::new();
    assert_eq!(a, same_h(&a));
    assert_eq!(a, a.clone());
    assert_ne!(a, b);

    let g: G<JsValue> = a.clone().into();
    assert_eq!(g, a.clone().into());
    assert_ne!(g, new_g());
}
