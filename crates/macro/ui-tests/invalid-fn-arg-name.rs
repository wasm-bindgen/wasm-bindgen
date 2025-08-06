use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name1(#[wasm_bindgen(js_name = "*firstArg")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name2(#[wasm_bindgen(js_name = "#firstArg")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name3(#[wasm_bindgen(js_name = "firstArg#")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name4(#[wasm_bindgen(js_name = "first-Arg")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name5(#[wasm_bindgen(js_name = "--firstArg")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub fn fn_with_invalid_arg_name6(#[wasm_bindgen(js_name = " first Arg")] arg: u32) -> JsValue {
    arg.into()
}

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name1(
        #[wasm_bindgen(js_name = "(firstArg)")] arg: u32,
    ) -> JsValue {
        arg.into()
    }

    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name2(
        #[wasm_bindgen(js_name = "[firstArg]")] arg: u32,
    ) -> JsValue {
        arg.into()
    }

    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name3(
        #[wasm_bindgen(js_name = "<firstArg>")] arg: u32,
    ) -> JsValue {
        arg.into()
    }

    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name4(
        #[wasm_bindgen(js_name = "firstArg+")] arg: u32,
    ) -> JsValue {
        arg.into()
    }

    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name5(
        #[wasm_bindgen(js_name = "@firstArg")] arg: u32,
    ) -> JsValue {
        arg.into()
    }

    #[wasm_bindgen]
    pub async fn method_with_invalid_arg_name6(
        #[wasm_bindgen(js_name = "!firstArg")] arg: u32,
    ) -> JsValue {
        arg.into()
    }
}

fn main() {}
