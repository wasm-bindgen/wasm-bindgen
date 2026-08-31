use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    type External<T: Clone>;
}

mod foreign {
    #[allow(unused_imports)]
    pub(crate) use super::External;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, generic_per_mono)]
    fn external_method<T: Clone>(this: &foreign::External<T>);
}

fn main() {}
