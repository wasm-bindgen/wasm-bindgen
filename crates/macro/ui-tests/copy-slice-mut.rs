use wasm_bindgen::prelude::*;

// Like `slice_to_array`, `copy_slice` hands JS an owned copy — here a fresh
// typed array — so there is nowhere for JS's writes to go; they are discarded
// when the call returns, whereas a plain `&mut [T]` argument gives JS a
// typed-array *view* whose writes land in the caller's buffer. Silent data
// loss with no runtime symptom, so the combination is a compile error. See
// `slice-to-array-mut.rs` for the same rule on the other attribute.

#[wasm_bindgen]
extern "C" {
    // Applied directly to the argument.
    fn direct(#[wasm_bindgen(copy_slice)] xs: &mut [u16]);

    // Applied to the function, inherited by the argument.
    #[wasm_bindgen(copy_slice)]
    fn per_fn(xs: &mut [u32]);

    // The `Option`-wrapped form must be caught too.
    #[wasm_bindgen(copy_slice)]
    fn optional(xs: Option<&mut [f64]>);
}

// Inherited from the enclosing block rather than the function.
#[wasm_bindgen(copy_slice)]
extern "C" {
    fn per_block(xs: &mut [i32]);
}

fn main() {}
