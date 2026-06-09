//! JSPI (JS Promise Integration) runtime support for wasm-bindgen.
//!
//! This module provides two primitives:
//!
//! - [`block_on_promise`] — suspends a WASM fiber until a specific JavaScript
//!   [`Promise`] settles (low-level).
//! - [`block_on`] — drives an arbitrary `async` Rust [`Future`] to completion
//!   inside a JSPI fiber, using a JS-Promise-backed waker (high-level).
//!
//! All bridge functions are bundled as inline JS, so no manual setup is
//! required when using the CLI-generated glue.
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

use crate::Promise;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::future::Future;
use core::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::*;

// Copy `ThreadLocalWrapper` impl
struct ThreadLocalWrapper<T>(T);

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Sync for ThreadLocalWrapper<T> {}

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Send for ThreadLocalWrapper<T> {}

// ─── JS bridge ───────────────────────────────────────────────────────────────
//
// All six functions share the same inline_js module so they can share the
// _jspiPending/_jspiResolved arrays and _jspiWakerMap without any extra
// coupling between Rust and JS.

#[wasm_bindgen(inline_js = "\
const _jspiPending  = new Array(256).fill(null);\n\
const _jspiResolved = new Array(256).fill(null);\n\
export function jspi_set_pending(id, promise)  { _jspiPending[id]  = promise; }\n\
export async function jspi_do_suspend(id) {\n\
    try { _jspiResolved[id] = await _jspiPending[id]; }\n\
    catch(e) { _jspiResolved[id] = e instanceof Error ? e : new Error(String(e)); }\n\
}\n\
export function jspi_get_resolved(id)          { return _jspiResolved[id]; }\n\
export function jspi_cleanup(id)               { _jspiPending[id] = _jspiResolved[id] = null; }\n\
const _jspiWakerMap = new Map();\n\
export function jspi_waker_create(id) {\n\
    return new Promise(resolve => _jspiWakerMap.set(id, resolve));\n\
}\n\
export function jspi_waker_wake(id) {\n\
    const resolve = _jspiWakerMap.get(id);\n\
    if (resolve) resolve();\n\
}\n\
export function jspi_waker_cleanup(id) { _jspiWakerMap.delete(id); }\n\
")]
extern "C" {
    fn jspi_set_pending(id: u32, promise: &Promise);
    #[wasm_bindgen(suspending)]
    fn jspi_do_suspend(id: u32);
    fn jspi_get_resolved(id: u32) -> JsValue;
    fn jspi_cleanup(id: u32);
    fn jspi_waker_create(id: u32) -> Promise;
    fn jspi_waker_wake(id: u32);
    fn jspi_waker_cleanup(id: u32);
}

// ─── Suspension ID pool (up to 256 concurrent suspensions) ───────────────────

#[cfg_attr(target_feature = "atomics", thread_local)]
static FREE_IDS: ThreadLocalWrapper<RefCell<Option<Vec<u32>>>> =
    ThreadLocalWrapper(RefCell::new(None));

fn alloc_id() -> u32 {
    FREE_IDS
        .0
        .borrow_mut()
        .get_or_insert_with(|| (0u32..256).rev().collect())
        .pop()
        .expect_throw("exceeded 256 concurrent JSPI suspensions")
}

fn release_id(id: u32) {
    FREE_IDS.0.borrow_mut().as_mut().unwrap_throw().push(id);
}

// ─── Waker ID pool (up to 256 concurrent futures) ────────────────────────────

#[cfg_attr(target_feature = "atomics", thread_local)]
static FREE_WAKER_IDS: ThreadLocalWrapper<RefCell<Option<Vec<u32>>>> =
    ThreadLocalWrapper(RefCell::new(None));

fn alloc_waker_id() -> u32 {
    FREE_WAKER_IDS
        .0
        .borrow_mut()
        .get_or_insert_with(|| (0u32..256).rev().collect())
        .pop()
        .expect_throw("exceeded 256 concurrent JSPI futures")
}

fn release_waker_id(id: u32) {
    FREE_WAKER_IDS
        .0
        .borrow_mut()
        .as_mut()
        .unwrap_throw()
        .push(id);
}

// ─── Low-level primitive: suspend on a JS Promise ────────────────────────────

/// Suspend the current WASM fiber until `promise` settles.
///
/// Returns `Ok(value)` on fulfillment, `Err(reason)` on rejection.
///
/// **Must only be called from a WASM export wrapped with `WebAssembly.promising`**
/// (i.e. from a function marked `#[wasm_bindgen(jspi)]`).
pub fn block_on_promise(promise: &Promise) -> Result<JsValue, JsValue> {
    let id = alloc_id();
    jspi_set_pending(id, promise);
    // #[inline(never)] so the function prologue/epilogue saves/restores
    // __stack_pointer, which JSPI does not preserve across fiber switches.
    suspend(id);
    let result = jspi_get_resolved(id);
    jspi_cleanup(id);
    release_id(id);
    if result.is_instance_of::<crate::Error>() {
        Err(result)
    } else {
        Ok(result)
    }
}

#[inline(never)]
fn suspend(id: u32) {
    jspi_do_suspend(id);
}

// ─── Waker ───────────────────────────────────────────────────────────────────

struct JspiWaker {
    id: u32,
}

impl alloc::task::Wake for JspiWaker {
    fn wake(self: Arc<Self>) {
        jspi_waker_wake(self.id);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        jspi_waker_wake(self.id);
    }
}

// ─── High-level primitive: drive a Rust Future ───────────────────────────────

/// Drive `fut` to completion inside a JSPI fiber.
///
/// Each time the future returns [`Poll::Pending`], a fresh JS `Promise` is
/// pre-created for the waker before polling so that if the waker fires
/// *during* the poll (before `Pending` is returned), the Promise is already
/// resolved and `block_on_promise` returns on the next microtask tick.
///
/// Nested calls are safe: each invocation gets its own unique `waker_id` and
/// its own suspension `id`.
///
/// **Must only be called from a function marked `#[wasm_bindgen(jspi)]`.**
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = Box::pin(fut);

    let waker_id = alloc_waker_id();
    let waker: Waker = Arc::new(JspiWaker { id: waker_id }).into();

    loop {
        // Pre-create the waker Promise before polling so that a synchronous
        // wake() call during poll sees a valid resolver in _jspiWakerMap.
        let promise = jspi_waker_create(waker_id);
        let mut cx = Context::from_waker(&waker);

        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(val) => {
                jspi_waker_cleanup(waker_id);
                release_waker_id(waker_id);
                return val;
            }
            Poll::Pending => {
                // Ignore the resolved value — we only care about being woken.
                let _ = block_on_promise(&promise);
            }
        }
    }
}
