use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
pub struct OptRefTestStruct {
    value: u32,
}

#[wasm_bindgen]
impl OptRefTestStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> OptRefTestStruct {
        OptRefTestStruct { value }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[wasm_bindgen(module = "tests/wasm/option_ref_struct.js")]
extern "C" {
    fn js_call_rust_with_some() -> u32;
    fn js_call_rust_with_none() -> u32;
}

// Test receiving Option<&OptRefTestStruct> from JS
#[wasm_bindgen]
pub fn rust_receive_option_ref(value: Option<&OptRefTestStruct>) -> u32 {
    match value {
        Some(s) => s.value,
        None => 0,
    }
}

#[wasm_bindgen_test]
fn test_option_ref_some() {
    let result = js_call_rust_with_some();
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn test_option_ref_none() {
    let result = js_call_rust_with_none();
    assert_eq!(result, 0);
}
