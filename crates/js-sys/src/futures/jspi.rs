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
//! [`spawn_local`] is the executor-side complement: it runs a future like
//! `futures::spawn_local`, but each poll is entered through a
//! `WebAssembly.promising` boundary, so sync code reached from the task may
//! itself suspend with [`block_on_promise`]. A suspension parks only that
//! task's poll — the queue and event loop continue.
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
// experimental-status deprecation warning; this module is itself part of the
// experimental JSPI surface, so silence it here.
#![allow(deprecated)]

use crate::Promise;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::mem::ManuallyDrop;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "__wbindgen_placeholder__")]
extern "C" {
    #[wasm_bindgen(catch, suspending)]
    fn __wbindgen_jspi_suspend(promise: &Promise) -> Result<JsValue, JsValue>;

    /// Schedules a task poll on the microtask queue, entered through the
    /// promising-wrapped `__wbg_jspi_task_poll` export so the poll runs
    /// on a fresh fiber. Consumes one strong `Rc<SpawnedTask>` reference,
    /// reclaimed by the trampoline.
    fn __wbindgen_jspi_spawn_poll(task: u32);

    /// The same operation, called only from [`spawn_local`]'s initial
    /// schedule. As a separate import, its presence in the linked module is
    /// the CLI's usage signal for the spawn machinery: the wake path's
    /// `__wbindgen_jspi_spawn_poll` is referenced by the always-exported
    /// trampoline, so only this import distinguishes a module that actually
    /// calls `spawn_local` from one that merely links it.
    fn __wbindgen_jspi_spawn_first(task: u32);
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

// ─── spawn_local: tasks whose polls run on fibers ─────────────────────────────

/// Poll-in-flight discipline for a spawned task.
#[derive(Clone, Copy, PartialEq)]
enum State {
    /// Not scheduled and not being polled.
    Idle,
    /// A poll is queued on the microtask queue but has not started.
    Scheduled,
    /// A poll is in flight — executing, or suspended mid-frame on its fiber.
    Running,
    /// A wake arrived during the in-flight poll: re-poll before going idle.
    RunningWoken,
}

struct SpawnedTask {
    /// `None` once the future has completed (late wakes are no-ops).
    future: RefCell<Option<Pin<Box<dyn Future<Output = ()>>>>>,
    state: Cell<State>,
}

impl SpawnedTask {
    /// Schedule a poll, consuming one strong reference into the intrinsic.
    fn schedule(this: Rc<SpawnedTask>) {
        this.state.set(State::Scheduled);
        __wbindgen_jspi_spawn_poll(Rc::into_raw(this) as u32);
    }

    fn wake(this: &Rc<SpawnedTask>) {
        match this.state.get() {
            // In flight (possibly suspended on its fiber): flag a re-poll.
            // The poll loop consumes the flag when the in-flight poll
            // returns, so the future is never entered reentrantly.
            State::Running => this.state.set(State::RunningWoken),
            State::RunningWoken | State::Scheduled => {}
            State::Idle => {
                if this.future.borrow().is_some() {
                    SpawnedTask::schedule(Rc::clone(this));
                }
            }
        }
    }

    fn poll(this: &Rc<SpawnedTask>) {
        loop {
            this.state.set(State::Running);
            let waker = task_waker(Rc::clone(this));
            let mut cx = Context::from_waker(&waker);
            let done = {
                let mut slot = this.future.borrow_mut();
                let Some(future) = slot.as_mut() else { return };
                // The fiber may suspend inside this poll; `slot` stays
                // mutably borrowed across the suspension, which is sound
                // because every other path to the future first checks
                // `state` (`Running`) and never touches the `RefCell`.
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        *slot = None;
                        true
                    }
                    Poll::Pending => false,
                }
            };
            if done {
                this.state.set(State::Idle);
                return;
            }
            match this.state.replace(State::Idle) {
                // Woken while the poll was in flight: re-poll on this fiber.
                State::RunningWoken => continue,
                _ => return,
            }
        }
    }
}

