use wasm_bindgen::prelude::*;

#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    pub type Foo<T: Clone>;
}

#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(method)]
    pub fn value<T>(this: &Foo<T>);
}

fn main() {}
