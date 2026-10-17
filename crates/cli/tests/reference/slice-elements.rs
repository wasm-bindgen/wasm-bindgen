// Reference test for borrowed slices of elements whose memory is not their
// ABI. The slice crosses as a buffer of each element's own ABI, which the
// glue walks, decoding each element with that element type's own glue, and
// then frees: `(ptr, len)` pairs decoded as strings, `u32`s as booleans,
// code points or enum values.
//
// `js_handles` pins the borrowed tier alongside: a handle's memory is its
// table index, so the slice is read in place, with no buffer to free.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Level {
    Low = 1,
    High = 2,
}

#[wasm_bindgen]
pub enum Mode {
    Read = "read",
    Write = "write",
}

#[wasm_bindgen]
extern "C" {
    fn js_strings(v: &[String]);
    fn js_strs(v: &[&str]);
    fn js_bools(v: &[bool]);
    fn js_opt_chars(v: Option<&[char]>);
    fn js_levels(v: &[Level]);
    fn js_modes(v: &[Mode]);
    fn js_handles(v: &[JsValue]);
}

#[wasm_bindgen]
pub fn driver() {
    js_strings(&["a".to_string()]);
    js_strs(&["b"]);
    js_bools(&[true]);
    js_opt_chars(Some(&['c']));
    js_levels(&[Level::High]);
    js_modes(&[Mode::Write]);
    js_handles(&[JsValue::NULL]);
}
