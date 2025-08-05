use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fn_with_unused_attr1(
    #[wasm_bindgen(js_name = "firstArg", js_name = "anotherFirstArg")] arg: u32,
) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub async fn fn_with_unused_attr2(
    #[wasm_bindgen(unchecked_param_type = "number", unchecked_param_type = "bigint")] arg: u32,
) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub async fn fn_with_unused_attr3(
    #[wasm_bindgen(unchecked_return_type = "number")] arg: u32,
) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub async fn fn_with_unused_attr4(
    #[wasm_bindgen(return_description = "some description")] arg: u32,
) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen]
    pub async fn fn_with_unused_attr(
        #[wasm_bindgen(param_description = "some number")]
        #[wasm_bindgen(param_description = "some other description")]
        arg: u32,
    ) -> JsValue {
        arg.into()
    }
}

#[wasm_bindgen(
    unchecked_return_type = "something",
    return_description = "something",
    unchecked_param_type = "something",
    param_description = "somthing"
)]
struct B {}

#[wasm_bindgen(
    unchecked_return_type = "something",
    return_description = "something",
    unchecked_param_type = "something",
    param_description = "somthing"
)]
impl B {
    #[wasm_bindgen]
    pub fn foo() {}
}

#[wasm_bindgen(
    unchecked_return_type = "something",
    return_description = "something",
    unchecked_param_type = "something",
    param_description = "somthing"
)]
pub enum C {
    Variat,
}

#[wasm_bindgen(
    unchecked_return_type = "something",
    return_description = "something",
    unchecked_param_type = "something",
    param_description = "somthing"
)]
impl C {
    #[wasm_bindgen]
    pub fn foo() {}
}

fn main() {}
