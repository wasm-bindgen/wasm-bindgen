#![cfg(target_arch = "wasm32")]

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;

// ---------------------------------------------------------------------------
// Atomics / multithread-specific tests
//
// These live here rather than in js-sys because they reference
// wait_async_mock.js which is local to this crate.
// ---------------------------------------------------------------------------

#[cfg(target_feature = "atomics")]
use std::future::Future;
#[cfg(target_feature = "atomics")]
use std::pin::Pin;
#[cfg(target_feature = "atomics")]
use std::task::{Context, Poll};

#[cfg(target_feature = "atomics")]
#[wasm_bindgen(module = "/tests/wait_async_mock.js")]
extern "C" {
    #[wasm_bindgen(js_name = installNotifyOnlyWaitAsyncMock)]
    fn install_wait_async_mock() -> JsValue;

    #[wasm_bindgen(js_name = restoreWaitAsyncMock)]
    fn restore_wait_async_mock(original: &JsValue);
}

#[cfg(target_feature = "atomics")]
struct WaitAsyncMockGuard {
    original: JsValue,
}

#[cfg(target_feature = "atomics")]
impl Drop for WaitAsyncMockGuard {
    fn drop(&mut self) {
        restore_wait_async_mock(&self.original);
    }
}

#[cfg(target_feature = "atomics")]
#[derive(Default)]
struct PendingThenReady {
    polled: bool,
}

#[cfg(target_feature = "atomics")]
impl Future for PendingThenReady {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.polled = true;
            Poll::Pending
        }
    }
}

// Reproduces a race condition where the waitAsync promise resolves without
// first transitioning the task state back to AWAKE. Without the fix this
// panics inside `Task::run` because it observes the `SLEEPING` state on entry.
#[cfg(target_feature = "atomics")]
#[wasm_bindgen_test(async)]
async fn wait_async_promise_callback_runs_without_wake() {
    use futures_channel::oneshot;

    let _guard = WaitAsyncMockGuard {
        original: install_wait_async_mock(),
    };

    let (done_tx, done_rx) = oneshot::channel::<()>();
    spawn_local(async move {
        PendingThenReady::default().await;
        done_tx.send(()).ok();
    });

    done_rx.await.expect("task finished");
}

// ---------------------------------------------------------------------------
// wasm-bindgen-test infrastructure tests (panic / ignore attributes)
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
#[should_panic]
async fn should_panic() {
    panic!()
}

#[wasm_bindgen_test]
#[should_panic = "error message"]
async fn should_panic_string() {
    panic!("error message")
}

#[wasm_bindgen_test]
#[should_panic(expected = "error message")]
async fn should_panic_expected() {
    panic!("error message")
}

#[wasm_bindgen_test]
#[ignore]
async fn ignore() {
    panic!("this test should have been ignored")
}

#[wasm_bindgen_test]
#[ignore = "reason"]
async fn ignore_reason() {
    panic!("this test should have been ignored")
}
