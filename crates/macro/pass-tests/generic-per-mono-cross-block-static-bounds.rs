use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type ExternalBounded<T: Clone>;
}

// Explicit class arguments carry enough information to form this impl even
// though the imported class declaration belongs to another extern block. Its
// declaration bounds must still be present on the generated impl.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        static_method_of = ExternalBounded<T>,
        generic_per_mono,
        js_name = create
    )]
    fn external_bounded_static<T: Clone>(value: T);
}

fn assert_cross_block_static_bound(value: JsValue) {
    ExternalBounded::<JsValue>::external_bounded_static(value);
}

fn main() {}
