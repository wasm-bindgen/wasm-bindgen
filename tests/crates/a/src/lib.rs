extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "C" {
    fn foo();

    // A generic import declared and monomorphised only in this dependency
    // crate, to verify the per-monomorphisation courier + holed-template
    // rodata survive the archive pull into the final artifact.
    #[wasm_bindgen(generic)]
    fn generic_record<T>(x: T);
}

pub fn test() {
    foo();
}

pub fn generic_test() {
    generic_record(10u32);
    generic_record("hi");
}
