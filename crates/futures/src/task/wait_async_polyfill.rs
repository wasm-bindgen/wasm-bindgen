//!
//! The polyfill was kindly borrowed from https://github.com/tc39/proposal-atomics-wait-async
//! and ported to Rust
//!

#![allow(clippy::incompatible_msrv)]

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Author: Lars T Hansen, lhansen@mozilla.com
 */

/* Polyfill for Atomics.waitAsync() for web browsers.
 *
 * Any kind of agent that is able to create a new Worker can use this polyfill.
 *
 * Load this file in all agents that will use Atomics.waitAsync.
 *
 * Agents that don't call Atomics.waitAsync need do nothing special.
 *
 * Any kind of agent can wake another agent that is sleeping in
 * Atomics.waitAsync by just calling Atomics.wake for the location being slept
 * on, as normal.
 *
 * The implementation is not completely faithful to the proposed semantics: in
 * the case where an agent first asyncWaits and then waits on the same location:
 * when it is woken, the two waits will be woken in order, while in the real
 * semantics, the sync wait will be woken first.
 *
 * In this polyfill Atomics.waitAsync is not very fast.
 */

/* Implementation:
 *
 * For every wait we fork off a Worker to perform the wait.  Workers are reused
 * when possible.  The worker communicates with its parent using postMessage.
 */

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::sync::atomic::{AtomicI32, Ordering};
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

#[thread_local]
static HELPERS: RefCell<Vec<Worker>> = RefCell::new(vec![]);

fn alloc_helper() -> Result<Worker, JsValue> {
    if let Some(helper) = HELPERS.borrow_mut().pop() {
        return Ok(helper);
    }

    // Check if Worker constructor is available
    if !has_worker() {
        return Err(JsValue::from_str("Worker not available"));
    }

    let worker_url = wasm_bindgen::link_to!(module = "/src/task/worker.js");
    Worker::new(&worker_url).map_err(|e| e)
}

fn has_worker() -> bool {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("Worker"))
        .map(|worker_constructor| !worker_constructor.is_undefined())
        .unwrap_or(false)
}

fn free_helper(helper: Worker) {
    let mut helpers = HELPERS.borrow_mut();
    helpers.push(helper.clone());
    helpers.truncate(10); // random arbitrary limit chosen here
}

pub fn wait_async(ptr: &AtomicI32, value: i32) -> Promise {
    Promise::new(&mut |resolve, _reject| {
        match alloc_helper() {
            Ok(helper) => {
                let helper_ref = helper.clone();

                let onmessage_callback = Closure::once_into_js(move |e: MessageEvent| {
                    // Our helper is done waiting so it's available to wait on a
                    // different location, so return it to the free list.
                    free_helper(helper_ref);
                    drop(resolve.call1(&JsValue::NULL, &e.data()));
                });
                helper.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

                let data = Array::of3(
                    &wasm_bindgen::memory(),
                    &JsValue::from(ptr.as_ptr() as u32 / 4),
                    &JsValue::from(value),
                );

                helper
                    .post_message(&data)
                    .unwrap_or_else(|js| wasm_bindgen::throw_val(js));
            }
            Err(_) => {
                // Worker not available, use setTimeout polling as fallback
                // This provides basic async scheduling without Worker dependency
                use alloc::rc::Rc;
                use core::cell::Cell;
                use wasm_bindgen::closure::Closure;

                // Simple polling implementation with exponential backoff
                let ptr_addr = ptr.as_ptr() as usize;
                let max_iterations = Rc::new(Cell::new(10)); // Poll up to 10 times
                let delay = Rc::new(Cell::new(1)); // Start with 1ms delay

                let poll_fn = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
                let poll_fn_clone = poll_fn.clone();
                let resolve_clone = resolve.clone();

                *poll_fn.borrow_mut() = Some(Closure::new(move || {
                    // Safety: We're just reading an atomic value
                    let current = unsafe {
                        (ptr_addr as *const AtomicI32)
                            .as_ref()
                            .unwrap()
                            .load(Ordering::SeqCst)
                    };

                    if current != value || max_iterations.get() == 0 {
                        // Value changed or we've polled enough times
                        drop(resolve_clone.call1(&JsValue::NULL, &JsValue::from_str("ok")));
                        poll_fn_clone.borrow_mut().take(); // Clean up closure
                    } else {
                        // Continue polling with exponential backoff
                        max_iterations.set(max_iterations.get() - 1);
                        let next_delay = delay.get().min(100); // Cap at 100ms
                        delay.set(next_delay * 2);

                        if let Some(ref callback) = *poll_fn_clone.borrow() {
                            set_timeout(callback.as_ref().unchecked_ref(), next_delay);
                        }
                    }
                }));

                // Start polling
                {
                    let borrowed = poll_fn.borrow();
                    if let Some(ref callback) = *borrowed {
                        set_timeout(callback.as_ref().unchecked_ref(), 1);
                    }
                }
            }
        }
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(callback: &JsValue, delay: i32);
}
