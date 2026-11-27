//! The JavaScript host loop as a tokio event loop's host.
//!
//! A tokio `LocalEventLoop` leaves its wait to a host: it exposes a
//! descriptor readable when it has work, a callback for the same, and the
//! time until its soonest timer, and the host ticks it when either fires.
//! Here the host is Emscripten's: a persistent readiness listener on
//! `event_fd` (`emscripten_epoll_listener_add`), a host timer armed for
//! `next_sleep` after every tick, and an immediate while work is ready, so a
//! busy loop yields a host turn between batches. Each callback ticks the
//! loop. Without the I/O driver there is no `event_fd`, and
//! `on_work_available` schedules the immediate instead.
//!
//! The loop holds the Emscripten runtime alive while it has tasks, as a
//! native one lives while `block_on` runs, so `main` may return with work in
//! flight.

extern crate std;

use core::ffi::c_void;
use core::future::Future;
use core::mem::ManuallyDrop;
use core::time::Duration;
use std::cell::{Cell, RefCell};
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering::Relaxed};
use std::sync::Arc;
use std::time::Instant;
use std::vec::Vec;

use ::tokio::runtime::{Builder, Id, LocalEventLoop};
use ::tokio::task::JoinHandle;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::Function;

type Callback = extern "C" fn(*mut c_void);
type EpollCallback = extern "C" fn(i32, *mut c_void);

extern "C" {
    /// Runs `cb(user_data)` after `msecs` on the host loop, holding the
    /// Emscripten runtime alive until it fires or is cleared.
    fn emscripten_set_timeout(cb: Callback, msecs: f64, user_data: *mut c_void) -> i32;
    fn emscripten_clear_timeout(id: i32);
    /// Runs `cb(user_data)` on the next host loop turn (`setImmediate`),
    /// holding the runtime alive until then.
    fn emscripten_set_immediate(cb: Callback, user_data: *mut c_void) -> i32;
    fn emscripten_clear_immediate(id: i32);
    fn emscripten_runtime_keepalive_push();
    fn emscripten_runtime_keepalive_pop();
    /// Persistent readiness listener on an epoll fd: `cb(epfd, user_data)`
    /// runs on the host loop whenever the set has uncollected ready events.
    /// Holds nothing itself.
    fn emscripten_epoll_listener_add(epfd: i32, cb: EpollCallback, user_data: *mut c_void) -> i32;
    fn emscripten_epoll_listener_remove(
        epfd: i32,
        cb: EpollCallback,
        user_data: *mut c_void,
    ) -> i32;
}

/// Tasks polled per tick: tokio's default `event_interval`.
const BATCH: usize = 61;

/// The host clamps a timeout above its 32-bit millisecond limit to one, so
/// longer sleeps are capped; the early tick re-arms.
const MAX_TIMEOUT_MS: f64 = i32::MAX as f64;

/// An armed deadline within this of the wanted one stays armed. Timers are
/// a millisecond apart on tokio's wheel, and the two readings of one
/// deadline differ only by the clock reads between them.
const DEADLINE_SLACK: Duration = Duration::from_micros(500);

std::thread_local! {
    /// The runtimes being driven on this thread's stack: ticks in progress
    /// and this module's `block_on`s, running or suspended through JSPI.
    static DRIVING: Cell<usize> = const { Cell::new(0) };
    /// Loops whose host tick arrived while a runtime was driven on this
    /// thread, where no tick can run: a `block_on` suspended through JSPI.
    static DEFERRED: RefCell<Vec<Weak<Host>>> = const { RefCell::new(Vec::new()) };
}

/// Marks a runtime as driven on this thread until dropped. Tokio refuses to
/// drive a second one meanwhile, and has no query for it: a runtime context
/// that is only entered (`Runtime::enter`) is not one, and holds no tick
/// back.
pub(super) struct Driving(());

impl Driving {
    pub(super) fn enter() -> Driving {
        DRIVING.set(DRIVING.get() + 1);
        Driving(())
    }

    pub(super) fn any() -> bool {
        DRIVING.get() > 0
    }
}

impl Drop for Driving {
    fn drop(&mut self) {
        DRIVING.set(DRIVING.get() - 1);
        resume_deferred();
    }
}

/// Reschedules the ticks deferred while a runtime was driven, once none is.
fn resume_deferred() {
    if Driving::any() || DEFERRED.with(|deferred| deferred.borrow().is_empty()) {
        return;
    }
    for host in DEFERRED.take() {
        if let Some(host) = host.upgrade() {
            let _ = host.listen(true);
            host.schedule();
        }
    }
}

/// The event loop's hold on the Emscripten runtime. The readiness listener
/// holds nothing, so the loop holds it itself while it has tasks: taken as
/// soon as a wakeup arrives while idle (a spawn from host context is one),
/// and reconciled with the live task count after every turn. The wakeup
/// callback outlives the loop in any `Handle` clone, so once the loop
/// detaches it takes no more.
#[derive(Debug, Default)]
struct Hold {
    held: AtomicBool,
    detached: AtomicBool,
    /// The loop to schedule a tick on for a wakeup while idle, set when it
    /// has no readiness listener for the driver to wake it through.
    wake: AtomicPtr<Host>,
}

