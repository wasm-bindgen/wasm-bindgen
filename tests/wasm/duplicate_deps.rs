use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen_test_crate_a as a;
use wasm_bindgen_test_crate_b as b;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "C" {
    fn assert_next_undefined();
    fn assert_next_ten();
    fn take_generic_cross() -> Vec<JsValue>;
}

#[wasm_bindgen_test]
fn works() {
    assert_next_undefined();
    a::test();
    assert_next_ten();
    b::test();
}

// The generic import is declared and instantiated only in crate `a`; this
// verifies its courier monomorphisations and holed-template rodata survive
// the cross-crate archive pull into the final wasm.
#[wasm_bindgen_test]
fn cross_crate_generic_import() {
    a::generic_test();
    let log = take_generic_cross();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].as_f64(), Some(10.0));
    assert_eq!(log[1].as_string(), Some("hi".to_string()));
}
