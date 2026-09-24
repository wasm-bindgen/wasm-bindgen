#![cfg(all(target_arch = "wasm32", target_os = "emscripten"))]

extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_emscripten);

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    #[allow(dead_code)]
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut() + std::panic::UnwindSafe,
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, clear its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.token);
    }
}

// Keep logging "hello" every second until the resulting `Interval` is dropped.
#[wasm_bindgen]
pub async fn hello() -> Interval {
    Interval::new(10, || log("hello"))
}

#[wasm_bindgen_test]
async fn hello_test() {
    hello().await;
}

// Compiles the `experimental_tokio` attribute expansion: an async export
// driven on the ambient event-loop runtime rather than the wasm-bindgen
// executor.
#[cfg(wasm_bindgen_unstable_tokio)]
#[wasm_bindgen(experimental_tokio)]
pub async fn tokio_sleep_ms(ms: u32) -> u32 {
    tokio::time::sleep(std::time::Duration::from_millis(ms as u64)).await;
    ms
}

// Two independently scheduled roots (as two `experimental_tokio` exports
// would be) must land on one runtime: a oneshot crosses between them, both
// use timers, and one spawns a subtask. NOTE: the emscripten harness is
// check-only today (it never instantiates the module), so this body is
// compile coverage until the harness executes tests.
#[cfg(wasm_bindgen_unstable_tokio)]
#[wasm_bindgen_test]
async fn tokio_ambient_runtime_is_shared() {
    use std::time::Duration;
    use tokio::sync::oneshot;
    use wasm_bindgen_futures::tokio::schedule;

    let (cross_tx, cross_rx) = oneshot::channel::<u32>();
    let (a_tx, a_rx) = oneshot::channel::<u32>();
    let (b_tx, b_rx) = oneshot::channel::<u32>();

    schedule(
        async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            cross_tx.send(7).unwrap();
            1u32
        },
        move |out| {
            a_tx.send(out.unwrap()).unwrap();
        },
    );

    schedule(
        async move {
            let v = cross_rx.await.unwrap();
            tokio::spawn(async move { v + 1 }).await.unwrap()
        },
        move |out| {
            b_tx.send(out.unwrap()).unwrap();
        },
    );

    assert_eq!(a_rx.await.unwrap(), 1);
    assert_eq!(b_rx.await.unwrap(), 8);
}