impl Hold {
    /// A wakeup arrived while the loop was idle.
    fn work_available(&self) {
        if self.detached.load(Relaxed) {
            return;
        }
        self.set(true);
        let host = self.wake.load(Relaxed);
        if !host.is_null() {
            // SAFETY: set in `Host::attach` to the `Host` in its `Rc`, and
            // cleared by `detach` in its `drop`. The target has no threads,
            // so this is the loop's thread.
            unsafe { &*host }.schedule();
        }
    }

    /// The final release: nothing is pushed after it.
    fn detach(&self) {
        self.detached.store(true, Relaxed);
        self.wake.store(core::ptr::null_mut(), Relaxed);
        self.set(false);
    }

    fn set(&self, alive: bool) {
        if self.held.swap(alive, Relaxed) != alive {
            // SAFETY: Emscripten runtime calls; every push is paired with one
            // pop here, the last in `Host::drop`.
            unsafe {
                if alive {
                    emscripten_runtime_keepalive_push();
                } else {
                    emscripten_runtime_keepalive_pop();
                }
            }
        }
    }
}

/// A tokio event loop attached to the host loop. Every host callback
/// receives the `Host` as its argument; it lives in an `Rc` at a fixed
/// address, and detaches on drop before the loop's descriptor closes. A
/// callback holds it for the tick it runs, so a task that lets go of the
/// loop frees nothing under the tick.
#[derive(Debug)]
pub(super) struct Host {
    this: Weak<Host>,
    pub(super) id: Id,
    /// Exclusively borrowed only for a tick, which then has this loop as the
    /// current runtime. Taken by `drop`.
    rt: RefCell<ManuallyDrop<LocalEventLoop>>,
    /// [`turn`](Self::turn) as a JavaScript function, which a tick calls
    /// it through.
    turn: Closure<dyn Fn()>,
    /// The reactor's epoll fd, with the I/O driver.
    epfd: Option<RawFd>,
    listening: Cell<bool>,
    /// The armed deadline's timeout id and when it fires.
    deadline: Cell<Option<(i32, Instant)>>,
    /// The follow-up tick for ready work.
    immediate: Cell<Option<i32>>,
    hold: Arc<Hold>,
}

impl Host {
    /// Builds the loop and registers it with the host. The callbacks take
    /// the `Host`'s address, so it is placed before it registers.
    pub(super) fn attach(builder: &mut Builder) -> io::Result<Rc<Host>> {
        let hold = Arc::new(Hold::default());
        let rt = builder
            .on_work_available({
                let hold = hold.clone();
                move || hold.work_available()
            })
            .build_local_event_loop(Default::default())?;
        let epfd = rt.event_fd().map(|fd| fd.as_raw_fd());
        let host = Rc::new_cyclic(|this| Host {
            this: this.clone(),
            id: rt.handle().id(),
            rt: RefCell::new(ManuallyDrop::new(rt)),
            turn: Closure::own_aborting({
                let this = this.clone();
                move || {
                    if let Some(host) = this.upgrade() {
                        host.turn();
                    }
                }
            }),
            epfd,
            listening: Cell::new(false),
            deadline: Cell::new(None),
            immediate: Cell::new(None),
            hold,
        });
        if epfd.is_none() {
            host.hold.wake.store(Rc::as_ptr(&host).cast_mut(), Relaxed);
        }
        host.listen(true)?;
        Ok(host)
    }

