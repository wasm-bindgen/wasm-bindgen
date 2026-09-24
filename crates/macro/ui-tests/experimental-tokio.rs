use wasm_bindgen::prelude::*;

#[wasm_bindgen(experimental_tokio)]
pub fn not_async() {}

#[wasm_bindgen(experimental_tokio = "shared")]
pub async fn bad_mode() {}

// Only supported on the emscripten target, so this fails on the host.
#[wasm_bindgen(experimental_tokio)]
pub async fn host_target() {}

fn main() {}
