use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Foo<T: Clone>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method)]
    pub fn value<T>(this: &Foo<T>);
}

fn main() {}
