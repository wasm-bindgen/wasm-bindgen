use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/extends_export.js")]
extern "C" {
    pub type RpcTarget;

    #[wasm_bindgen(constructor)]
    fn new() -> RpcTarget;

    #[wasm_bindgen(method)]
    fn base_method(this: &RpcTarget) -> String;

    pub type AnotherBase;
}

#[wasm_bindgen(extends = RpcTarget)]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn increment(&mut self, amount: i32) -> i32 {
        self.value += amount;
        self.value
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[wasm_bindgen]
pub struct SimpleStruct {
    data: String,
}

#[wasm_bindgen]
impl SimpleStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(data: String) -> SimpleStruct {
        SimpleStruct { data }
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}

#[wasm_bindgen(extends = RpcTarget)]
pub struct WithExtendsNoConstructor {
    name: String,
}

#[wasm_bindgen]
impl WithExtendsNoConstructor {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen_test]
fn test_into_works() {
    let counter = Counter { value: 42 };

    // Convert Counter into RpcTarget and call base method
    let rpc_target: RpcTarget = counter.into();
    let _base_result = rpc_target.base_method();
}

#[wasm_bindgen_test]
fn test_extends_inheritance() {
    let counter = Counter { value: 10 };
    assert_eq!(counter.get_value(), 10);
}

#[wasm_bindgen_test]
fn test_simple_constructor_still_works() {
    let simple = SimpleStruct::new("test".to_string());
    assert_eq!(simple.get_data(), "test");
}
