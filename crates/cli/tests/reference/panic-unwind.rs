// FLAGS: --target=bundler

// This test is built with `-C panic=unwind` and `-Zbuild-std=std,panic_unwind`
// (see `runtest_panic_unwind` in the reference test harness). Under those
// flags the wasm has Wasm exception-handling instructions and an
// `__instance_terminated` global, which makes wasm-bindgen import
// `WebAssembly.JSTag` and emit a `__wbindgen_wrapped_jstag` constant for the
// catch-handler machinery. The bundler target's emit of that import is the
// thing we want to lock in via snapshot.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn maybe_panic(x: u32) -> u32 {
    if x == 0 {
        panic!("zero");
    }
    x + 1
}
