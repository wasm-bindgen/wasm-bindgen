use js_sys::Number;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

fn main() {
    // Invalid: `&mut` references are invariant in their pointee, so widening
    // `&mut Vec<Number>` to `&mut Vec<JsValue>` is rejected (issue #5176).
    // Allowing it (the old covariant behavior) would let a non-`Number`
    // `JsValue` be written back through the `&mut Vec<Number>`, leaving it
    // holding a value that is not a `Number`.
    let mut xs: Vec<Number> = Vec::new();
    let _widened: &mut Vec<JsValue> = (&mut xs).upcast_into();
}
