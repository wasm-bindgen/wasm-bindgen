use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(generic_per_mono)]
    type Holder<'a, T>;

    // Rust permits this lifetime to be omitted in an argument type, but the
    // generated impl cannot use an elided lifetime in its self type.
    #[wasm_bindgen(method, generic_per_mono)]
    fn get<T, U>(this: &Holder<T>) -> U;
}

fn main() {}
