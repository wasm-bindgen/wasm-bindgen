use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    type Renamed<'a, T = JsValue>;

    // The Rust class identity, not its independent JS name, must select the
    // unhoisted-return lifetime validation.
    #[wasm_bindgen(constructor, js_class = Different, generic_per_mono)]
    fn new<'a, T: IntoIterator>() -> Renamed<'a, T::Item>;
}

fn main() {}
