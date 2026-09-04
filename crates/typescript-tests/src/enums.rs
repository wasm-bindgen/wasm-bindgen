use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Foo {
    A = 1,
    B = 3,
}

#[wasm_bindgen]
pub fn fn_expects_enum(_: Foo) {}

#[wasm_bindgen]
pub fn fn_returns_enum() -> Foo {
    Foo::A
}

#[wasm_bindgen]
pub fn fn_expects_option_enum(_: Option<Foo>) {}

#[wasm_bindgen]
pub fn fn_returns_option_enum() -> Option<Foo> {
    Some(Foo::A)
}

#[wasm_bindgen]
pub fn fn_expects_enum_vec(_: Vec<Foo>) {}

#[wasm_bindgen]
pub fn fn_returns_enum_vec() -> Vec<Foo> {
    vec![Foo::A]
}
