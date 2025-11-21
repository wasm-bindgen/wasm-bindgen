use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = math)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_namespace = math)]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen(js_namespace = "default")]
pub fn concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

#[wasm_bindgen(js_namespace = ["utils", "string"])]
pub fn uppercase(s: &str) -> String {
    s.to_uppercase()
}

#[wasm_bindgen(js_name = "uppercase", js_namespace = ["default", "uppercase"])]
pub fn default_uppercase(s: &str) -> String {
    s.to_uppercase()
}

#[wasm_bindgen(js_namespace = math, js_name = "divide")]
pub fn div(a: f64, b: f64) -> f64 {
    a / b
}

#[wasm_bindgen]
pub fn regular_function() -> i32 {
    42
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

#[wasm_bindgen(js_namespace = ["shapes", "http"])]
pub enum HttpShape {
    Blah = "Blah"
}

#[wasm_bindgen(js_namespace = models)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen(js_namespace = ["models", "3d"])]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen(js_namespace = "default")]
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
}

#[wasm_bindgen]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[wasm_bindgen]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
