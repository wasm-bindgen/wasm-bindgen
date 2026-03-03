//! Tests specific to the wasm64/memory64 architecture.
//!
//! These tests exercise code paths that differ between wasm32 and wasm64,
//! such as 64-bit pointer handling, BigInt conversions in JS glue code,
//! and correct round-tripping of values through the wasm64 ABI.

#![cfg(test)]

use std::ptr::NonNull;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/wasm64.js")]
extern "C" {
    fn js_verify_pointer_size() -> bool;
    fn js_roundtrip_large_slice(slice: &[u8]) -> Vec<u8>;
    fn js_create_and_free_class() -> bool;
    fn js_call_closure_returning_usize() -> bool;
    fn js_test_option_nonnull_none() -> bool;
    fn js_test_option_nonnull_some() -> bool;
    fn js_test_option_ptr_roundtrip() -> bool;
    fn js_test_jsvalue_array_roundtrip() -> bool;
}

// A simple exported class to test class creation/destruction with 64-bit pointers.
#[wasm_bindgen]
pub struct Wasm64TestClass {
    value: u64,
}

#[wasm_bindgen]
impl Wasm64TestClass {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u64) -> Self {
        Wasm64TestClass { value }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    pub fn add(&self, other: u64) -> u64 {
        self.value + other
    }
}

#[wasm_bindgen]
pub fn wasm64_return_usize() -> usize {
    // Return a value that would overflow u32 to verify 64-bit handling
    0x1_0000_0001usize
}

#[wasm_bindgen]
pub fn wasm64_accept_usize(val: usize) -> bool {
    val == 0x1_0000_0001usize
}

#[wasm_bindgen]
pub fn wasm64_slice_length(data: &[u8]) -> usize {
    data.len()
}

#[wasm_bindgen]
pub fn wasm64_return_vec(len: usize) -> Vec<u8> {
    vec![42u8; len]
}

#[wasm_bindgen]
pub fn wasm64_closure_returning_usize() -> usize {
    let closure =
        Closure::wrap(Box::new(|| -> usize { 0x1_0000_0001usize }) as Box<dyn Fn() -> usize>);
    let _js_val = closure.as_ref().clone();
    // We can't easily call the closure from Rust side, but we verify it compiles
    // and the closure type is correct for wasm64. The JS side will test the actual call.
    closure.forget();
    0x1_0000_0001usize
}

#[wasm_bindgen_test]
fn test_pointer_size() {
    assert_eq!(
        std::mem::size_of::<usize>(),
        8,
        "usize should be 8 bytes on wasm64"
    );
    assert_eq!(
        std::mem::size_of::<*const u8>(),
        8,
        "pointers should be 8 bytes on wasm64"
    );
}

#[wasm_bindgen_test]
fn test_usize_roundtrip() {
    // Verify that usize values larger than u32::MAX work correctly
    let large_val: usize = 0x1_0000_0001;
    assert_eq!(large_val as u64, 4294967297u64);
}

#[wasm_bindgen_test]
fn test_class_operations() {
    let obj = Wasm64TestClass::new(42);
    assert_eq!(obj.get_value(), 42);
    assert_eq!(obj.add(8), 50);
}

#[wasm_bindgen_test]
fn test_js_verify_pointer_size() {
    assert!(js_verify_pointer_size());
}

#[wasm_bindgen_test]
fn test_js_slice_roundtrip() {
    let data = vec![1u8, 2, 3, 4, 5];
    let result = js_roundtrip_large_slice(&data);
    assert_eq!(result, data);
}

#[wasm_bindgen_test]
fn test_js_class_create_and_free() {
    assert!(js_create_and_free_class());
}

#[wasm_bindgen_test]
fn test_js_closure_usize() {
    assert!(js_call_closure_returning_usize());
}

// Bug #1: Option<NonNull<T>> — None must produce undefined, not 0
#[wasm_bindgen]
pub fn wasm64_option_nonnull_none() -> Option<NonNull<u8>> {
    None
}

#[wasm_bindgen]
pub fn wasm64_option_nonnull_some() -> Option<NonNull<u8>> {
    // Use a non-null pointer (dangling is fine for this test — never dereferenced)
    Some(NonNull::dangling())
}

#[wasm_bindgen_test]
fn test_option_nonnull_none() {
    assert!(js_test_option_nonnull_none());
}

#[wasm_bindgen_test]
fn test_option_nonnull_some() {
    assert!(js_test_option_nonnull_some());
}

// Bug #3: Option<*const T> round-trip through JS
#[wasm_bindgen]
pub fn wasm64_option_ptr_none() -> Option<*const u8> {
    None
}

#[wasm_bindgen]
pub fn wasm64_option_ptr_some() -> Option<*const u8> {
    Some(0x42usize as *const u8)
}

#[wasm_bindgen_test]
fn test_option_ptr_roundtrip() {
    assert!(js_test_option_ptr_roundtrip());
}

// Bug #2: JsValue array round-trip (stride must be 4, not 8)
#[wasm_bindgen]
pub fn wasm64_jsvalue_array_roundtrip(vals: Vec<JsValue>) -> Vec<JsValue> {
    vals
}

#[wasm_bindgen_test]
fn test_jsvalue_array_roundtrip() {
    assert!(js_test_jsvalue_array_roundtrip());
}
