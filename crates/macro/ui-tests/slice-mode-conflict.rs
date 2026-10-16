use wasm_bindgen::prelude::*;

// `slice_to_array` and `copy_slice` contradict each other — JS cannot receive
// both a plain `Array` and a fresh typed array — so writing both at one level
// is rejected, naming both spans. Different levels do not conflict: the more
// specific level overrides (see the runtime tests).

#[wasm_bindgen]
extern "C" {
    // On one argument.
    fn per_arg(#[wasm_bindgen(slice_to_array, copy_slice)] xs: &[u8]);

    // On one function.
    #[wasm_bindgen(slice_to_array, copy_slice)]
    fn per_fn(xs: &[u8]);
}

// On one `extern "C"` block.
#[wasm_bindgen(slice_to_array, copy_slice)]
extern "C" {
    fn per_block(xs: &[u8]);
}

fn main() {}
