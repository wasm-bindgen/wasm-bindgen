//! Roundtrip tests for borrowed-vector outgoing arguments (`&Vec<T>`).
//!
//! Each test passes `&Vec<T>` from Rust into a JS function which
//! observes the value as a plain `Array`, transforms it, and returns
//! the result. Rust verifies the returned values and that the original
//! `Vec<T>` was not modified.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/vec_ref.js")]
extern "C" {
    fn js_roundtrip_vec_ref_u8(v: &Vec<u8>) -> Vec<u8>;
    fn js_roundtrip_vec_ref_u16(v: &Vec<u16>) -> Vec<u16>;
    fn js_roundtrip_vec_ref_i32(v: &Vec<i32>) -> Vec<i32>;
    fn js_roundtrip_vec_ref_f64(v: &Vec<f64>) -> Vec<f64>;
    fn js_roundtrip_vec_ref_string(v: &Vec<String>) -> Vec<String>;
    fn js_roundtrip_vec_ref_optional_u16(v: Option<&Vec<u16>>) -> Option<Vec<u16>>;
}

#[wasm_bindgen_test]
fn roundtrip_u8() {
    let v = vec![1u8, 2, 3];
    let out = js_roundtrip_vec_ref_u8(&v);
    assert_eq!(out, vec![2u8, 4, 6]);
    // Caller's data is left untouched.
    assert_eq!(v, vec![1u8, 2, 3]);
}

#[wasm_bindgen_test]
fn roundtrip_u16() {
    let v = vec![10u16, 20, 30];
    let out = js_roundtrip_vec_ref_u16(&v);
    assert_eq!(out, vec![20u16, 40, 60]);
    assert_eq!(v, vec![10u16, 20, 30]);
}

#[wasm_bindgen_test]
fn roundtrip_i32() {
    let v = vec![-1i32, 0, 1];
    let out = js_roundtrip_vec_ref_i32(&v);
    assert_eq!(out, vec![-2i32, 0, 2]);
    assert_eq!(v, vec![-1i32, 0, 1]);
}

#[wasm_bindgen_test]
fn roundtrip_f64() {
    let v = vec![1.5f64, 2.5, 3.5];
    let out = js_roundtrip_vec_ref_f64(&v);
    assert_eq!(out, vec![3.0f64, 5.0, 7.0]);
    assert_eq!(v, vec![1.5f64, 2.5, 3.5]);
}

#[wasm_bindgen_test]
fn roundtrip_string() {
    let v = vec!["hello".to_string(), "world".to_string()];
    let out = js_roundtrip_vec_ref_string(&v);
    assert_eq!(out, vec!["hello!".to_string(), "world!".to_string()]);
    assert_eq!(v, vec!["hello".to_string(), "world".to_string()]);
}

#[wasm_bindgen_test]
fn roundtrip_optional() {
    let v = vec![5u16, 6, 7];
    let out = js_roundtrip_vec_ref_optional_u16(Some(&v));
    assert_eq!(out, Some(vec![10u16, 12, 14]));
    assert_eq!(v, vec![5u16, 6, 7]);
    assert_eq!(js_roundtrip_vec_ref_optional_u16(None), None);
}
