#![cfg(not(target_family = "wasm"))]

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test(unsupported = test)]
fn test_success() {}
