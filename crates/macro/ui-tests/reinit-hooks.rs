use wasm_bindgen::prelude::*;

// Case 1: start + pre_reinit_hook conflict
#[wasm_bindgen(start, pre_reinit_hook)]
fn f1() {}

// Case 2: start + post_reinit_hook conflict
#[wasm_bindgen(start, post_reinit_hook)]
fn f2() {}

// Case 3: both hooks on the same function
#[wasm_bindgen(pre_reinit_hook, post_reinit_hook)]
fn f3() {}

// Case 4: async pre_reinit_hook
#[wasm_bindgen(pre_reinit_hook)]
async fn f4() {}

// Case 5: async post_reinit_hook
#[wasm_bindgen(post_reinit_hook)]
async fn f5() {}

// Case 6: pre_reinit_hook with generics
#[wasm_bindgen(pre_reinit_hook)]
fn f6<T>() {}

// Case 7: pre_reinit_hook with arguments
#[wasm_bindgen(pre_reinit_hook)]
fn f7(x: u32) {}

// Case 8: post_reinit_hook with generics
#[wasm_bindgen(post_reinit_hook)]
fn f8<T>() {}

// Case 9: post_reinit_hook with more than one argument
#[wasm_bindgen(post_reinit_hook)]
fn f9(a: u32, b: u32) {}

// Case 10: post_reinit_hook argument not Option<T>
#[wasm_bindgen(post_reinit_hook)]
fn f10(x: String) {}

// Case 11: post_reinit_hook with a return value
#[wasm_bindgen(post_reinit_hook)]
fn f11() -> u32 {
    0
}

fn main() {}
