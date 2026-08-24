use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type External;
}

// Static generic_per_mono methods require class metadata from their own block,
// even when the class is non-generic.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(static_method_of = External, generic_per_mono)]
    fn external_static<T>(value: T);
}

fn main() {}
