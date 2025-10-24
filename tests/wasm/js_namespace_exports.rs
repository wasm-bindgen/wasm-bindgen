use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(js_namespace = api)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_namespace = api)]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen(js_namespace = ["utils", "math"])]
pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}

#[wasm_bindgen(js_namespace = ["utils", "math"], js_name = "subtract")]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[wasm_bindgen(js_namespace = models)]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> Counter {
        Counter { value: initial }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn add(&mut self, amount: i32) {
        self.value += amount;
    }
}

#[wasm_bindgen(js_namespace = types)]
pub enum Status {
    Pending = 0,
    Active = 1,
    Complete = 2,
}

#[wasm_bindgen(js_namespace = ["types", "http"])]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

#[wasm_bindgen(js_namespace = shapes)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen(js_namespace = ["shapes", "3d"])]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen(module = "tests/wasm/js_namespace_exports.js")]
extern "C" {
    fn test_api_namespace();
    fn test_nested_namespace();
    fn test_class_namespace();
    fn test_enum_namespace();
    fn test_nested_enum_namespace();
    fn test_struct_namespace();
    fn test_nested_struct_namespace();
}

#[wasm_bindgen_test]
fn test_namespaced_exports() {
    test_api_namespace();
    test_nested_namespace();
    test_class_namespace();
    test_enum_namespace();
    test_nested_enum_namespace();
    test_struct_namespace();
    test_nested_struct_namespace();
}
