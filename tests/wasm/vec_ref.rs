//! Tests for borrowed-vector outgoing arguments (`&Vec<T>` passed from
//! Rust to JS).
//!
//! `&Vec<T>` shares the wire format of owned `Vec<T>` (the elements are
//! cloned into a freshly-allocated buffer that JS owns and must free)
//! but is rendered on the JS side as a plain `Array` rather than a
//! typed array. For string and externref element kinds the existing
//! buffer helper already produces a plain `Array`, so the only place
//! the JS-visible type changes is for primitive numeric kinds.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/vec_ref.js")]
extern "C" {
    fn js_take_vec_ref_u8(v: &Vec<u8>);
    fn js_take_vec_ref_u16(v: &Vec<u16>);
    fn js_take_vec_ref_i32(v: &Vec<i32>);
    fn js_take_vec_ref_f64(v: &Vec<f64>);
    fn js_take_vec_ref_string(v: &Vec<String>);
    fn js_take_vec_ref_optional_u16(v: Option<&Vec<u16>>);

    // Drives the JS-side test that an exported function receiving
    // `Vec<u16>` accepts a plain JS `Array<number>` as input (not just a
    // `Uint16Array`). This tests the *incoming* direction (JS calling
    // Rust) and verifies the existing typed-array buffer ABI is happy
    // to coerce array-likes via `TypedArray.prototype.set`.
    fn js_drive_vec_u16_from_array();
}

/// Exported Rust function receiving an owned `Vec<u16>`. The JS test
/// driver passes either a plain `Array<number>` or a `Uint16Array` and
/// asserts the values arrive correctly.
#[wasm_bindgen]
pub fn rust_consume_vec_u16(v: Vec<u16>) -> u32 {
    let mut sum: u32 = 0;
    for x in &v {
        sum += *x as u32;
    }
    sum
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_u8() {
    let v = vec![1u8, 2, 3];
    js_take_vec_ref_u8(&v);
    // Caller's data is left untouched.
    assert_eq!(v, vec![1u8, 2, 3]);
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_u16() {
    let v = vec![10u16, 20, 30];
    js_take_vec_ref_u16(&v);
    assert_eq!(v, vec![10u16, 20, 30]);
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_i32() {
    let v = vec![-1i32, 0, 1];
    js_take_vec_ref_i32(&v);
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_f64() {
    let v = vec![1.5f64, 2.5, 3.5];
    js_take_vec_ref_f64(&v);
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_string() {
    let v = vec!["hello".to_string(), "world".to_string()];
    js_take_vec_ref_string(&v);
    // Caller's data is left untouched.
    assert_eq!(v.len(), 2);
}

#[wasm_bindgen]
pub fn rust_pass_vec_ref_optional() {
    let v = vec![5u16, 6, 7];
    js_take_vec_ref_optional_u16(Some(&v));
    js_take_vec_ref_optional_u16(None);
}

#[wasm_bindgen_test]
fn vec_ref_u8() {
    rust_pass_vec_ref_u8();
}

#[wasm_bindgen_test]
fn vec_ref_u16() {
    rust_pass_vec_ref_u16();
}

#[wasm_bindgen_test]
fn vec_ref_i32() {
    rust_pass_vec_ref_i32();
}

#[wasm_bindgen_test]
fn vec_ref_f64() {
    rust_pass_vec_ref_f64();
}

#[wasm_bindgen_test]
fn vec_ref_string() {
    rust_pass_vec_ref_string();
}

#[wasm_bindgen_test]
fn vec_ref_optional() {
    rust_pass_vec_ref_optional();
}

#[wasm_bindgen_test]
fn vec_u16_accepts_plain_array_from_js() {
    js_drive_vec_u16_from_array();
}
