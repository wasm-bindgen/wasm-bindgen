// FLAGS: --debug

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn take_slice(a: &[u8]) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_vec(a: Vec<f64>) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_boxed_slice(a: Box<[i64]>) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_clamped(a: Clamped<Vec<u8>>) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_optional_vec(a: Option<Vec<u16>>) {
    drop(a);
}

#[wasm_bindgen]
pub fn take_mut_slice(a: &mut [f32]) {
    a.fill(0.0);
}

#[wasm_bindgen]
pub fn take_jsvalue_vec(a: Vec<JsValue>) {
    drop(a);
}

#[wasm_bindgen]
extern "C" {
    fn get_bytes() -> Vec<u8>;
}

#[wasm_bindgen]
pub fn call_get_bytes() -> usize {
    get_bytes().len()
}