/// A `Waker` over `Rc<SpawnedTask>`. As in `task::singlethread`, `Waker`'s
/// nominal `Send + Sync` demand is safely ignored: JSPI is single-threaded
/// (rejected under the `atomics` feature), so the lie is confined to the
/// vtable.
fn task_waker(task: Rc<SpawnedTask>) -> Waker {
    unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
        let rc = ManuallyDrop::new(Rc::from_raw(ptr as *const SpawnedTask));
        RawWaker::new(Rc::into_raw(Rc::clone(&rc)) as *const (), &VTABLE)
    }

    unsafe fn raw_wake(ptr: *const ()) {
        let rc = Rc::from_raw(ptr as *const SpawnedTask);
        SpawnedTask::wake(&rc);
    }

    unsafe fn raw_wake_by_ref(ptr: *const ()) {
        let rc = ManuallyDrop::new(Rc::from_raw(ptr as *const SpawnedTask));
        SpawnedTask::wake(&rc);
    }

    unsafe fn raw_drop(ptr: *const ()) {
        drop(Rc::from_raw(ptr as *const SpawnedTask));
    }

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

    unsafe { Waker::from_raw(RawWaker::new(Rc::into_raw(task) as *const (), &VTABLE)) }
}

/// The promising-entered poll trampoline backing [`spawn_local`]. Each call
/// runs one task's poll loop on a fresh fiber. Not public API.
///
/// A raw wasm export rather than a `#[wasm_bindgen(jspi)]` one: the CLI
/// special-cases it by name — when the `__wbindgen_jspi_spawn_first`
/// import is present (i.e. `spawn_local` is actually called somewhere) it
/// receives the same in-wasm fiber wrapper as a jspi export and is invoked
/// through `WebAssembly.promising` by the spawn intrinsics' JS shims;
/// otherwise the export is deleted, so builds that never touch
/// `spawn_local` contain no JSPI instrumentation (and keep running on
/// engines without exnref/JSPI support).
///
/// `extern "C-unwind"`: an unwind must be able to escape the poll — a panic
/// under panic=unwind, or a rethrown rejection of a non-`catch` suspending
/// import — to reject the promising call's promise. Plain `extern "C"` is
/// nounwind, whose `panic_cannot_unwind` guard would abort the whole
/// instance instead of abandoning the one task.
#[no_mangle]
pub extern "C-unwind" fn __wbg_jspi_task_poll(task: u32) {
    let task = unsafe { Rc::from_raw(task as *const SpawnedTask) };
    SpawnedTask::poll(&task);
}

/// Runs a `Future<Output = ()>` on the current thread, like
/// `futures::spawn_local` — but each poll is entered through a
/// `WebAssembly.promising` boundary, so the task's whole call tree may
/// suspend: sync code reached from the future can call [`block_on_promise`].
///
/// A suspension parks only this task's poll; the microtask queue, other
/// tasks, and the event loop continue. A wake arriving while the poll is
/// suspended re-polls after the suspended poll returns.
///
/// The task is leaked if its future never completes, exactly like
/// `spawn_local`.
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    let task = Rc::new(SpawnedTask {
        future: RefCell::new(Some(Box::pin(future))),
        state: Cell::new(State::Idle),
    });
    task.state.set(State::Scheduled);
    __wbindgen_jspi_spawn_first(Rc::into_raw(task) as u32);
}

/// Converts a Rust `Future` into a JavaScript `Promise`, running the future
/// via [`spawn_local`] — each poll is entered through a
/// `WebAssembly.promising` boundary, so sync code reached from the future
/// may itself suspend with [`block_on_promise`].
///
/// This is the jspi counterpart of `futures::future_to_promise`, and backs
/// `#[wasm_bindgen(jspi)] async fn` exports.
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    let mut future = Some(future);

    Promise::new_typed(&mut move |resolve, reject| {
        let future = future.take().unwrap_throw();

        spawn_local(async move {
            match future.await {
                Ok(val) => {
                    resolve.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
                Err(val) => {
                    reject.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
            }
        });
    })
}
