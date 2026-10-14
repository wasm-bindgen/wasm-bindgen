use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn fn_expects_number_vec(arg1: Vec<f32>) -> f32 {
    arg1.into_iter().sum()
}

#[wasm_bindgen]
pub fn fn_expects_number_slice(arg1: &[f64]) -> f64 {
    arg1.iter().sum()
}

#[wasm_bindgen]
pub fn fn_return_number_vec(arg1: u32) -> Vec<u32> {
    (0..arg1).collect()
}

#[wasm_bindgen]
pub fn fn_u8_to_f32(arg1: &[u8]) -> Vec<f32> {
    arg1.iter().map(|x| *x as f32).collect()
}

#[wasm_bindgen]
pub fn fn_fill_number_slice(arg1: &mut [f64]) {
    arg1.fill(1.0);
}

#[wasm_bindgen]
pub fn fn_expects_clamped_vec(arg1: Clamped<Vec<u8>>) -> usize {
    arg1.len()
}
