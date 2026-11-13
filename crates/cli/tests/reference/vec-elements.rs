// Reference test for owned vectors of elements whose memory is not their ABI.
// They cross as their slices do, as a buffer of each element's own ABI, in
// both directions: out of Rust the glue decodes each element with its own
// glue and frees the buffer; into Rust it allocates the buffer and encodes
// each element with its own glue, and Rust takes ownership.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Mode {
    Read = "read",
    Write = "write",
}

#[wasm_bindgen]
pub fn bools(v: Vec<bool>) -> Vec<bool> {
    v
}

#[wasm_bindgen]
pub fn opt_chars(v: Option<Vec<char>>) -> Option<Vec<char>> {
    v
}

#[wasm_bindgen]
pub fn modes(v: Vec<Mode>) -> Vec<Mode> {
    v
}

#[wasm_bindgen]
pub fn strs() -> Vec<&'static str> {
    vec!["a"]
}

#[wasm_bindgen]
extern "C" {
    fn js_bools(v: Vec<bool>) -> Vec<bool>;
    fn js_strs(v: Vec<&str>);
}

#[wasm_bindgen]
pub fn driver() {
    js_bools(js_bools(vec![true]));
    js_strs(vec!["b"]);
}
