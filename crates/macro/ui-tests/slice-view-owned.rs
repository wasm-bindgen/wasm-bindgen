use wasm_bindgen::prelude::*;

// Owned data is freed once the call returns, so it can never reach JS as a
// view: `slice = view` written on an owned argument is rejected, pointing at
// `slice = copy`. Inherited from the function or block, `view` is a no-op on
// owned arguments, as on any other argument it does not apply to.

#[wasm_bindgen]
extern "C" {
    fn vec(#[wasm_bindgen(slice = view)] v: Vec<u8>);
    fn boxed(#[wasm_bindgen(slice = view)] v: Box<[f64]>);
    fn option_vec(#[wasm_bindgen(slice = view)] v: Option<Vec<u16>>);
    fn clamped_vec(#[wasm_bindgen(slice = view)] v: wasm_bindgen::Clamped<Vec<u8>>);

    // Accepted: inherited from the function.
    #[wasm_bindgen(slice = view)]
    fn fn_level(owned: Vec<u8>, borrowed: &[u8]);
}

// Accepted: inherited from the block, and overridden back to `view` from a
// block-level mode on the borrowed argument.
#[wasm_bindgen(slice = array)]
extern "C" {
    fn block_level(#[wasm_bindgen(slice = copy)] owned: Vec<u8>, #[wasm_bindgen(slice = view)] borrowed: &[u8]);
}

#[wasm_bindgen(slice = view)]
extern "C" {
    fn view_block(owned: Vec<u8>);
}

fn main() {}
