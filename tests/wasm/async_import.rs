//! Runtime coverage for ordinary (non-generic) `async` imports whose resolved
//! type is *not* handle-shaped.
//!
//! An `async` import hands back a `Promise` handle — an externref — regardless of
//! what it resolves to, so that is what its descriptor has to say. When the
//! descriptor instead named the resolved type, cli-support marshalled the promise
//! handle as if it were that type: `async fn f() -> u32` read the handle index as
//! the integer, silently producing garbage.
//!
//! Every pre-existing async-import test in `futures.rs` resolves to `JsValue`,
//! `JsString` or `()`. Those are handle-shaped or empty, so they cannot observe
//! the bug — which is exactly why it survived. The assertions here are on
//! concrete values for that reason: asserting merely that the future resolved
//! would pass against the broken descriptor too.
//!
//! The reference test `crates/cli/tests/reference/async-import.rs` pins the
//! *shape* of the emitted glue; this pins the values that actually cross.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/async_import.js")]
extern "C" {
    async fn asyncU32() -> u32;
    async fn asyncI32() -> i32;
    async fn asyncF64() -> f64;
    async fn asyncBool() -> bool;
    async fn asyncString() -> String;
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
    assert_eq!(asyncU32().await, u32::MAX);
    assert_eq!(asyncI32().await, i32::MIN);
    assert_eq!(asyncF64().await, 1.5);
    assert!(asyncBool().await);
}

#[wasm_bindgen_test]
async fn async_import_returns_string() {
    assert_eq!(asyncString().await, "hello");
}

#[wasm_bindgen_test]
async fn async_import_returns_option() {
    assert_eq!(async_opt_some().await, Some(7));
    assert_eq!(async_opt_none().await, None);
}

#[wasm_bindgen_test]
async fn async_import_catch_with_scalar_ok_type() {
    // The `catch` + non-handle-`Ok` combination is worth its own case: the
    // `Result` is unwrapped from the promise rather than from a synchronous
    // return, so it goes through `JsFuture` rather than `__wbindgen_exn_store`.
    let err = async_u32_throws().await.unwrap_err();
    assert!(
        err.as_string().unwrap_or_default().contains("boom") || format!("{err:?}").contains("boom"),
        "unexpected error value: {err:?}"
    );
}

#[wasm_bindgen_test]
async fn async_import_takes_and_returns_scalar() {
    // Covers the argument direction too; the JS side rejects a non-number, so a
    // boxed argument fails loudly rather than silently round-tripping.
    assert_eq!(async_echo_u32(12345).await, 12345);
}
