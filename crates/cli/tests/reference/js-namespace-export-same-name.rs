use wasm_bindgen::prelude::*;

/// Two structs with the same js_name in different namespaces should not collide.

#[wasm_bindgen(js_namespace = foo, js_name = "Point")]
pub struct FooPoint {
    pub x: f64,
}

#[wasm_bindgen]
impl FooPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64) -> FooPoint {
        FooPoint { x }
    }
}

#[wasm_bindgen(js_namespace = bar, js_name = "Point")]
pub struct BarPoint {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl BarPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> BarPoint {
        BarPoint { x, y }
    }
}

/// Two enums with the same js_name in different namespaces should not collide.

#[wasm_bindgen(js_namespace = foo, js_name = "Status")]
pub enum FooStatus {
    Active = 0,
    Inactive = 1,
}

#[wasm_bindgen(js_namespace = bar, js_name = "Status")]
pub enum BarStatus {
    Pending = 0,
    Complete = 1,
    Failed = 2,
}

/// Two functions with the same js_name in different namespaces should not collide.

#[wasm_bindgen(js_namespace = foo, js_name = "greet")]
pub fn foo_greet() -> String {
    "hello from foo".to_string()
}

#[wasm_bindgen(js_namespace = bar, js_name = "greet")]
pub fn bar_greet() -> String {
    "hello from bar".to_string()
}
