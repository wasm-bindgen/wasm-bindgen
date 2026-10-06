// FLAGS: --target=bundler

// Every `inline_js` snippet is snippet 0 of its own macro invocation, so two
// imports with the same Rust signature from two different snippets must still
// get two distinct shims. A single shim would make one of the two call sites
// read the other snippet's export.

use wasm_bindgen::prelude::*;

mod a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const X = 1; export function get() { return 1; }")]
    extern "C" {
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
        pub fn get() -> u32;
    }
}

mod b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export const X = 2; export function get() { return 2; }")]
    extern "C" {
        #[wasm_bindgen(thread_local_v2)]
        pub static X: JsValue;
        pub fn get() -> u32;
    }
}

#[wasm_bindgen]
pub fn read() -> u32 {
    let x = a::X.with(|x| x.as_f64().unwrap()) + b::X.with(|x| x.as_f64().unwrap());
    x as u32 + a::get() + b::get()
}
