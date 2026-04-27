//! Tests specific to the wasm64/memory64 architecture.
//!
//! These tests exercise code paths that differ between wasm32 and wasm64,
//! such as 64-bit pointer handling, JS-number pointer conversions, and
//! correct round-tripping of values through the wasm64 ABI.

#![cfg(test)]

use wasm_bindgen::__rt::WasmWord;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/wasm64.js")]
extern "C" {
    fn js_verify_pointer_size() -> bool;
    fn js_roundtrip_large_slice(slice: &[u8]) -> Vec<u8>;
    fn js_create_and_free_class() -> bool;
    fn js_call_closure_returning_usize() -> bool;
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
fn test_wasm_word_roundtrip() {
    let large_word = 1usize << 60;
    assert_eq!(WasmWord::from_usize(large_word).into_usize(), large_word);

    let signed_word = -(1isize << 40);
    assert_eq!(WasmWord::from_isize(signed_word).into_isize(), signed_word);
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
