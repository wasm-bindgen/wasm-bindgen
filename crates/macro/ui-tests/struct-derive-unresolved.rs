use wasm_bindgen::prelude::wasm_bindgen;

// Simulates the re-invocation that occurs when the emitted
// `#[derive(::wasm_bindgen::__rt::BindgenedStruct)]` fails to resolve (e.g.
// when `wasm-bindgen` is not a direct dependency): rustc strips the failed
// derive and re-invokes the attribute macro with the marker still attached,
// which must produce a proper error rather than recursing indefinitely.
#[wasm_bindgen]
#[__wasm_bindgen_retried]
struct Foo;

fn main() {}
