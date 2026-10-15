use crate::Promise;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::panic::AssertUnwindSafe;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

#[wasm_bindgen]
extern "C" {
    // The `run_all` closure as a JS function, or that function bound to the
    // sequence number of one scheduled run.
    type RunAll;

    #[wasm_bindgen(method, js_name = bind)]
    fn bind_seq(this: &RunAll, this_arg: &JsValue, seq: u32) -> RunAll;

    #[wasm_bindgen]
    fn queueMicrotask(run_all: &RunAll);

    #[wasm_bindgen(method, js_name = then)]
    fn then_run_all(this: &Promise, run_all: &RunAll) -> Promise;

    type Global;

    #[wasm_bindgen(method, getter, js_name = queueMicrotask)]
    fn hasQueueMicrotask(this: &Global) -> JsValue;
}

struct QueueState {
    // The queue of Tasks which are to be run in order. In practice this is all the
    // synchronous work of futures, and each `Task` represents calling `poll` on
    // a future "at the right time". Each task carries the sequence number of the
    // `run_all` microtask that was queued for it.
    tasks: RefCell<VecDeque<(u32, Rc<crate::futures::task::Task>)>>,

    // The sequence number of the most recently scheduled task.
    last_seq: Cell<u32>,
}

impl QueueState {
    fn run_all(&self, seq: u32) {
        // Run the tasks scheduled up to and including the one this microtask was
        // queued for, in order. A task scheduled after it, including one woken
        // while these tasks run, has a microtask of its own still to come, and
        // runs on that later tick. If earlier microtasks were discarded (see
        // `schedule_task`), their tasks are still here, ahead of this one, and
        // run now.
        loop {
            let task = {
                let mut tasks = self.tasks.borrow_mut();
                match tasks.front() {
                    // Wrapping comparison: `task_seq` is not after `seq`. It is
                    // sound because the sequence numbers from the front task's
                    // to `last_seq` all belong to tasks still in this queue, so
                    // they span far fewer than 2^31 values.
                    Some(&(task_seq, _)) if seq.wrapping_sub(task_seq) as i32 >= 0 => {
                        tasks.pop_front().unwrap().1
                    }
                    _ => break,
                }
            };
            task.run();
        }
    }
}

pub(crate) struct Queue {
    state: Rc<QueueState>,
    promise: Promise,
    // `Fn`, not `FnMut`: the JS glue for an `FnMut` closure clears the closure
    // pointer for the duration of each call and restores it in a `finally`.
    // A terminated execution never runs that `finally`, so after a termination
    // inside `run_all` every later call would throw "closure invoked
    // recursively or after being dropped". `run_all` only needs `&self`.
    closure: Closure<dyn Fn(u32) -> Result<(), JsError>>,
    has_queue_microtask: bool,
}

impl Queue {
    // Schedule a task to run on the next tick
    pub(crate) fn schedule_task(&self, task: Rc<crate::futures::task::Task>) {
        // Every task gets a `run_all` microtask of its own, tagged with a
        // sequence number, instead of one microtask per batch behind a flag. A
        // host that terminates execution inside a microtask checkpoint (V8's
        // `TerminateExecution`, as used for CPU-time limits) discards every
        // pending microtask, and nothing tells the executor so. A flag saying a
        // `run_all` is pending would then stay set with no `run_all` left to
        // clear it, and no task would run again. Here the next task scheduled
        // after such a loss queues its own microtask, whose `run_all` also runs
        // the tasks left behind. The sequence number keeps every `run_all` to
        // the tasks scheduled before it, so each microtask runs about one task
        // and pending microtasks never outnumber queued tasks. A task is queued
        // at most once until it next runs, because `Task::wake` dedupes on
        // `is_queued`.
        let seq = self.state.last_seq.get().wrapping_add(1);
        self.state.last_seq.set(seq);
        self.state.tasks.borrow_mut().push_back((seq, task));
        let run_all = self
            .closure
            .as_ref()
            .unchecked_ref::<RunAll>()
            .bind_seq(&JsValue::UNDEFINED, seq);
        // Use queueMicrotask to execute as soon as possible. If it does not exist
        // fall back to the promise resolution
        if self.has_queue_microtask {
            queueMicrotask(&run_all);
        } else {
            let _ = self.promise.then_run_all(&run_all);
        }
    }
    // Append a task to the currently running queue, or schedule it
    #[cfg(not(target_feature = "atomics"))]
    pub(crate) fn push_task(&self, task: Rc<crate::futures::task::Task>) {
        // It would make sense to run this task on the same tick.  For now, we
        // make the simplifying choice of always scheduling tasks for a future tick.
        self.schedule_task(task)
    }
}

impl Queue {
    fn new() -> Self {
        let state = Rc::new(QueueState {
            tasks: RefCell::new(VecDeque::new()),
            last_seq: Cell::new(0),
        });

        let has_queue_microtask = crate::global()
            .unchecked_into::<Global>()
            .hasQueueMicrotask()
            .is_function();

        Self {
            promise: Promise::resolve(&JsValue::undefined()),

            closure: {
                let state = AssertUnwindSafe(Rc::clone(&state));

                // This closure will only be called on the next microtask event
                // tick, bound to the sequence number of the task it was queued
                // for.
                Closure::new(move |seq| {
                    state.run_all(seq);
                    Ok(())
                })
            },

            state,
            has_queue_microtask,
        }
    }

    pub(crate) fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        use wasm_bindgen::__rt::LazyCell;

        #[cfg_attr(target_feature = "atomics", thread_local)]
        static QUEUE: LazyCell<Queue> = LazyCell::new(Queue::new);

        f(&QUEUE)
    }
}
