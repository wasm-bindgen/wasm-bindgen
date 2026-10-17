use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn import_param(#[cfg_attr(all(), wasm_bindgen(slice_to_array))] _x: &[f64]);
}

#[wasm_bindgen]
extern "C" {
    #[cfg_attr(all(), wasm_bindgen(js_name = renamed))]
    pub fn import_item();
}

#[wasm_bindgen]
pub fn export_param(#[cfg_attr(all(), wasm_bindgen(unchecked_param_type = "number"))] _x: u32) {}

#[wasm_bindgen]
pub struct A;

#[wasm_bindgen]
impl A {
    pub fn method_param(
        #[cfg_attr(all(), wasm_bindgen(unchecked_param_type = "number"))] _x: u32,
    ) {
    }
}

fn main() {}
