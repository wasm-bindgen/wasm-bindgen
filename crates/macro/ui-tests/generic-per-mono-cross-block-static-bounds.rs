use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type ExternalBounded<T: Clone>;
}

// Repeating the class bound is insufficient because static generic_per_mono
// methods require the imported class declaration in their own extern block.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        static_method_of = ExternalBounded<T>,
        generic_per_mono,
        js_name = create
    )]
    fn external_bounded_static<T: Clone>(value: T);
}

fn main() {}
