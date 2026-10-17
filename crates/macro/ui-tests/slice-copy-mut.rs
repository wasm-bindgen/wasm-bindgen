use wasm_bindgen::prelude::*;

// Like `slice = array`, `slice = copy` hands JS an owned copy — here a fresh
// typed array — so there is nowhere for JS's writes to go; they are discarded
// when the call returns, whereas a plain `&mut [T]` argument gives JS a
// typed-array *view* whose writes land in the caller's buffer. Silent data
// loss with no runtime symptom, so the combination is a compile error. See
// `slice-to-array-mut.rs` for the same rule on the `array` mode.

#[wasm_bindgen]
extern "C" {
    // Applied directly to the argument.
    fn direct(#[wasm_bindgen(slice = copy)] xs: &mut [u16]);

    // Applied to the function, inherited by the argument.
    #[wasm_bindgen(slice = copy)]
    fn per_fn(xs: &mut [u32]);

    // The `Option`- and `Clamped`-wrapped forms must be caught too.
    #[wasm_bindgen(slice = copy)]
    fn optional(xs: Option<&mut [f64]>);
    #[wasm_bindgen(slice = copy)]
    fn clamped(xs: wasm_bindgen::Clamped<&mut [u8]>);
}

// Inherited from the enclosing block rather than the function. An explicit
// `slice = view` on the argument is the documented way back to the writable
// view, so `writable` must compile while `rejected` must not.
#[wasm_bindgen(slice = copy)]
extern "C" {
    fn per_block(rejected: &mut [i32], #[wasm_bindgen(slice = view)] writable: &mut [i32]);
}

fn main() {}
