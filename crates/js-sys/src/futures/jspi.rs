//! JSPI (JS Promise Integration) runtime support for wasm-bindgen.
//!
//! The runtime primitive is [`block_on_promise`]: suspend the current WASM
//! fiber until a specific JavaScript [`Promise`] settles.
//!
//! It is the complete programming model. Promises are eager, so concurrency
//! is expressed at the promise level — start several calls, then suspend on
//! each (or on a `Promise::all` / `Promise::race` combination). A Rust
//! `Future` is awaited by scheduling it on the ordinary microtask executor
//! and suspending on its completion:
//! `block_on_promise(&future_to_promise(fut))`.
//!
//! The runtime state is minimal by construction. The single bridge import,
//! `__wbindgen_jspi_suspend`, is a plain `#[wasm_bindgen(catch, suspending)]`
//! import whose JS body is an identity function: JSPI itself awaits the
//! returned `Promise` and resumes the fiber with the settled value as the
//! import's `externref` return value, while a rejection is thrown into wasm
//! as a `WebAssembly.JSTag` exception and marshalled to `Err` by the
//! standard `catch + suspending` machinery (see
//! `cli-support/src/transforms/jspi.rs`), so no per-suspension JS-side state
//! exists at all.
//!
//! ## Usage
//!
//! Mark exports that call `block_on_promise` with `#[wasm_bindgen(jspi)]`:
//!
//! ```rust,ignore
//! use js_sys::futures::jspi::block_on_promise;
//!
//! #[wasm_bindgen(jspi)]
//! pub fn do_work() {
//!     let result = block_on_promise(&some_promise()).unwrap_throw();
//!     // ...
//! }
//! ```

// The `suspending` attribute on the internal bridge import generates an
// experimental-status deprecation warning; this module is already opted in
// via `js_sys_unstable_apis`, so silence it here.
#![allow(deprecated)]

use crate::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "__wbindgen_placeholder__")]
extern "C" {
    #[wasm_bindgen(catch, suspending)]
    fn __wbindgen_jspi_suspend(promise: &Promise) -> Result<JsValue, JsValue>;
}

// ─── Low-level primitive: suspend on a JS Promise ────────────────────────────

/// Suspend the current WASM fiber until `promise` settles.
///
/// Returns `Ok(value)` on fulfillment, `Err(reason)` on rejection.
///
/// **Must only be called from a WASM export wrapped with `WebAssembly.promising`**
/// (i.e. from a function marked `#[wasm_bindgen(jspi)]`).
pub fn block_on_promise(promise: &Promise) -> Result<JsValue, JsValue> {
    // On return the shadow stack is guaranteed to hold the correct contents
    // for this fiber, regardless of how many other fibers ran while this one
    // was suspended.  The wasm-bindgen CLI instruments every
    // `#[wasm_bindgen(suspending)]` import with an in-wasm wrapper that
    // evacuates the fiber's live shadow-stack region to a heap buffer before
    // suspending and copies it back — restoring `__stack_pointer` from a wasm
    // local, which JSPI preserves — as the very first instructions after the
    // fiber resumes (see `cli-support/src/transforms/jspi.rs`).  The `catch`
    // marshalling (rejections as `Err` data) rides the same wrapper.
    suspend(promise)
}

// `__wbindgen_jspi_suspend` must not be inlined into `block_on_promise` so that
// the two functions' wasm shadow-stack frames are distinct.  This is not
// load-bearing for correctness (the CLI's in-wasm suspending wrapper handles
// the shadow stack), but it keeps the generated wasm readable and the call
// graph unambiguous for future analysis.
#[inline(never)]
fn suspend(promise: &Promise) -> Result<JsValue, JsValue> {
    __wbindgen_jspi_suspend(promise)
}
