use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {
    inner: u32,
}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen]
    pub fn method_with_self_attr1(
        #[wasm_bindgen(unchecked_param_type = "number")] &self,
        arg: u32,
    ) -> JsValue {
        (self.inner + arg).into()
    }

    #[wasm_bindgen]
    pub fn method_with_self_attr2(
        #[wasm_bindgen(param_description = "some description")] &self,
        arg: u32,
    ) -> JsValue {
        (self.inner + arg).into()
    }
}

fn main() {}
