use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/async_abi_returns.js")]
extern "C" {
    async fn js_async_abi_returns();
}

#[wasm_bindgen]
pub async fn async_char() -> char {
    'é'
}

#[wasm_bindgen]
pub async fn async_option_char(some: bool) -> Option<char> {
    some.then_some('x')
}

#[wasm_bindgen]
pub async fn async_nullable(some: bool) -> Nullable<u32> {
    if some {
        Nullable::Value(7)
    } else {
        Nullable::Null
    }
}

#[wasm_bindgen]
pub async fn async_result_char(ok: bool) -> Result<char, JsError> {
    if ok {
        Ok('y')
    } else {
        Err(JsError::new("no char"))
    }
}

pub struct IntoOnlyError(&'static str);

impl From<IntoOnlyError> for JsValue {
    fn from(e: IntoOnlyError) -> JsValue {
        JsValue::from_str(e.0)
    }
}

#[wasm_bindgen]
pub async fn async_into_only_error(ok: bool) -> Result<u32, IntoOnlyError> {
    if ok {
        Ok(3)
    } else {
        Err(IntoOnlyError("into only"))
    }
}

#[wasm_bindgen]
pub async fn async_unit_error(ok: bool) -> Result<(), IntoOnlyError> {
    if ok {
        Ok(())
    } else {
        Err(IntoOnlyError("unit"))
    }
}

#[wasm_bindgen_test]
async fn works() {
    js_async_abi_returns().await;
}
