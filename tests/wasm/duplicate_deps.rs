use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen_test_crate_a as a;
use wasm_bindgen_test_crate_b as b;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "C" {
    fn assert_next_undefined();
    fn assert_next_ten();
    fn call_apply(dupe: JsValue, x: u32) -> u32;
}

#[wasm_bindgen_test]
fn private_dupes() {
    assert_eq!(call_apply(a::make_dupe().into(), 5), 18);
    assert_eq!(call_apply(b::make_dupe().into(), 5), 19);
}

#[wasm_bindgen_test]
fn works() {
    assert_next_undefined();
    a::test();
    assert_next_ten();
    b::test();
}
