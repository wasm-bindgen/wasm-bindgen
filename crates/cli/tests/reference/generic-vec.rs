// Built with `--cfg wasm_bindgen_unstable_generic_vec`, under which every
// sequence crosses by its element's own ABI: in place for numbers and
// handles, and otherwise as a buffer of each element's ABI.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Foo;

    fn take_strings(v: Vec<String>);
    fn give_strings() -> Vec<String>;
    fn take_foos(v: Vec<Foo>);
    fn give_foos() -> Vec<Foo>;
    fn take_opt_foos(v: Vec<Option<Foo>>);
    fn give_opt_foos() -> Vec<Option<Foo>>;
    fn take_opt_bytes(v: Vec<Option<u8>>);
    fn give_opt_bytes() -> Vec<Option<u8>>;
    fn take_bytes(v: Vec<u8>);
    fn give_bytes() -> Vec<u8>;
    fn take_borrowed_foos(v: &[Foo]);
    fn take_borrowed_strings(v: &[String]);
}

#[wasm_bindgen]
pub struct Point {
    pub x: u32,
}

#[wasm_bindgen]
pub fn strings(v: Vec<String>) -> Vec<String> {
    v
}

#[wasm_bindgen]
pub fn foos(v: Vec<Foo>) -> Vec<Foo> {
    v
}

#[wasm_bindgen]
pub fn opt_foos(v: Vec<Option<Foo>>) -> Vec<Option<Foo>> {
    v
}

#[wasm_bindgen]
pub fn opt_bytes(v: Vec<Option<u8>>) -> Vec<Option<u8>> {
    v
}

#[wasm_bindgen]
pub fn bytes(v: Vec<u8>) -> Vec<u8> {
    v
}

#[wasm_bindgen]
pub fn floats(v: Vec<f64>) -> Vec<f64> {
    v
}

#[wasm_bindgen]
pub fn values(v: Vec<JsValue>) -> Vec<JsValue> {
    v
}

#[wasm_bindgen]
pub fn points(v: Vec<Point>) -> Vec<Point> {
    v
}

#[wasm_bindgen]
pub fn wide(v: Vec<u128>) -> Vec<u128> {
    v
}

#[wasm_bindgen]
pub fn imports() {
    take_strings(give_strings());
    take_foos(give_foos());
    take_opt_foos(give_opt_foos());
    take_opt_bytes(give_opt_bytes());
    take_bytes(give_bytes());
    take_borrowed_foos(&give_foos());
    take_borrowed_strings(&give_strings());
}

#[wasm_bindgen]
pub fn nested(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    v
}

#[wasm_bindgen]
pub fn give_nested_strings() -> Vec<Vec<String>> {
    vec![vec!["a".to_string()]]
}

#[wasm_bindgen]
pub fn flags(v: Vec<bool>) -> Vec<bool> {
    v
}

#[wasm_bindgen]
pub fn narrow(a: u8, b: Option<i16>, c: bool, d: Option<bool>) -> Option<u8> {
    (c && d.is_some()).then_some(a.wrapping_add(b.unwrap_or(0) as u8))
}
