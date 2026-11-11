//! Tokio event loops for `#[wasm_bindgen(experimental_tokio)]` exports.
//! Only available on the `wasm32-unknown-emscripten` target under
//! `--cfg wasm_bindgen_unstable_tokio` (plus tokio's own `--cfg tokio_unstable`).
//!
//! By default all such exports share the thread's ambient event loop: one
//! timer arm, one I/O driver, one keepalive count, and `tokio::spawn` from
//! any of them lands on the same scheduler. With
//! `experimental_tokio = "isolated"` each invocation instead owns a fresh
//! event loop ([`schedule_isolated`]).
//!
//! Either way the event loop is hosted: it ticks itself from host callbacks
//! (reactor readiness, the next timer deadline, an immediate while work is
//! ready), and a schedule from a top-level JS call ticks it once on the
//! caller's stack for a synchronous first poll.
//!
//! Combined with `jspi`, the export is a promising activation and the future
//! runs to completion inside it with [`block_on`] / [`block_on_isolated`] on
//! a runtime that parks by JSPI suspension (linked with `-sJSPI`).

#[cfg(not(tokio_unstable))]
compile_error!("`wasm_bindgen_unstable_tokio` requires tokio's `--cfg tokio_unstable`");

extern crate std;

use core::future::Future;
use std::cell::{Cell, OnceCell, RefCell};
use std::rc::Rc;

pub use ::tokio::runtime::{LocalEventLoop, Runtime};
pub use ::tokio::task::JoinError;
use ::tokio::task::JoinHandle;

std::thread_local! {
    static AMBIENT: OnceCell<RefCell<LocalEventLoop>> = const { OnceCell::new() };
    static AMBIENT_PARKED: OnceCell<Runtime> = const { OnceCell::new() };
}

/// Tasks polled by the tick a schedule runs on its caller's stack: tokio's
/// default `event_interval`, the batch a host tick runs.
const FIRST_TICK: usize = 61;

/// A hosted event loop with every driver the enabled tokio features
/// provide. The I/O driver registers with emscripten's epoll readiness
/// listeners, so it needs Emscripten 6.0.12 or later.
fn build() -> std::io::Result<LocalEventLoop> {
    ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build_hosted_local_event_loop(Default::default())
}

fn ambient<R>(f: impl FnOnce(&RefCell<LocalEventLoop>) -> R) -> R {
    AMBIENT.with(|cell| {
        f(cell.get_or_init(|| {
            RefCell::new(build().expect("failed to build ambient tokio event loop"))
        }))
    })
}

/// Runs `f` with this thread's ambient event loop, building a default one
/// (`enable_all`) on first touch.
///
/// # Panics
///
/// Panics if called from a task which [`schedule`] is polling on its
/// caller's stack: the loop is exclusively borrowed for that tick. A task
/// reaches its own loop through `tokio::spawn` and `tokio::task::spawn_local`.
pub fn with_ambient<R>(f: impl FnOnce(&LocalEventLoop) -> R) -> R {
    ambient(|rt| f(&rt.borrow()))
}

/// Installs `rt` as this thread's ambient event loop, for callers needing
/// their own builder configuration. Must win the race with the first
/// schedule: errs `rt` back if the ambient is already initialized.
pub fn try_set_ambient(rt: LocalEventLoop) -> Result<(), LocalEventLoop> {
    AMBIENT.with(|cell| cell.set(RefCell::new(rt)).map_err(RefCell::into_inner))
}

/// Spawns `future` as a root and a sibling task that awaits its `JoinHandle`
/// and delivers the outcome to `on_complete`, so a root panic arrives as
/// `Err(JoinError)`. Both go to `rt`, or with `None` to the loop whose tick
/// is on the stack. They are queued only.
fn spawn_root<F, C>(rt: Option<&LocalEventLoop>, future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    fn spawn<F>(rt: Option<&LocalEventLoop>, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        match rt {
            Some(rt) => rt.spawn_local(future),
            None => ::tokio::task::spawn_local(future),
        }
    }
    let root = spawn(rt, future);
    spawn(rt, async move { on_complete(root.await) });
}

