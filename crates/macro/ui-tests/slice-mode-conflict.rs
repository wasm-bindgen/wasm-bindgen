use wasm_bindgen::prelude::*;

// The deprecated `slice_to_array` and `slice = ..` at one level could only
// agree or contradict, so the pair is rejected naming both spans. Different
// levels do not conflict: the more specific level overrides (see the runtime
// tests). An unknown or reserved mode value is rejected at its own span, on
// exports too, where a valid mode is inert.

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

    // A reserved mode value.
    #[wasm_bindgen(slice = transfer)]
    fn reserved_mode(xs: Vec<u8>);
}

// On one `extern "C"` block.
#[wasm_bindgen(slice_to_array, slice = copy)]
extern "C" {
    fn per_block(xs: &[u8]);
}

// On an export argument.
#[wasm_bindgen]
pub fn export_unknown_mode(#[wasm_bindgen(slice = borrow)] _xs: &[u8]) {}

fn main() {}
