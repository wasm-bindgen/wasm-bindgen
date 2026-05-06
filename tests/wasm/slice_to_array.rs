//! Tests for the `slice_to_array` attribute on imported functions.
//!
//! The attribute lets a `&[T]` (or `Option<&[T]>`) outgoing argument
//! arrive on the JS side as a plain `Array` rather than a typed array
//! (for primitive element kinds), without changing the Rust-side
//! `&[T]` signature. The slice contents are cloned into a freshly
//! allocated buffer that JS owns and frees; the caller's slice is
//! left untouched.
//!
//! Coverage:
//! - per-fn `slice_to_array` for primitive element kinds
//!   (`u8`, `u16`, `i32`, `f64`)
//! - `&[String]`
//! - `&[T]` where `T` is a JS-imported type
//! - `Option<&[T]>` (both `Some` and `None`)
//! - block-level `slice_to_array` applying to all imports in the block
//!
//! `&[T]` for Rust-exported struct types is intentionally not supported
//! (mirroring the existing default `&[T]` behaviour, where exported
//! structs are also unsupported). Use `Vec<T>` to transfer ownership of
//! a sequence of exported struct values to JS.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/slice_to_array.js")]
extern "C" {
    // Imported JS type — exercises `&[ImportedType]` element handling.
    // No `Clone` bound is required: `slice_to_array` builds a fresh
    // `[u32]` index buffer via `&T -> JsValue`, which for handle-shaped
    // types is a refcount bump on the existing externref slot.
    pub type Tagged;

    #[wasm_bindgen(constructor)]
    fn new(tag: &str) -> Tagged;

    #[wasm_bindgen(method, getter)]
    fn tag(this: &Tagged) -> String;

    // Per-fn `slice_to_array`.
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_u8(v: &[u8]) -> Vec<u8>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_u16(v: &[u16]) -> Vec<u16>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_i32(v: &[i32]) -> Vec<i32>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_f64(v: &[f64]) -> Vec<f64>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_string(v: &[String]) -> Vec<String>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_imported(v: &[Tagged]) -> Vec<String>;
    #[wasm_bindgen(slice_to_array)]
    fn js_st2a_optional_u16(v: Option<&[u16]>) -> Option<Vec<u16>>;
}

// Block-level `slice_to_array` propagates to every fn in the block.
#[wasm_bindgen(module = "tests/wasm/slice_to_array.js", slice_to_array)]
extern "C" {
    fn js_st2a_block_a(v: &[u16]) -> Vec<u16>;
    fn js_st2a_block_b(v: &[String]) -> Vec<String>;
}

#[wasm_bindgen_test]
fn primitive_u8() {
    let v = vec![1u8, 2, 3];
    let out = js_st2a_u8(&v);
    assert_eq!(out, vec![2u8, 4, 6]);
    assert_eq!(v, vec![1u8, 2, 3]);
}

#[wasm_bindgen_test]
fn primitive_u16() {
    let v = [10u16, 20, 30];
    let out = js_st2a_u16(&v);
    assert_eq!(out, vec![20u16, 40, 60]);
}

#[wasm_bindgen_test]
fn primitive_i32() {
    let v = vec![-1i32, 0, 1];
    let out = js_st2a_i32(&v);
    assert_eq!(out, vec![-2i32, 0, 2]);
}

#[wasm_bindgen_test]
fn primitive_f64() {
    let v = vec![1.5f64, 2.5, 3.5];
    let out = js_st2a_f64(&v);
    assert_eq!(out, vec![3.0f64, 5.0, 7.0]);
}

#[wasm_bindgen_test]
fn string_elements() {
    let v = vec!["hello".to_string(), "world".to_string()];
    let out = js_st2a_string(&v);
    assert_eq!(out, vec!["hello!".to_string(), "world!".to_string()]);
    assert_eq!(v, vec!["hello".to_string(), "world".to_string()]);
}

#[wasm_bindgen_test]
fn imported_type_elements() {
    let v = vec![Tagged::new("a"), Tagged::new("b"), Tagged::new("c")];
    let tags = js_st2a_imported(&v);
    assert_eq!(tags, vec!["a", "b", "c"]);
    // `v` was cloned by the macro (each `Tagged` is a `JsValue` wrapper,
    // which is `Clone`), so the originals remain usable here.
    assert_eq!(v[0].tag(), "a");
}

#[wasm_bindgen_test]
fn optional() {
    let v = vec![5u16, 6, 7];
    let out = js_st2a_optional_u16(Some(&v));
    assert_eq!(out, Some(vec![10u16, 12, 14]));
    assert_eq!(v, vec![5u16, 6, 7]);
    assert_eq!(js_st2a_optional_u16(None), None);
}

#[wasm_bindgen_test]
fn block_level() {
    let v = vec![1u16, 2, 3];
    let out = js_st2a_block_a(&v);
    assert_eq!(out, vec![2u16, 4, 6]);

    let s = vec!["x".to_string(), "y".to_string()];
    let out = js_st2a_block_b(&s);
    assert_eq!(out, vec!["x!".to_string(), "y!".to_string()]);
}
