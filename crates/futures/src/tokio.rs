//! Tokio event loops for `#[wasm_bindgen(tokio)]` exports.
//!
//! By default all such exports share the thread's ambient event loop: one
//! timer arm, one I/O driver, one keepalive count, and `tokio::spawn` from
//! any of them lands on the same scheduler. With `tokio = "isolated"` each
//! invocation instead owns a fresh event loop ([`schedule_isolated`]).

use core::future::Future;
use std::cell::OnceCell;

pub use ::tokio::runtime::LocalEventLoop;
pub use ::tokio::task::JoinError;

std::thread_local! {
    static AMBIENT: OnceCell<LocalEventLoop> = const { OnceCell::new() };
}

fn build() -> std::io::Result<LocalEventLoop> {
    ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build_hosted_local_event_loop(Default::default())
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
