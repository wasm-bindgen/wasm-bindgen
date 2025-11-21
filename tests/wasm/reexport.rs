use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(reexport, js_name = "console")]
    pub type Console;

    #[wasm_bindgen(method)]
    fn log(this: &Console, s: &str);
}

#[wasm_bindgen(module = "tests/wasm/reexport.js")]
extern "C" {
    #[wasm_bindgen(thread_local_v2, reexport = "PI_VALUE")]
    static PI: f64;
}

#[wasm_bindgen(module = "tests/wasm/reexport.js")]
extern "C" {
    #[wasm_bindgen(reexport = "customAdd")]
    fn add(a: i32, b: i32) -> i32;

    #[wasm_bindgen(reexport)]
    fn multiply(a: i32, b: i32) -> i32;
}

pub enum Status {
    Active,
    Inactive,
}

#[wasm_bindgen_test]
fn test_reexports() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(multiply(4, 5), 20);
    PI.with(|pi| assert!((*pi - 3.14159).abs() < 0.001));
}
