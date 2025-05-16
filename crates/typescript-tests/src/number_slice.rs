use wasm_bindgen::prelude::*;

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
