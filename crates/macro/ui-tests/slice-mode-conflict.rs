use wasm_bindgen::prelude::*;

// The deprecated `slice_to_array` and `slice = ..` at one level could only
// agree or contradict, so the pair is rejected naming both spans. Different
// levels do not conflict: the more specific level overrides (see the runtime
// tests), and `vec = ..` doesn't overlap `slice_to_array`. An unknown,
// reserved or meaningless mode value is rejected at its own span, on exports
// too, where a valid mode is inert.

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

    // Owned data is freed after the call, so it is never a view.
    fn vec_view(#[wasm_bindgen(vec = view)] xs: Vec<u8>);

    #[wasm_bindgen(vec = borrow)]
    fn unknown_vec_mode(xs: Vec<u8>);

    // Accepted.
    #[wasm_bindgen(slice_to_array, vec = array)]
    fn alias_with_vec(xs: &[u8], ys: Vec<u8>);
}

// On one `extern "C"` block.
#[wasm_bindgen(slice_to_array, slice = copy)]
extern "C" {
    fn per_block(xs: &[u8]);
}

// Both keys are rejected on exports, whatever their value, where
// `slice_to_array` keeps its released no-op.
#[wasm_bindgen]
pub fn export_slice(
    #[wasm_bindgen(slice = copy)] _xs: &[u8],
    #[wasm_bindgen(vec = borrow)] _ys: Vec<u8>,
    #[wasm_bindgen(slice_to_array)] _zs: &[u8],
) {
}

#[wasm_bindgen(slice = copy)]
pub fn export_fn_slice(_xs: &[u8]) {}

#[wasm_bindgen]
pub struct Exported;

#[wasm_bindgen]
impl Exported {
    pub fn method(#[wasm_bindgen(vec = array)] _xs: Vec<u8>) {}

    #[wasm_bindgen(slice = array)]
    pub fn fn_method(_xs: &[u8]) {}
}

#[wasm_bindgen(vec = copy)]
impl Exported {
    pub fn block_method(_xs: Vec<u8>) {}
}

fn main() {}
