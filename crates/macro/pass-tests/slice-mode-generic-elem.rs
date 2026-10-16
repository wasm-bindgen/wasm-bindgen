// A slice whose element type is a type parameter resolves its slice mode
// (`slice = array | copy`) per concrete instantiation on the
// `experimental_generic_mono` path, and downgrades to the default view mode on
// the type-erasure path, where the slice erases to the handle tier (which
// already hands JS a fresh `Array`) and both modes are documented no-ops.
//
// Either way the declaration compiles; this used to be a hard parse error for
// `slice_to_array`. This is what lets one attribute cover a whole
// `extern "C"` block without inspecting every element type.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono, slice = array)]
    fn per_mono_array<T>(xs: &[T], other: T);

    #[wasm_bindgen(experimental_generic_mono, slice = copy)]
    fn per_mono_copy<T>(xs: &[T], other: T);

    // Nested in the element type: the whole element is generic, so the mode
    // resolves per instantiation too.
    #[wasm_bindgen(experimental_generic_mono, slice = array)]
    fn per_mono_nested_elem<T>(xs: &[Vec<T>], other: T);
}

// Inherited from the enclosing block on both paths, through the released
// `slice_to_array` alias.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn erased_array<T>(xs: &[T]);

    #[wasm_bindgen(experimental_generic_mono)]
    fn per_mono_block_array<T>(xs: &[T], other: T);
}

#[wasm_bindgen(slice = copy)]
extern "C" {
    fn erased_copy<T>(xs: &[T]);
}

fn main() {}
