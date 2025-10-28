// FLAGS: --target=web

use wasm_bindgen::prelude::*;

// This is for testing the type generation of the wasm-exported functions.
// Here, example should be exported as `(arg0: number, arg1: bigint, arg2: any, arg3: number, arg4: number) => [number, number]`.
// Notes: `arg2: any` is an external reference to a JS value, and the ABI of strings is `number, number` (pointer, length).

#[wasm_bindgen]
pub fn example(a: u32, b: u64, c: JsValue, d: &str) -> String {
    todo!()
}

#[wasm_bindgen]
pub fn example_128(a: u128) -> Option<u128> {
    None
}
