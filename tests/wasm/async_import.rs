//! Runtime coverage for `async` imports whose resolved type is not
//! handle-shaped. The descriptor must name the `Promise` handle that crosses
//! the ABI, not the resolved type; the pre-existing async-import tests in
//! `futures.rs` all resolve to `JsValue`/`JsString`/`()`, which marshal the
//! same either way and so cannot observe a mismatch. Assertions here are on
//! concrete values for the same reason.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/async_import.js")]
extern "C" {
    #[wasm_bindgen(js_name = asyncU32)]
    async fn async_u32() -> u32;
    #[wasm_bindgen(js_name = asyncI32)]
    async fn async_i32() -> i32;
    #[wasm_bindgen(js_name = asyncF64)]
    async fn async_f64() -> f64;
    #[wasm_bindgen(js_name = asyncBool)]
    async fn async_bool() -> bool;
    #[wasm_bindgen(js_name = asyncString)]
    async fn async_string() -> String;
    #[wasm_bindgen(js_name = asyncOptSome)]
    async fn async_opt_some() -> Option<u32>;
    #[wasm_bindgen(js_name = asyncOptNone)]
    async fn async_opt_none() -> Option<u32>;
    #[wasm_bindgen(catch, js_name = asyncU32Throws)]
    async fn async_u32_throws() -> Result<u32, JsValue>;
    #[wasm_bindgen(js_name = asyncEchoU32)]
    async fn async_echo_u32(x: u32) -> u32;
}

#[wasm_bindgen_test]
async fn async_import_returns_scalars() {
    // `u32::MAX` rather than a small value: a small integer can coincide with a
    // plausible externref index, whereas this cannot.
    assert_eq!(async_u32().await, u32::MAX);
    assert_eq!(async_i32().await, i32::MIN);
    assert_eq!(async_f64().await, 1.5);
    assert!(async_bool().await);
}

#[wasm_bindgen_test]
async fn async_import_returns_string() {
    assert_eq!(async_string().await, "hello");
}

#[wasm_bindgen_test]
async fn async_import_returns_option() {
    assert_eq!(async_opt_some().await, Some(7));
    assert_eq!(async_opt_none().await, None);
}

#[wasm_bindgen_test]
async fn async_import_catch_with_scalar_ok_type() {
    let err = async_u32_throws().await.unwrap_err();
    assert!(
        err.as_string().unwrap_or_default().contains("boom") || format!("{err:?}").contains("boom"),
        "unexpected error value: {err:?}"
    );
}

#[wasm_bindgen_test]
async fn async_import_takes_and_returns_scalar() {
    // The JS side rejects a non-number argument, covering the argument
    // direction as well.
    assert_eq!(async_echo_u32(12345).await, 12345);
}