    /// Spawns on the loop, or on the current runtime while a tick holds the
    /// loop, which then is this loop.
    pub(super) fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        match self.rt.try_borrow() {
            Ok(rt) => rt.spawn_local(future),
            Err(_) => ::tokio::task::spawn_local(future),
        }
    }

    /// Runs one batch of ready tasks on this stack, then re-arms the host.
    /// No runtime is being driven on this thread.
    ///
    /// A tick either completes or the instance aborts. A JavaScript
    /// exception thrown by an import while a task is polled is no Rust
    /// panic: tokio's task harness cannot complete the task with it, and
    /// under `panic=abort` it passes without running a single drop, leaving
    /// the runtime entered for good. Only JavaScript can catch it, so the
    /// tick runs under a JavaScript frame that does.
    pub(super) fn tick(&self) {
        let driving = Driving::enter();
        let turn: &Function = self.turn.as_ref().unchecked_ref();
        if let Err(exception) = turn.call0(&JsValue::UNDEFINED) {
            std::eprintln!(
                "JavaScript exception thrown through a tokio event loop tick; aborting: {exception:?}"
            );
            std::process::abort();
        }
        drop(driving);
    }

    fn turn(&self) {
        let mut rt = self.rt.borrow_mut();
        rt.tick(BATCH);
        self.after_turn(&rt);
    }

    /// Re-arms the host after a tick: an immediate for ready work, a timer
    /// for the deadline, and the hold on the runtime while tasks are alive.
    /// Every host call is a wasm-to-JS call, so an armed timer stays when
    /// its deadline did not move, and ready work leaves it to the next turn.
    fn after_turn(&self, rt: &LocalEventLoop) {
        match rt.next_sleep() {
            Some(Duration::ZERO) => self.schedule(),
            Some(sleep) => self.arm(sleep),
            None => self.disarm(),
        }
        self.hold.set(rt.handle().metrics().num_alive_tasks() > 0);
    }

    fn arm(&self, sleep: Duration) {
        let Some(at) = Instant::now().checked_add(sleep) else {
            return self.disarm();
        };
        if let Some((_, armed)) = self.deadline.get() {
            let moved = if at > armed { at - armed } else { armed - at };
            if moved < DEADLINE_SLACK {
                return;
            }
        }
        self.disarm();
        let ms = (sleep.as_secs_f64() * 1000.0).min(MAX_TIMEOUT_MS);
        // SAFETY: `user_data` is this `Host`, which clears the timeout in
        // `drop`.
        let id = unsafe { emscripten_set_timeout(deadline, ms, self.user_data()) };
        self.deadline.set(Some((id, at)));
    }

    fn disarm(&self) {
        if let Some((id, _)) = self.deadline.take() {
            // SAFETY: a pending timeout armed in `arm`.
            unsafe { emscripten_clear_timeout(id) };
        }
    }

    /// Adds or removes the readiness listener. It fires every host turn
    /// while the set has uncollected events, so it comes off while a tick
    /// cannot run (see [`host_tick`](Self::host_tick)).
    fn listen(&self, on: bool) -> io::Result<()> {
        let Some(epfd) = self.epfd else {
            return Ok(());
        };
        if self.listening.get() == on {
            return Ok(());
        }
        // SAFETY: the reactor's live epoll fd; `user_data` is this `Host`,
        // which removes the listener in `drop` before the loop drops.
        let rc = unsafe {
            if on {
                emscripten_epoll_listener_add(epfd, ready, self.user_data())
            } else {
                emscripten_epoll_listener_remove(epfd, ready, self.user_data())
            }
        };
        if rc != 0 {
            return Err(io::Error::from_raw_os_error(rc));
        }
        self.listening.set(on);
        Ok(())
    }

    /// Schedules a tick on the next host turn, unless one is pending.
    fn schedule(&self) {
        if self.immediate.get().is_some() {
            return;
        }
        // SAFETY: `user_data` is this `Host`, which clears the immediate in
        // `drop`.
        self.immediate.set(Some(unsafe {
            emscripten_set_immediate(immediate, self.user_data())
        }));
    }

    fn user_data(&self) -> *mut c_void {
        self as *const Host as *mut c_void
    }

    /// The `Host` a callback was registered by, held for the callback.
    ///
    /// # Safety
    ///
    /// `user_data` is the argument a `Host` registered the callback with.
    /// `drop` removes every callback, so none fires on a freed `Host`.
    unsafe fn from_callback(user_data: *mut c_void) -> Rc<Host> {
        let host = unsafe { &*(user_data as *const Host) };
        host.this
            .upgrade()
            .expect("a registered callback's loop is alive")
    }

    /// Ticks the loop from a host callback.
    ///
    /// While a runtime is driven on this thread (a `block_on` suspended
    /// through JSPI) no tick can run; the tick is deferred until it returns,
    /// and the readiness listener comes off until then rather than firing
    /// every host turn.
    ///
    /// A task may let go of the loop while it is ticked. The callback holds
    /// the `Host` until the tick returns, so the loop then detaches and
    /// shuts down here, outside the tick.
    fn host_tick(self: Rc<Self>) {
        if Driving::any() {
            let _ = self.listen(false);
            DEFERRED.with(|deferred| deferred.borrow_mut().push(self.this.clone()));
            return;
        }
        self.tick();
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        self.disarm();
        if let Some(id) = self.immediate.take() {
            // SAFETY: a pending immediate armed in `schedule`.
            unsafe { emscripten_clear_immediate(id) };
        }
        let _ = self.listen(false);
        self.hold.detach();
        // The last hold may go inside another loop's tick or a `block_on`,
        // where dropping a runtime panics as a blocking wait.
        // SAFETY: taken once, here.
        let rt = unsafe { ManuallyDrop::take(self.rt.get_mut()) };
        rt.shutdown_background();
    }
}

/// The readiness listener: the set has uncollected events.
extern "C" fn ready(_epfd: i32, user_data: *mut c_void) {
    // SAFETY: `user_data` is the `Host` that registered this listener, which
    // `drop` removes.
    let host = unsafe { Host::from_callback(user_data) };
    host.host_tick();
}

/// The deadline timer fired.
extern "C" fn deadline(user_data: *mut c_void) {
    // SAFETY: as `ready`; the timeout is cleared in `drop`.
    let host = unsafe { Host::from_callback(user_data) };
    host.deadline.set(None);
    host.host_tick();
}

/// The follow-up for ready work.
extern "C" fn immediate(user_data: *mut c_void) {
    // SAFETY: as `ready`; the immediate is cleared in `drop`.
    let host = unsafe { Host::from_callback(user_data) };
    host.immediate.set(None);
    host.host_tick();
}
