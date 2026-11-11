//! Tokio event loops for `#[wasm_bindgen(experimental_tokio)]` exports.
//! Only available on the `wasm32-unknown-emscripten` target under
//! `--cfg wasm_bindgen_unstable_tokio` (plus tokio's own `--cfg tokio_unstable`).
//!
//! An [`EventLoop`] is a hosted tokio event loop: it ticks itself from host
//! callbacks (reactor readiness, the next timer deadline, an immediate while
//! work is ready), and a schedule from a top-level JS call ticks it once on
//! the caller's stack for a synchronous first poll. Every root and spawned
//! task holds its loop, so a loop lives while anyone holds it.
//!
//! An `experimental_tokio` export roots on [`EventLoop::current`]: the
//! calling task's loop, else the innermost [`EventLoop::enter`], else the
//! thread's ambient loop, so by default all such exports share one timer
//! arm, one I/O driver, and one scheduler. With
//! `experimental_tokio = "isolated"` each invocation instead roots on a
//! fresh [`EventLoop::new`], isolated from other invocations for
//! multiplexed hosts (e.g. Cloudflare Workers) where one invocation's event
//! loop must not perform I/O on behalf of another's context.
//!
//! * Work outliving the root, such as a Workers `ctx.waitUntil`:
//!   `EventLoop::current().spawn(fut)` holds the root's loop until `fut`
//!   finishes.
//! * One loop per long-lived I/O context, such as a Durable Object
//!   instance: the host holds an `EventLoop` and calls the instance's
//!   exports inside `rt.enter(|| ..)`.
//!
//! Combined with `jspi`, the export is a promising activation and the future
//! runs to completion inside it with [`block_on`] / [`block_on_isolated`] on
//! a runtime that parks by JSPI suspension (linked with `-sJSPI`).

#[cfg(not(tokio_unstable))]
compile_error!("`wasm_bindgen_unstable_tokio` requires tokio's `--cfg tokio_unstable`");

extern crate std;

use core::future::Future;
use std::cell::{OnceCell, RefCell};
use std::rc::{Rc, Weak};
use std::vec::Vec;

use ::tokio::runtime::{Handle, Id};
pub use ::tokio::runtime::{LocalEventLoop, Runtime};
pub use ::tokio::task::{JoinError, JoinHandle};

std::thread_local! {
    static AMBIENT: OnceCell<EventLoop> = const { OnceCell::new() };
    static AMBIENT_PARKED: OnceCell<Runtime> = const { OnceCell::new() };
    /// The loops of the [`EventLoop::enter`] calls on the stack, innermost
    /// last.
    static ENTERED: RefCell<Vec<EventLoop>> = const { RefCell::new(Vec::new()) };
    /// Every live loop by runtime id, for [`EventLoop::current`] to find the
    /// calling task's. Each removes itself on drop.
    static LOOPS: RefCell<Vec<(Id, Weak<Inner>)>> = const { RefCell::new(Vec::new()) };
}

/// Tasks polled by the tick a schedule runs on its caller's stack: tokio's
/// default `event_interval`, the batch a host tick runs.
const FIRST_TICK: usize = 61;

/// A hosted tokio event loop, shared by its clones.
#[derive(Clone, Debug)]
pub struct EventLoop(Rc<Inner>);

#[derive(Debug)]
struct Inner {
    id: Id,
    /// Exclusively borrowed only by a tick on the caller's stack, which then
    /// has this loop as the current runtime.
    rt: RefCell<LocalEventLoop>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let _ = LOOPS.try_with(|loops| loops.borrow_mut().retain(|(id, _)| *id != self.id));
    }
}

impl EventLoop {
    /// A fresh loop with every driver the enabled tokio features provide.
    /// The I/O driver registers with emscripten's epoll readiness listeners,
    /// so it needs Emscripten 6.0.12 or later.
    #[allow(clippy::new_without_default)]
    pub fn new() -> EventLoop {
        let rt = ::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build_hosted_local_event_loop(Default::default())
            .expect("failed to build tokio event loop");
        EventLoop::from_local(rt)
    }

