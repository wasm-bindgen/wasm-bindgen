//! This is a test that we compile `wasm-bindgen` itself in `no_std` mode and we
//! can export/import various items.
//!
//! This doesn't actually run any tests, it's mostly a compile-time verification
//! that things work.

#![no_std]
#![allow(dead_code)]

extern crate std as _some_other_name;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(_a: u32) {}

#[wasm_bindgen]
extern "C" {
    // fn test(a: &str);

    type Js;
    // #[wasm_bindgen(constructor)]
    // fn new() -> Js;
    // #[wasm_bindgen(method, structural)]
    // fn init(this: &Js);
}