/// Whether a tick may run on this stack: no runtime is entered on the
/// thread. One is while a task's JS import re-enters an export, and while a
/// `block_on` is suspended through JSPI.
fn can_tick() -> bool {
    ::tokio::runtime::Handle::try_current().is_err()
}

/// Schedules `future` as a root on the ambient event loop, delivering its
/// outcome (or a panic, as `Err(JoinError)`) to `on_complete`.
///
/// Called outside any runtime context (a top-level JS call) this ticks the
/// loop once on the caller's stack, so the first poll is synchronous (parity
/// with `future_to_promise`) and belongs to the calling invocation's host
/// task rather than to whichever one armed the loop's next host callback.
/// The tick runs one batch of the loop's ready tasks, so other roots' tasks
/// may be polled in it as well.
///
/// Called inside a runtime context (a task's JS import invoking an export,
/// or while a `block_on` is suspended) no tick can nest, so the root is only
/// queued: the tick in progress picks it up, or the loop's next host tick.
pub fn schedule<F, C>(future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    ambient(|cell| {
        // Exclusively borrowed only by the tick below, which is then on the
        // stack with the ambient loop as the current runtime.
        match cell.try_borrow() {
            Ok(rt) => spawn_root(Some(&rt), future, on_complete),
            Err(_) => spawn_root(None, future, on_complete),
        }
        if can_tick() {
            if let Ok(mut rt) = cell.try_borrow_mut() {
                rt.tick(FIRST_TICK);
            }
        }
    })
}

/// Schedules `future` as the root of a fresh event loop owned by this call,
/// with the same first-poll semantics as [`schedule`]. The event loop's
/// reactor, timers, and any tasks spawned inside `future` are fully isolated
/// from other invocations, for multiplexed hosts (e.g. Cloudflare Workers)
/// where one invocation's event loop must not perform I/O on behalf of
/// another's context.
///
/// The completion task owns the event loop, so it lives until the root
/// settles and then tears down with native `Runtime` drop semantics: spawned
/// tasks still in flight are dropped and the reactor is closed.
pub fn schedule_isolated<F, C>(future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    let mut rt = build().expect("failed to build isolated tokio event loop");
    let owned = Rc::new(Cell::new(None));
    let settled = owned.clone();
    spawn_root(Some(&rt), future, move |out| {
        on_complete(out);
        drop(settled.take());
    });
    if can_tick() {
        rt.tick(FIRST_TICK);
    }
    // The completion task is gone if the root settled in that tick; the
    // loop then drops here.
    if Rc::strong_count(&owned) > 1 {
        owned.set(Some(rt));
    }
}

/// A parked runtime: a current-thread `Runtime` whose idle waits are JSPI
/// suspensions of the calling activation.
fn build_parked() -> std::io::Result<Runtime> {
    ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}

/// Runs `future` to completion on the thread's ambient parked runtime, for
/// `#[wasm_bindgen(jspi, experimental_tokio)]`: the export is a promising
/// activation, and every wait of the runtime (timers, I/O, an idle scheduler)
/// is a JSPI suspension of that activation. `jspi_block_on_promise` and
/// suspending imports inside task code park it mid-poll the same way.
///
/// The runtime stays entered on the thread for as long as the activation is
/// parked, so one invocation runs at a time:
///
/// * A sibling `block_on` or [`block_on_isolated`] arriving meanwhile panics
///   as a nested runtime.
/// * A sibling [`schedule`] or [`schedule_isolated`] is queued without a
///   first poll, and its event loop's host ticks are deferred until this
///   call returns.
pub fn block_on<F: Future>(future: F) -> F::Output {
    AMBIENT_PARKED.with(|cell| {
        cell.get_or_init(|| build_parked().expect("failed to build ambient tokio runtime"))
            .block_on(future)
    })
}

/// [`block_on`] on a fresh runtime owned by this call and dropped once
/// `future` settles (tasks still in flight are dropped, the reactor closed),
/// for `#[wasm_bindgen(jspi, experimental_tokio = "isolated")]`.
pub fn block_on_isolated<F: Future>(future: F) -> F::Output {
    build_parked()
        .expect("failed to build isolated tokio runtime")
        .block_on(future)
}