    /// A loop from a custom builder:
    /// `Builder::new_current_thread()..build_hosted_local_event_loop(..)`.
    pub fn from_local(rt: LocalEventLoop) -> EventLoop {
        let id = rt.handle().id();
        let inner = Rc::new(Inner {
            id,
            rt: RefCell::new(rt),
        });
        LOOPS.with(|loops| loops.borrow_mut().push((id, Rc::downgrade(&inner))));
        EventLoop(inner)
    }

    /// This thread's ambient loop, built with [`new`](Self::new) on first
    /// use unless [`set_ambient`](Self::set_ambient) came first.
    pub fn ambient() -> EventLoop {
        AMBIENT.with(|cell| cell.get_or_init(EventLoop::new).clone())
    }

    /// Installs this loop as the thread's ambient one. Errs it back if the
    /// ambient is already built.
    pub fn set_ambient(self) -> Result<(), EventLoop> {
        AMBIENT.with(|cell| cell.set(self))
    }

    /// The loop of the calling task, else of the innermost
    /// [`enter`](Self::enter), else the ambient loop.
    pub fn current() -> EventLoop {
        if let Ok(handle) = Handle::try_current() {
            let id = handle.id();
            let task = LOOPS.with(|loops| {
                let loops = loops.borrow();
                loops.iter().find(|(i, _)| *i == id)?.1.upgrade()
            });
            if let Some(inner) = task {
                return EventLoop(inner);
            }
        }
        if let Some(rt) = ENTERED.with(|entered| entered.borrow().last().cloned()) {
            return rt;
        }
        EventLoop::ambient()
    }

    /// Runs `f` with this loop as [`current`](Self::current) outside its
    /// tasks, so the `experimental_tokio` exports `f` calls root on it.
    /// Nests, and restores the previous loop as `f` returns or unwinds. It
    /// does not enter a tokio runtime context.
    pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        struct Exit;
        impl Drop for Exit {
            fn drop(&mut self) {
                let rt = ENTERED.with(|entered| entered.borrow_mut().pop());
                drop(rt);
            }
        }
        ENTERED.with(|entered| entered.borrow_mut().push(self.clone()));
        let _exit = Exit;
        f()
    }

    /// Schedules `future` as a root, delivering its outcome (or a panic, as
    /// `Err(JoinError)`) to `on_complete`. The root holds the loop until it
    /// settles.
    ///
    /// Called outside any runtime context (a top-level JS call) this ticks
    /// the loop once on the caller's stack, so the first poll is synchronous
    /// (parity with `future_to_promise`) and belongs to the calling
    /// invocation's host task rather than to whichever one armed the loop's
    /// next host callback. The tick runs one batch of the loop's ready tasks,
    /// so other roots' tasks may be polled in it as well.
    ///
    /// Called inside a runtime context (a task's JS import invoking an
    /// export, or while a `block_on` is suspended) no tick can nest, so the
    /// root is only queued: the tick in progress picks it up, or the loop's
    /// next host tick.
    pub fn schedule<F, C>(&self, future: F, on_complete: C)
    where
        F: Future + 'static,
        F::Output: 'static,
        C: FnOnce(Result<F::Output, JoinError>) + 'static,
    {
        let root = self.spawn_local(future);
        let hold = self.clone();
        self.spawn_local(async move {
            on_complete(root.await);
            drop(hold);
        });
        if Handle::try_current().is_err() {
            if let Ok(mut rt) = self.0.rt.try_borrow_mut() {
                rt.tick(FIRST_TICK);
            }
        }
        // A root that settled in that tick let go of the loop; if that was
        // the last hold, the caller's clone drops it outside the tick.
    }

    /// Spawns `future` on the loop, holding it until `future` finishes. Only
    /// queued. Dropping the `JoinHandle` detaches the task, which keeps its
    /// hold; a task that never finishes holds the loop for good.
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let hold = self.clone();
        self.spawn_local(async move {
            let out = future.await;
            drop(hold);
            out
        })
    }

    /// Spawns on the loop, or on the current runtime while this loop's
    /// caller-stack tick holds it, which then is this loop.
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        match self.0.rt.try_borrow() {
            Ok(rt) => rt.spawn_local(future),
            Err(_) => ::tokio::task::spawn_local(future),
        }
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
/// * A sibling [`EventLoop::schedule`] is queued without a first poll, and
///   its event loop's host ticks are deferred until this call returns.
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
