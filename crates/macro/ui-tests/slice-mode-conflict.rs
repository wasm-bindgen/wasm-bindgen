use wasm_bindgen::prelude::*;

// `slice_to_array` is an alias of `slice = array`, so writing both at one
// level could only agree or contradict; the pair is rejected naming both
// spans. Different levels do not conflict: the more specific level overrides
// (see the runtime tests). An unknown mode value is rejected at its own span.

#[wasm_bindgen]
extern "C" {
    // On one argument.
    fn per_arg(#[wasm_bindgen(slice_to_array, slice = copy)] xs: &[u8]);

    // On one function.
    #[wasm_bindgen(slice_to_array, slice = array)]
    fn per_fn(xs: &[u8]);

    // An unknown mode value.
    #[wasm_bindgen(slice = borrow)]
    fn unknown_mode(xs: &[u8]);
}

// On one `extern "C"` block.
#[wasm_bindgen(slice_to_array, slice = copy)]
extern "C" {
    fn per_block(xs: &[u8]);
}

fn main() {}
