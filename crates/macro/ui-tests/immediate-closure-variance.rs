use js_sys::Number;
use wasm_bindgen::prelude::*;

fn main() {
    // Invalid: even though u32 can upcast to Number in JS,
    // Rust closures are not erasably generic on Rust types.
    // The closure returns u32, not Number - the conversion happens
    // at the wasm-bindgen boundary, not in the closure itself.
    let mut closure = || 42u32;
    let immediate: ImmediateClosure<dyn FnMut() -> u32> = ImmediateClosure::new_mut(&mut closure);
    let _bad: &ImmediateClosure<dyn FnMut() -> Number> = immediate.upcast_ref();

    // Same issue with arguments: even though Number can be converted to u32,
    // the closure expects u32, not Number
    let mut closure2 = |_: Number| {};
    let immediate2: ImmediateClosure<dyn FnMut(Number)> = ImmediateClosure::new_mut(&mut closure2);
    let _bad2: &ImmediateClosure<dyn FnMut(u32)> = immediate2.upcast_ref();
}
