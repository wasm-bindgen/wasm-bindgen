//! JSPI (JS Promise Integration) runtime support for wasm-bindgen.
//!
//! This module provides two primitives:
//!
//! - [`block_on_promise`] — suspends a WASM fiber until a specific JavaScript
//!   [`Promise`] settles (low-level).
//! - [`block_on`] — drives an arbitrary `async` Rust [`Future`] to completion
//!   inside a JSPI fiber, using a JS-Promise-backed waker (high-level).
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
//! Mark exports that call `block_on` or `block_on_promise` with
//! `#[wasm_bindgen(jspi)]`:
//!
//! ```rust,ignore
//! use js_sys::futures::jspi::block_on;
//!
//! #[wasm_bindgen(jspi)]
//! pub fn do_work() {
//!     let result = block_on(some_async_fn()).unwrap_throw();
//!     // ...
//! }
//! ```

// The `suspending` attribute on the internal bridge import generates an
// experimental-status deprecation warning; this module is already opted in
// via `js_sys_unstable_apis`, so silence it here.
#![allow(deprecated)]

use crate::{Function, Promise};
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::future::Future;
use core::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

// ─── Waker ───────────────────────────────────────────────────────────────────

/// A waker backed by the `resolve` function of a JS `Promise` the fiber
/// suspends on: waking resolves the promise, which resumes the fiber.
struct JspiWaker {
    resolve: Function,
}

// SAFETY: `Waker::from(Arc<W>)` requires `Send + Sync`, but JSPI fibers are
// single-threaded — the waker is only ever created, woken, and dropped on
// the one wasm thread (JSPI is not supported with threads).
unsafe impl Send for JspiWaker {}
unsafe impl Sync for JspiWaker {}

impl alloc::task::Wake for JspiWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = self.resolve.call0(&JsValue::UNDEFINED);
    }
}

// ─── High-level primitive: drive a Rust Future ───────────────────────────────

/// Drive `fut` to completion inside a JSPI fiber.
///
/// Each time the future returns [`Poll::Pending`], the fiber suspends on a
/// fresh JS `Promise` whose `resolve` function backs the waker. The promise
/// (and its resolver) is created *before* polling, so a `wake()` that fires
/// synchronously during the poll has already resolved the promise by the
/// time the fiber suspends on it, and the fiber resumes on the next
/// microtask tick.
///
/// Nested calls are safe: every poll iteration owns its own promise/resolver
/// pair, held as ordinary Rust values.
///
/// **Must only be called from a function marked `#[wasm_bindgen(jspi)]`.**
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = Box::pin(fut);

    loop {
        // Create the promise/resolver pair before polling so that a
        // synchronous wake() during poll resolves a live promise.
        let mut resolve_slot: Option<Function> = None;
        let promise: Promise = Promise::new(&mut |resolve, _reject| {
            resolve_slot = Some(resolve.unchecked_into());
        });
        let resolve = resolve_slot.expect_throw("Promise executor did not run synchronously");
        let waker: Waker = Arc::new(JspiWaker { resolve }).into();
        let mut cx = Context::from_waker(&waker);

        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // Ignore the resolved value — we only care about being woken.
                let _ = block_on_promise(&promise);
            }
        }
    }
}
