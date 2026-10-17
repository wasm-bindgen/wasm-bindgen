// A slice whose element type is a type parameter resolves its slice mode
// (`slice = array | copy`) per concrete instantiation on the
// `experimental_generic_mono` path, and against the erased slice on the
// type-erasure path. Either way the CLI resolves it, as it can see the
// element.
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

    // On the per-mono path a slice needs only its element's
    // `VectorRefIntoWasmAbi`, so a bare generic slice needs no mode,
    // `Option<&[T]>` is not a rejected "nested reference", and `String` is a
    // valid element (and, at the call site, a valid `T`).
    #[wasm_bindgen(experimental_generic_mono)]
    fn per_mono_bare<T>(xs: &[T], other: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn per_mono_option<T>(xs: Option<&[T]>, other: T);

    #[wasm_bindgen(experimental_generic_mono, slice = copy)]
    fn per_mono_option_copy<T>(xs: Option<&[T]>, other: T);

    #[wasm_bindgen(experimental_generic_mono, slice = array)]
    fn per_mono_option_array<T>(xs: Option<&[T]>, other: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn per_mono_concrete_strings<T>(xs: &[String], other: T);
}

// Inherited from the enclosing block on both paths, through the deprecated
// `slice_to_array`.
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
