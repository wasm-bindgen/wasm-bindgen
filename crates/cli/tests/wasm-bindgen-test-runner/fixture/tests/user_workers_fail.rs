//! Browser tests whose spawned workers log and then fail, run without
//! `--nocapture` so the worker logs must show up in the failure output.

mod common;

use common::{blob_url, first_message};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{ErrorEvent, SharedWorker, Worker};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn dedicated_worker_logs_then_fails() {
    let url = blob_url(
        r#"
        console.log("SPAWNED_WORKER_FAILURE_TEST_MARKER_8Y3M4");
        throw new Error("Intentional worker failure");
        "#,
    );
    let worker = Worker::new(&url).unwrap();
    // Rejects with the worker's error message once it throws.
    let error = Promise::new(&mut |_resolve, reject| {
        let onerror = Closure::once_into_js(move |e: ErrorEvent| {
            reject.call1(&JsValue::NULL, &e.message().into()).unwrap();
        });
        worker.set_onerror(Some(onerror.unchecked_ref()));
    });
    JsFuture::from(error).await.unwrap();
}

#[wasm_bindgen_test]
async fn shared_worker_logs_then_fails() {
    // A `SharedWorker`'s `error` event doesn't fire for exceptions thrown in
    // its `connect` handler, so waiting for one would hang until the runner
    // times out. Instead the worker posts once it has logged, and the test
    // fails then. Forwarded console messages travel over the same port, so
    // the log has already arrived.
    let url = blob_url(
        r#"
        onconnect = function(e) {
            console.log("SPAWNED_SHARED_WORKER_FAILURE_MARKER_5K9N2");
            e.ports[0].postMessage("logged");
            throw new Error("Intentional shared worker failure");
        };
        "#,
    );
    let worker = SharedWorker::new(&url).unwrap();
    let port = worker.port();
    let result = first_message("shared worker log", |onmessage, _| {
        port.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("logged"));
    panic!("shared worker failed after logging");
}
