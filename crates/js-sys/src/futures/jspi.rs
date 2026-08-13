//! Internal JSPI (JS Promise Integration) machinery.
//!
//! The public surface is a single function, `futures::jspi_block_on_promise`:
//! suspend the current promising execution until a JS `Promise` settles. The
//! executor side has no API at all — `spawn_local` (and everything built on
//! it, like `future_to_promise` and async exports) checks the ambient JSPI
//! context at spawn time and enters the task's polls through a
//! `WebAssembly.promising` boundary when spawned from within one, so sync
//! code reached from such a task may itself suspend. The capability is
//! inherited transitively: a task spawned from a promising-entered poll is
//! itself promising-entered.
//!
//! The runtime state is minimal by construction. The suspend bridge,
//! `__wbindgen_jspi_suspend`, is a plain `#[wasm_bindgen(catch, suspending)]`
//! import whose JS body is an identity function: JSPI itself awaits the
//! returned `Promise` and resumes with the settled value, while a rejection
//! is thrown into wasm as a `WebAssembly.JSTag` exception and marshalled to
//! `Err` by the standard `catch + suspending` machinery (see
//! `cli-support/src/transforms/jspi.rs`). The ambient context probe,
//! `__wbindgen_jspi_in_context`, is rewritten in-wasm by the CLI: it reads
//! the `__jspi_stack_base` global when JSPI instrumentation is active and is
//! a constant `0` otherwise, so modules that never use JSPI attributes carry
//! no JSPI machinery and keep running on engines without exnref/JSPI.

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

    /// Whether a JSPI context is on the stack: a `#[wasm_bindgen(jspi)]`
    /// export or a promising-entered poll. Rewritten in-wasm by the CLI to
    /// read the `__jspi_stack_base` global (constant `0` in modules without
    /// JSPI instrumentation); no JS shim is ever emitted.
    fn __wbindgen_jspi_in_context() -> u32;

    /// Schedules a task poll on the microtask queue, entered through the
    /// promising-wrapped `__wbg_jspi_task_poll` export so the poll runs
    /// on a fresh suspendable stack. Consumes one strong `Rc<SpawnedTask>`
    /// reference, reclaimed by the trampoline.
    fn __wbindgen_jspi_spawn_poll(task: u32);

    /// The same operation, called only from the initial schedule in
    /// [`spawn_promising`].
    fn __wbindgen_jspi_spawn_first(task: u32);
}

/// Whether the caller is executing within a JSPI context, i.e. whether a
/// spawned task's polls should be promising-entered.
pub(crate) fn in_context() -> bool {
    __wbindgen_jspi_in_context() != 0
}

/// Suspend the current promising execution until `promise` settles.
/// Backs `futures::jspi_block_on_promise`.
///
/// On return the shadow stack is guaranteed to hold the correct contents
/// for this execution, regardless of what else ran while it was suspended:
/// the wasm-bindgen CLI instruments every `#[wasm_bindgen(suspending)]`
/// import with an in-wasm wrapper that evacuates the live shadow-stack
/// region to a heap buffer before suspending and copies it back as the very
/// first instructions after resume (see `cli-support/src/transforms/jspi.rs`).
/// The `catch` marshalling (rejections as `Err` data) rides the same wrapper.
#[inline(never)]
pub(crate) fn suspend(promise: &Promise) -> Result<JsValue, JsValue> {
    __wbindgen_jspi_suspend(promise)
}

// ─── Promising-entered task polls ────────────────────────────────────────────

/// Poll-in-flight discipline for a spawned task.
#[derive(Clone, Copy, PartialEq)]
enum State {
    /// Not scheduled and not being polled.
    Idle,
    /// A poll is queued on the microtask queue but has not started.
    Scheduled,
    /// A poll is in flight — executing, or suspended mid-frame.
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
            // In flight (possibly suspended): flag a re-poll. The poll loop
            // consumes the flag when the in-flight poll returns, so the
            // future is never entered reentrantly.
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
                // The poll may suspend; `slot` stays mutably borrowed across
                // the suspension, which is sound because every other path to
                // the future first checks `state` (`Running`) and never
                // touches the `RefCell`.
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
                // Woken while the poll was in flight: re-poll.
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

/// The promising-entered poll trampoline. Each call runs one task's poll
/// loop on a fresh suspendable stack. Not public API.
///
/// A raw wasm export rather than a `#[wasm_bindgen(jspi)]` one: the CLI
/// special-cases it by name — in a module with `#[wasm_bindgen(jspi)]`
/// exports it receives the same in-wasm wrapper as a jspi export and is
/// invoked through `WebAssembly.promising` by the spawn intrinsics' JS
/// shims; in a module without them the ambient context is constant-false,
/// so the export is deleted and the spawn intrinsics are stubbed out,
/// leaving no JSPI machinery at all.
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

/// Runs a `Future<Output = ()>` with every poll entered through a
/// `WebAssembly.promising` boundary, so the task's whole call tree may
/// suspend. Backs `spawn_local` when called within a JSPI context, and
/// `#[wasm_bindgen(jspi)] async fn` exports unconditionally.
///
/// A suspension parks only this task's poll; the microtask queue, other
/// tasks, and the event loop continue. A wake arriving while the poll is
/// suspended re-polls after the suspended poll returns. The task is leaked
/// if its future never completes, exactly like `spawn_local`.
pub(crate) fn spawn_promising<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    let task = Rc::new(SpawnedTask {
        future: RefCell::new(Some(Box::pin(future))),
        state: Cell::new(State::Scheduled),
    });
    __wbindgen_jspi_spawn_first(Rc::into_raw(task) as u32);
}

/// `future_to_promise`, but with the future's polls promising-entered
/// unconditionally. Backs `#[wasm_bindgen(jspi)] async fn` exports, which
/// are entered from JS where the ambient context is never set.
pub fn future_to_promise_promising<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    let mut future = Some(future);

    Promise::new_typed(&mut move |resolve, reject| {
        let future = future.take().unwrap_throw();

        spawn_promising(async move {
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
