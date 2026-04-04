use futures_channel::oneshot;
use js_sys::{futures::future_to_promise, futures::spawn_local, futures::JsFuture, Promise};
use std::ops::FnMut;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

// ---------------------------------------------------------------------------
// IntoFuture — direct promise.await
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn promise_await_resolve() {
    let p = Promise::resolve(&JsValue::from(42));
    let x = p.await.unwrap();
    assert_eq!(x, 42);
}

#[wasm_bindgen_test]
async fn promise_await_reject() {
    let p = Promise::<JsValue>::reject(&JsValue::from(42));
    let e = p.await.unwrap_err();
    assert_eq!(e, 42);
}

#[wasm_bindgen_test]
async fn typed_promise_await() {
    use js_sys::Number;
    let p: Promise<Number> = Promise::resolve(&Number::from(99.0));
    let n: Number = p.await.unwrap();
    assert_eq!(n.value_of(), 99.0);
}

// ---------------------------------------------------------------------------
// JsFuture
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn promise_resolve_is_ok_future() {
    let p = Promise::resolve(&JsValue::from(42));
    let x = JsFuture::from(p).await.unwrap();
    assert_eq!(x, 42);
}

#[wasm_bindgen_test]
async fn promise_reject_is_error_future() {
    let p = Promise::<JsValue>::reject(&JsValue::from(42));
    let e = JsFuture::from(p).await.unwrap_err();
    assert_eq!(e, 42);
}

#[wasm_bindgen_test]
fn debug_jsfuture() {
    let p = Promise::resolve(&JsValue::from(42));
    let f = JsFuture::from(p);
    assert_eq!(&format!("{:?}", f), "JsFuture { ... }");
}

#[wasm_bindgen_test]
async fn can_create_multiple_futures_from_same_promise() {
    let promise = Promise::resolve(&JsValue::null());
    let a = JsFuture::from(promise.clone());
    let b = JsFuture::from(promise);
    a.await.unwrap();
    b.await.unwrap();
}

// ---------------------------------------------------------------------------
// future_to_promise
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn ok_future_is_resolved_promise() {
    let p = future_to_promise(async { Ok(JsValue::from(42)) });
    let x = JsFuture::from(p).await.unwrap();
    assert_eq!(x, 42);
}

#[wasm_bindgen_test]
async fn error_future_is_rejected_promise() {
    let p = future_to_promise(async { Err(JsValue::from(42)) });
    let e = JsFuture::from(p).await.unwrap_err();
    assert_eq!(e, 42);
}

// ---------------------------------------------------------------------------
// spawn_local
// ---------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    fn setTimeout(c: &Closure<dyn FnMut()>);
}

#[wasm_bindgen_test]
async fn oneshot_works() {
    let (tx, rx) = oneshot::channel::<u32>();
    let mut tx = Some(tx);
    let closure = Closure::wrap(Box::new(move || {
        drop(tx.take().unwrap());
    }) as Box<dyn FnMut()>);
    setTimeout(&closure);
    closure.forget();
    rx.await.unwrap_err();
}

#[wasm_bindgen_test]
async fn spawn_local_runs() {
    let (tx, rx) = oneshot::channel::<u32>();
    spawn_local(async {
        tx.send(42).unwrap();
    });
    assert_eq!(rx.await.unwrap(), 42);
}

// Uses promise.then() which has different signatures under stable vs unstable APIs.
// Only run under stable to avoid the unstable overload mismatch.
#[cfg(not(js_sys_unstable_apis))]
#[wasm_bindgen_test]
async fn spawn_local_nested() {
    let (ta, mut ra) = oneshot::channel::<u32>();
    let (ts, rs) = oneshot::channel::<u32>();
    let (tx, rx) = oneshot::channel::<u32>();
    let promise = Promise::resolve(&JsValue::null());

    spawn_local(async move {
        let inbetween = Closure::wrap(Box::new(move |_| {
            assert_eq!(
                ra.try_recv().unwrap(),
                None,
                "Nested task should not have run yet"
            );
        }) as Box<dyn FnMut(JsValue)>);
        let inbetween = promise.then(&inbetween);
        spawn_local(async {
            ta.send(0xdead).unwrap();
            ts.send(0xbeaf).unwrap();
        });
        JsFuture::from(inbetween).await.unwrap();
        assert_eq!(
            rs.await.unwrap(),
            0xbeaf,
            "Nested task should run eventually"
        );
        tx.send(42).unwrap();
    });

    assert_eq!(rx.await.unwrap(), 42);
}

#[wasm_bindgen_test]
async fn spawn_local_err_no_exception() {
    let (tx, rx) = oneshot::channel::<u32>();
    spawn_local(async {});
    spawn_local(async {
        tx.send(42).unwrap();
    });
    assert_eq!(rx.await.unwrap(), 42);
}

// ---------------------------------------------------------------------------
// Atomics / multithread-specific tests
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
// JsStream (requires futures-core-03-stream feature)
// ---------------------------------------------------------------------------

#[cfg(feature = "futures-core-03-stream")]
#[wasm_bindgen_test]
async fn can_use_an_async_iterable_as_stream() {
    use futures_lite::stream::StreamExt;
    use js_sys::futures::stream::JsStream;

    let async_iter = js_sys::Function::new_no_args(
        "return async function*() {
            yield 42;
            yield 24;
        }()",
    )
    .call0(&JsValue::undefined())
    .unwrap()
    .unchecked_into::<js_sys::AsyncIterator>();

    let mut stream = JsStream::from(async_iter);
    assert_eq!(stream.next().await, Some(Ok(JsValue::from(42))));
    assert_eq!(stream.next().await, Some(Ok(JsValue::from(24))));
    assert_eq!(stream.next().await, None);
}
