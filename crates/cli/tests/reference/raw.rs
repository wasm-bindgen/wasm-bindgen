use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn r#test1(r#test: u32) -> u32 {
    r#test2();
    let r#other = r#Other::new();
    r#other.r#do();
    r#test
}

#[wasm_bindgen]
pub struct r#Test;

#[wasm_bindgen]
impl r#Test {
    pub fn r#test1(r#test: u32) -> Self {
        Self
    }

    pub fn r#test2(&self, r#test: u32) {}
}

#[wasm_bindgen(module = "test")]
extern "C" {
    fn r#test2() -> JsValue;
}

#[wasm_bindgen(module = "other")]
extern "C" {
    pub type r#Other;

    #[wasm_bindgen(constructor)]
    fn new() -> r#Other;

    #[wasm_bindgen(method)]
    fn r#do(this: &r#Other);
}

#[wasm_bindgen]
pub enum r#Enum {
    r#A,
    r#B,
}
