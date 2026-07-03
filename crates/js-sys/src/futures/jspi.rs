//! JSPI (JS Promise Integration) runtime support for wasm-bindgen.
//!
//! This module provides two primitives:
//!
//! - [`block_on_promise`] — suspends a WASM fiber until a specific JavaScript
//!   [`Promise`] settles (low-level).
//! - [`block_on`] — drives an arbitrary `async` Rust [`Future`] to completion
//!   inside a JSPI fiber, using a JS-Promise-backed waker (high-level).
//!
//! The bridge functions are wasm-bindgen intrinsics emitted directly into the
//! generated glue, so no manual setup is required and every target is
//! supported (including `--target no-modules`).
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
// These are wasm-bindgen intrinsics generated directly into the main JS glue
// by the CLI (see `crates/cli-support/src/intrinsic.rs`).  They share the
// module-scoped `_jspiPending`/`_jspiResolved`/`_jspiRejected` arrays and the
// `_jspiWakerMap` emitted once by `Context::expose_jspi_bridge`.
//
// Using intrinsics rather than an `inline_js` snippet means the JSPI runtime
// works with every target, including `--target no-modules`, which cannot
// import from `./snippets/...`.

#[wasm_bindgen(raw_module = "__wbindgen_placeholder__")]
extern "C" {
    fn __wbindgen_jspi_set_pending(id: u32, promise: &Promise);
    #[wasm_bindgen(suspending)]
    fn __wbindgen_jspi_suspend(id: u32);
    fn __wbindgen_jspi_is_rejected(id: u32) -> bool;
    fn __wbindgen_jspi_get_resolved(id: u32) -> JsValue;
    fn __wbindgen_jspi_cleanup(id: u32);
    fn __wbindgen_jspi_waker_create(id: u32) -> Promise;
    fn __wbindgen_jspi_waker_wake(id: u32);
    fn __wbindgen_jspi_waker_cleanup(id: u32);
}

// ─── Growable ID pool ─────────────────────────────────────────────────────────

struct IdPool {
    free: Vec<u32>,
    next: u32,
}

impl IdPool {
    const fn new() -> Self {
        Self {
            free: Vec::new(),
            next: 0,
        }
    }

    fn alloc(&mut self) -> u32 {
        self.free.pop().unwrap_or_else(|| {
            let id = self.next;
            self.next += 1;
            id
        })
    }

    fn release(&mut self, id: u32) {
        self.free.push(id);
    }
}

#[cfg_attr(target_feature = "atomics", thread_local)]
static SUSPEND_IDS: ThreadLocalWrapper<RefCell<IdPool>> =
    ThreadLocalWrapper(RefCell::new(IdPool::new()));

fn alloc_id() -> u32 {
    SUSPEND_IDS.0.borrow_mut().alloc()
}

fn release_id(id: u32) {
    SUSPEND_IDS.0.borrow_mut().release(id);
}

#[cfg_attr(target_feature = "atomics", thread_local)]
static WAKER_IDS: ThreadLocalWrapper<RefCell<IdPool>> =
    ThreadLocalWrapper(RefCell::new(IdPool::new()));

fn alloc_waker_id() -> u32 {
    WAKER_IDS.0.borrow_mut().alloc()
}

fn release_waker_id(id: u32) {
    WAKER_IDS.0.borrow_mut().release(id);
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
    __wbindgen_jspi_set_pending(id, promise);
    suspend(id);
    // At this point `__stack_pointer` is guaranteed to hold the correct value
    // for this fiber, regardless of how many other fibers ran while this one
    // was suspended.  The invariant is maintained entirely in JS, not by any
    // Rust/LLVM trick.  The execution sequence is:
    //
    //   1. `suspend(id)` calls `__wbindgen_jspi_suspend(id)`.
    //   2. The CLI-generated JS for that import (all `#[wasm_bindgen(suspending)]`
    //      imports are wrapped) runs:
    //
    //        async function(...args) {
    //            const __sp = wasm.__stack_pointer.value;   // ← save
    //            try { return await __inner(...args); }      // ← fiber suspends
    //            finally { wasm.__stack_pointer.value = __sp; } // ← restore
    //        }
    //
    //   3. The `finally` block executes on the JS microtask queue BEFORE the
    //      `WebAssembly.Suspending` mechanism delivers the resolved value back
    //      to wasm.  The fiber does not run a single wasm instruction between
    //      the `finally` restore and the next Rust statement here.
    //
    //   4. Therefore, everything that follows — `__wbindgen_jspi_is_rejected`, `release_id`
    //      (which calls `Vec::push` and may trigger `malloc`), and any call in
    //      the user's code after `block_on_promise` returns — all execute with
    //      the correct stack pointer, even after deep recursion or concurrent
    //      fiber interleaving.
    let rejected = __wbindgen_jspi_is_rejected(id);
    let result = __wbindgen_jspi_get_resolved(id);
    __wbindgen_jspi_cleanup(id);
    release_id(id); // Vec::push — allocates with correct SP
    if rejected {
        Err(result)
    } else {
        Ok(result)
    }
}

// `__wbindgen_jspi_suspend` must not be inlined into `block_on_promise` so that
// the two functions' wasm shadow-stack frames are distinct.  This is not
// load-bearing for SP correctness (the JS wrapper handles that), but it
// keeps the generated wasm readable and the call graph unambiguous for
// future analysis.
#[inline(never)]
fn suspend(id: u32) {
    __wbindgen_jspi_suspend(id);
}

// ─── Waker ───────────────────────────────────────────────────────────────────

struct JspiWaker {
    id: u32,
}

impl alloc::task::Wake for JspiWaker {
    fn wake(self: Arc<Self>) {
        __wbindgen_jspi_waker_wake(self.id);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        __wbindgen_jspi_waker_wake(self.id);
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
        let promise = __wbindgen_jspi_waker_create(waker_id);
        let mut cx = Context::from_waker(&waker);

        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(val) => {
                __wbindgen_jspi_waker_cleanup(waker_id);
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
