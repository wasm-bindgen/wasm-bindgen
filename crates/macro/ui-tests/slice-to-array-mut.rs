use wasm_bindgen::prelude::*;

// `slice_to_array` hands JS an owned `Array` copied out of linear memory, so
// there is nowhere for JS's writes to that `Array` to go — they are discarded
// when the call returns. A plain `&mut [T]` argument instead gives JS a
// typed-array *view* into wasm memory, where writes do land in the caller's
// buffer.
//
// The two are therefore incompatible rather than merely unimplemented, and the
// failure mode is silent data loss with no runtime symptom, so the combination is
// rejected. `slice_to_array` has only ever been documented for `&[T]` and
// `Option<&[T]>`, so nothing supported is being taken away.

#[wasm_bindgen]
extern "C" {
    // Applied directly to the argument.
    fn direct(#[wasm_bindgen(slice_to_array)] xs: &mut [u16]);

    // Applied to the function, inherited by the argument.
    #[wasm_bindgen(slice_to_array)]
    fn per_fn(xs: &mut [u32]);

    // The `Option`-wrapped form must be caught too.
    #[wasm_bindgen(slice_to_array)]
    fn optional(xs: Option<&mut [f64]>);

    // A shared slice alongside a mutable one: only the mutable argument is
    // rejected.
    #[wasm_bindgen(slice_to_array)]
    fn mixed(readable: &[u8], writable: &mut [u8]);

    // Two offending arguments in one signature: both are reported in a
    // single compile rather than aborting at the first.
    #[wasm_bindgen(slice_to_array)]
    fn two_muts(a: &mut [u8], b: &mut [u16]);

    // Doubly invalid: the slice is `&mut` *and* its element type is a type
    // parameter (see `slice-to-array-generic-elem.rs` for that restriction on
    // its own). Only the `&mut`-slice error is reported; the generic-element
    // one surfaces on the next compile once the `&mut` is fixed.
    #[wasm_bindgen(slice_to_array)]
    fn generic_and_mut<T>(xs: &mut [T]);
}

// Inherited from the enclosing block rather than the function. This is the case a
// user is most likely to hit without having written `slice_to_array` anywhere
// near the offending argument.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn per_block(xs: &mut [i32]);
}

fn main() {}
