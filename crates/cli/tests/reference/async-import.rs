// Pins the descriptor/binding shape of `async` *imports* on the ordinary
// (non-`generic_per_mono`) path.
//
// What actually crosses the ABI for an `async` import is the `Promise` handle —
// an externref — never the resolved value. The resolved value is produced later,
// on the Rust side, by awaiting `JsFuture`. So every import below must show the
// same `(result externref)` in the `.wat` regardless of what it resolves to, and
// the JS shim must simply forward the promise.
//
// Before this was fixed, the descriptor named the *resolved* type instead, so
// cli-support marshalled the promise handle as if it were e.g. a `u32` and
// silently produced garbage for any resolved type that is not itself
// handle-shaped. `async_number` and `async_string` are the regression tests for
// that; `async_handle` and `async_unit` are the cases that worked either way and
// are here to prove they did not change.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Widget;

    // The regression: a non-handle resolved type. Must be `(result externref)`.
    async fn async_number() -> u32;

    // Another non-handle resolved type, this one heap-allocated.
    async fn async_string() -> String;

    // Handle-shaped resolved type. This one was already correct, because the
    // resolved type and the promise handle marshal identically.
    async fn async_handle() -> Widget;

    // No declared return type still yields a promise at the ABI.
    async fn async_unit();

    // `catch` composes with the above: the `Result` is produced by awaiting the
    // promise, so the ABI return is still just the promise handle.
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
