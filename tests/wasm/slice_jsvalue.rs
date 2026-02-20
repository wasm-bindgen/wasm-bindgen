use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/slice_jsvalue.js")]
extern "C" {
    fn js_receive_slice(values: &[JsValue]) -> u32;
    fn js_receive_slice_and_sum(values: &[JsValue]) -> f64;
    fn js_verify_slice_values(values: &[JsValue]) -> bool;
}

#[wasm_bindgen_test]
fn pass_jsvalue_slice_to_js() {
    let arr: Vec<JsValue> = vec![1.into(), 2.into(), 3.into()];
    let len = js_receive_slice(&arr);
    assert_eq!(len, 3);
    let len = js_receive_slice(&arr);
    assert_eq!(len, 3);
}

#[wasm_bindgen_test]
fn pass_jsvalue_slice_sum() {
    let arr: Vec<JsValue> = vec![10.into(), 20.into(), 30.into()];
    let sum = js_receive_slice_and_sum(&arr);
    assert_eq!(sum, 60.0);
}

#[wasm_bindgen_test]
fn pass_mixed_jsvalue_slice() {
    let arr: Vec<JsValue> = vec![42.into(), "hello".into(), true.into()];
    let verified = js_verify_slice_values(&arr);
    assert!(verified);
}

#[wasm_bindgen_test]
fn pass_empty_jsvalue_slice() {
    let arr: Vec<JsValue> = vec![];
    let len = js_receive_slice(&arr);
    assert_eq!(len, 0);
}
