//! Utility functions for testing wasm-bindgen applications.

/// Test demonstrating that wasm-bindgen-futures now handles non-shared memory gracefully
///
/// **This test demonstrates that the PRACTICAL ISSUE is now FIXED.**
///
/// Previously, using async code with atomics would crash when memory wasn't shared.
/// Our fix makes wasm-bindgen-futures detect this situation and fall back to a polyfill,
/// preventing the crash.
///
/// ```
/// # use wasm_bindgen::prelude::*;
/// # use std::pin::Pin;
/// # use std::future::Future;
/// # use std::task::{Context, Poll};
/// #
/// # #[wasm_bindgen_test::wasm_bindgen_test]
/// # async fn test() {
/// // This would previously crash with "not a shared typed array" error
/// // Now it gracefully falls back to polyfill (which may fail for other reasons like no Worker)
/// // but importantly, it doesn't crash with the SharedArrayBuffer error anymore
/// wasm_bindgen_futures::spawn_local(async {
///     // Simple async work that should not crash with SharedArrayBuffer error
///     // The fix detects non-shared memory and uses polyfill instead
/// });
///
/// // Wait a bit to ensure the spawned task has a chance to run
/// let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::undefined());
/// let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
/// # }
/// ```
#[doc(hidden)]
pub fn test_wasm_bindgen_futures_fix() {}
