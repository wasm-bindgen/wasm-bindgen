// An `async` import returns a `Promise` handle at the ABI regardless of its
// resolved type (see `DescribeImport` in macro-support), so every import below
// must show `(result externref)` in the `.wat` and the JS shim must simply
// forward the promise.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Widget;

    // Non-handle resolved types: the regression cases.
    async fn async_number() -> u32;
    async fn async_string() -> String;

    // Handle-shaped resolved type: marshals the same either way.
    async fn async_handle() -> Widget;

    // No declared return type still yields a promise at the ABI.
    async fn async_unit();

    // `catch`: the `Result` comes from awaiting the promise, so the ABI
    // return is still just the promise handle.
    #[wasm_bindgen(catch)]
    async fn async_catch() -> Result<u32, JsValue>;
}

#[wasm_bindgen]
pub async fn exported() -> Result<(), JsValue> {
    let _ = async_number().await;
    let _ = async_string().await;
    let _ = async_handle().await;
    async_unit().await;
    let _ = async_catch().await?;
    Ok(())
}
