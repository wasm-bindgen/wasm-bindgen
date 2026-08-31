use wasm_bindgen::prelude::*;

#[wasm_bindgen(generic_per_mono)]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    pub type Foo<T: Clone>;
}

#[wasm_bindgen(generic_per_mono)]
extern "C" {
    #[wasm_bindgen(method)]
    pub fn value<T>(this: &Foo<T>);
}

fn main() {}
