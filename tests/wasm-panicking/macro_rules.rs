//! This tests that the `wasm_bindgen` macro produces code that compiles for this use case.
//! `cargo test --target wasm32-unknown-unknown` will not run if this test breaks.
use wasm_bindgen::prelude::*;

macro_rules! my_export {
    ($i: ident, $s: ty) => {
        #[wasm_bindgen]
        pub fn $i(_: $s) {}
    };
}

my_export!(should_compile, &[i32]);
