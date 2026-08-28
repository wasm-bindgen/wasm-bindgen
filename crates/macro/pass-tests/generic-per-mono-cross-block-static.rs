use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type External;
}

// A bare, non-generic class does not need declaration metadata from its block.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(static_method_of = External, experimental_generic_mono)]
    fn external_static<T>(value: T);
}

fn assert_cross_block_static(value: JsValue) {
    External::external_static(value);
}

fn main() {}
