use js_sys::Number;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen(module = "tests/wasm/js_vec.js")]
extern "C" {
    fn pass_array_with_allocation();
}

#[wasm_bindgen]
pub fn test_sum(param: Vec<Number>) -> f64 {
    let mut sum = 0.;

    for data in param {
        sum += data.value_of();
    }

    sum
}

#[wasm_bindgen_test]
fn test() {
    pass_array_with_allocation();
}
