#![no_implicit_prelude]

extern crate tokio;
extern crate wasm_bindgen_test_macro;
//
use wasm_bindgen_test_macro::wasm_bindgen_test;

#[wasm_bindgen_test(unsupported = tokio::test(flavor = "multi_thread", worker_threads = 1))]
async fn success() {}

#[wasm_bindgen_test(unsupported)]
fn failure_1() {}

#[wasm_bindgen_test(unsupported(test))]
fn failure_2() {}

fn main() {}
