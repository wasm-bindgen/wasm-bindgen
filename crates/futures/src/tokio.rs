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
//! Combined with `jspi`, the export is a promising activation and the future
//! runs to completion inside it with [`block_on`] / [`block_on_isolated`] on
//! a runtime that parks by JSPI suspension (linked with `-sJSPI`).

#[cfg(not(tokio_unstable))]
compile_error!("`wasm_bindgen_unstable_tokio` requires tokio's `--cfg tokio_unstable`");

extern crate std;

use core::future::Future;
use std::cell::OnceCell;

pub use ::tokio::runtime::{LocalEventLoop, Runtime};
pub use ::tokio::task::JoinError;

std::thread_local! {
    static AMBIENT: OnceCell<LocalEventLoop> = const { OnceCell::new() };
    static AMBIENT_PARKED: OnceCell<Runtime> = const { OnceCell::new() };
}

/// Builds with every driver the enabled tokio features provide. The I/O
/// driver needs emscripten's epoll readiness listeners
/// (emscripten-core/emscripten#27547); on an emscripten without them it
/// reports `Unsupported`, and the loop is rebuilt with timers only.
fn build() -> std::io::Result<LocalEventLoop> {
    match ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build_hosted_local_event_loop(Default::default())
    {
        Err(e) if e.kind() == std::io::ErrorKind::Unsupported => {
            ::tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build_hosted_local_event_loop(Default::default())
        }
        built => built,
    }
}

/// Runs `f` with this thread's ambient event loop, building a default one
/// (`enable_all`) on first touch.
pub fn with_ambient<R>(f: impl FnOnce(&LocalEventLoop) -> R) -> R {
    AMBIENT.with(|cell| {
        f(cell.get_or_init(|| build().expect("failed to build ambient tokio event loop")))
    })
}

/// Installs `rt` as this thread's ambient event loop, for callers needing
/// their own builder configuration. Must win the race with the first
/// schedule: errs `rt` back if the ambient is already initialized.
pub fn try_set_ambient(rt: LocalEventLoop) -> Result<(), LocalEventLoop> {
    AMBIENT.with(|cell| cell.set(rt))
}

/// Spawns `future` as a root on `rt` and a sibling task that awaits its
/// `JoinHandle` and delivers the outcome to `on_complete`, so a root panic
/// arrives as `Err(JoinError)`. Both are queued only (the hosted loop
/// schedules its own drive); nothing runs on the caller's stack.
fn spawn_root<F, C>(rt: &LocalEventLoop, future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    let root = rt.spawn_local(future);
    rt.spawn_local(async move { on_complete(root.await) });
}

/// Schedules `future` as a root on the ambient event loop, delivering its
/// outcome (or a panic, as `Err(JoinError)`) to `on_complete`.
///
/// Called outside any runtime context — a top-level JS call — this drives
/// one batch immediately, so the first poll is synchronous (parity with
/// `future_to_promise`). Called re-entrantly — a task's JS import invoking
/// an export mid-drive — driving on the caller's stack would nest the
/// runtime context, so the root is only queued; the in-progress drive picks
/// it up, or the hosted loop's own scheduled drive does once it returns.
pub fn schedule<F, C>(future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    with_ambient(|rt| {
        spawn_root(rt, future, on_complete);
        if ::tokio::runtime::Handle::try_current().is_err() {
            rt.drive();
        }
    })
}

/// Schedules `future` as the root of a fresh event loop owned by this call,
/// with the same drive semantics as [`schedule`]. The event loop's reactor,
/// timers, and any tasks spawned inside `future` are fully isolated from
/// other invocations, for multiplexed hosts (e.g. Cloudflare Workers) where
/// one invocation's event loop must not perform I/O on behalf of another's
/// context.
///
/// The completion task owns the event loop, so it lives until the root
/// settles and then tears down with native `Runtime` drop semantics: spawned
/// tasks still in flight are dropped and the reactor is closed. The drop is
/// deferred to a microtask, since a runtime cannot be dropped from inside
/// its own drive. Armed host callbacks only hold weak references, so nothing
/// else keeps it alive.
pub fn schedule_isolated<F, C>(future: F, on_complete: C)
where
    F: Future + 'static,
    F::Output: 'static,
    C: FnOnce(Result<F::Output, JoinError>) + 'static,
{
    let rt = std::rc::Rc::new(build().expect("failed to build isolated tokio event loop"));
    let keep = rt.clone();
    spawn_root(&rt, future, move |out| {
        crate::spawn_local(async move { drop(keep) });
        on_complete(out);
    });
    if ::tokio::runtime::Handle::try_current().is_err() {
        rt.drive();
    }
}

/// Builds a parked runtime: a current-thread `Runtime` whose idle waits are
/// JSPI suspensions of the calling activation. Same driver fallback as
/// [`build`].
fn build_parked() -> std::io::Result<Runtime> {
    match ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Err(e) if e.kind() == std::io::ErrorKind::Unsupported => {
            ::tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
        }
        built => built,
    }
}

/// Runs `future` to completion on the thread's ambient parked runtime, for
/// `#[wasm_bindgen(jspi, experimental_tokio)]`: the export is a promising
/// activation, and every wait of the runtime (timers, I/O, an idle scheduler)
/// is a JSPI suspension of that activation, which leaves the runtime. A
/// sibling invocation arriving meanwhile enters the same runtime and waits
/// on its own.
///
/// A suspension the runtime does not issue (`jspi_block_on_promise` or a
/// suspending import inside task code) parks the activation mid-poll. With
/// tokio's fiber-owned runtime context (`--cfg tokio_unstable_jspi_hooks`) a
/// sibling still enters, and its timers and I/O advance once the parked
/// activation resumes and releases the scheduler core; without it the
/// runtime stays entered across the suspension and the sibling's `block_on`
/// is a nested runtime.
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
