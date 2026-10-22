// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }
// FLAGS: --target=experimental-nodejs-module
// FLAGS: --target=experimental-nodejs-module --js-string-builtins
// FLAGS: --target=web --js-string-builtins
// FLAGS: --target=nodejs --js-string-builtins
// FLAGS: --target=bundler --js-string-builtins

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static HELLO: JsString = "Hello, World!";
    #[wasm_bindgen(thread_local_v2, static_string)]
    static EMOJI: JsString = "😀";
    // Shares the table slot of `HELLO`, so the table has two string slots
    // and two imports under `--js-string-builtins`.
    #[wasm_bindgen(thread_local_v2, static_string)]
    static HELLO_AGAIN: JsString = "Hello, World!";
}

#[wasm_bindgen]
pub fn hello() -> JsString {
    HELLO.with(|s| s.clone())
}

#[wasm_bindgen]
pub fn emoji() -> JsString {
    EMOJI.with(|s| s.clone())
}

#[wasm_bindgen]
pub fn hello_again() -> JsString {
    HELLO_AGAIN.with(|s| s.clone())
}
