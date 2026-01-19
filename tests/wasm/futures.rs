use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[rustfmt::skip]
#[wasm_bindgen(module = "tests/wasm/futures.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn call_exports(catch_unwind: bool) -> Result<JsValue, JsValue>;

    async fn call_promise() -> JsValue;
    #[wasm_bindgen(js_name = call_promise)]
    async fn call_promise_string() -> JsString;
    #[wasm_bindgen(catch)]
    async fn call_promise_ok() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, js_name = call_promise_ok)]
    async fn call_promise_ok_string() -> Result<JsString, JsString>;
    #[wasm_bindgen(catch)]
    async fn call_promise_err() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, js_name = call_promise_err)]
    async fn call_promise_err_string() -> Result<JsString, JsString>;

    #[wasm_bindgen]
    async fn call_promise_unit();
    #[wasm_bindgen(catch)]
    async fn call_promise_ok_unit() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    async fn call_promise_err_unit() -> Result<(), JsValue>;

    #[wasm_bindgen]
    async fn check_panic();
}

#[wasm_bindgen_test]
async fn smoke() {
    call_exports(cfg!(all(feature = "std", panic = "unwind")))
        .await
        .unwrap();
}

#[wasm_bindgen]
pub async fn async_do_nothing() {}

#[wasm_bindgen]
pub async fn async_return_1() -> JsValue {
    1.into()
}

#[wasm_bindgen]
pub async fn async_return_2() -> u32 {
    2
}

#[wasm_bindgen]
pub async fn async_nothing_again() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub async fn async_return_3() -> Result<u32, JsValue> {
    Ok(3)
}

#[wasm_bindgen]
pub async fn async_return_4() -> Result<JsValue, JsValue> {
    Ok(4.into())
}

#[wasm_bindgen]
pub struct AsyncCustomReturn {
    pub val: u32,
}

#[wasm_bindgen]
pub async fn async_return_5() -> AsyncCustomReturn {
    AsyncCustomReturn { val: 5 }
}

#[wasm_bindgen]
pub async fn async_return_6() -> Result<AsyncCustomReturn, JsValue> {
    Ok(AsyncCustomReturn { val: 6 })
}

#[wasm_bindgen]
pub async fn async_throw_7() -> Result<AsyncCustomReturn, JsValue> {
    Err(7.into())
}

#[wasm_bindgen]
pub async fn async_throw_custom() -> Result<AsyncCustomReturn, JsValue> {
    Err(AsyncCustomReturn { val: 8 }.into())
}

#[wasm_bindgen]
pub async fn async_throw_message() -> Result<(), JsValue> {
    Err(js_sys::Error::new("async message").into())
}

#[wasm_bindgen]
pub async fn async_throw_jserror() -> Result<AsyncCustomReturn, JsError> {
    Err(JsError::new("async message"))
}

pub struct AsyncCustomError {
    pub val: JsValue,
}

impl From<AsyncCustomError> for JsValue {
    fn from(e: AsyncCustomError) -> Self {
        e.val
    }
}

#[wasm_bindgen]
pub async fn async_throw_custom_error() -> Result<AsyncCustomReturn, AsyncCustomError> {
    Err(AsyncCustomError {
        val: JsValue::from("custom error"),
    })
}

#[wasm_bindgen]
pub async fn async_take_reference(x: &str) -> String {
    format!("Hi, {x}!")
}

#[wasm_bindgen]
pub struct AsyncStruct;

#[wasm_bindgen]
impl AsyncStruct {
    #[wasm_bindgen(constructor)]
    #[allow(deprecated)]
    pub async fn new() -> AsyncStruct {
        AsyncStruct
    }

    pub async fn method(&self) -> u32 {
        42
    }
}

#[wasm_bindgen]
pub async fn async_take_js_reference(x: &JsValue) {
    assert_eq!(*x, 42);
}

// TODO: Not sure how to mark the slice argument as unwind safe while also
// preserving WasmAbi trait impl.
#[cfg(not(all(feature = "std", panic = "unwind")))]
#[wasm_bindgen]
pub async fn async_take_mut_slice(x: &mut [i32]) {
    x.fill(42);
}

#[wasm_bindgen_test]
async fn test_promise() {
    assert_eq!(call_promise().await.as_string(), Some(String::from("ok")))
}

#[wasm_bindgen_test]
async fn test_promise_string() {
    assert_eq!(String::from(call_promise_string().await), "ok")
}

#[wasm_bindgen_test]
async fn test_promise_ok() {
    assert_eq!(
        call_promise_ok().await.map(|j| j.as_string()),
        Ok(Some(String::from("ok")))
    )
}

#[wasm_bindgen_test]
async fn test_promise_ok_string() {
    assert_eq!(
        call_promise_ok_string().await.map(String::from),
        Ok(String::from("ok"))
    )
}

#[wasm_bindgen_test]
async fn test_promise_err() {
    assert_eq!(
        call_promise_err().await.map_err(|j| j.as_string()),
        Err(Some(String::from("error")))
    )
}

#[wasm_bindgen_test]
async fn test_promise_err_string() {
    assert_eq!(
        call_promise_err_string().await.map_err(String::from),
        Err(String::from("error"))
    )
}

#[wasm_bindgen_test]
async fn test_promise_unit() {
    call_promise_unit().await
}

#[wasm_bindgen_test]
async fn test_promise_ok_unit() {
    call_promise_ok_unit().await.unwrap()
}

#[wasm_bindgen_test]
async fn test_promise_err_unit() {
    assert_eq!(
        call_promise_err_unit().await.map_err(|j| j.as_string()),
        Err::<(), _>(Some(String::from("error")))
    )
}

#[wasm_bindgen]
pub async fn panics() -> u32 {
    call_promise().await;
    panic!("Oops!");
}

#[cfg(all(panic = "unwind", feature = "std"))]
#[wasm_bindgen_test]
async fn test_promise_panic() {
    check_panic().await;
}
