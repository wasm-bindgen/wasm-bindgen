//! Browser tests that spawn their own workers, run with `--nocapture`. Every
//! test passes and logs markers unique to it, so the harness checks each
//! test's markers in the output of a single run.

mod common;

use common::{blob_url, first_message};
use wasm_bindgen_test::*;
use web_sys::{SharedWorker, Url, Worker, WorkerOptions, WorkerType};

wasm_bindgen_test_configure!(run_in_browser);

fn module_worker_options() -> WorkerOptions {
    let options = WorkerOptions::new();
    options.set_type(WorkerType::Module);
    options
}

/// Logs at every console level from a blob dedicated worker.
#[wasm_bindgen_test]
async fn dedicated_worker_logs() {
    let url = blob_url(
        r#"
        console.debug("DEDICATED_WORKER_DEBUG_MARKER_7X9K2");
        console.log("DEDICATED_WORKER_LOG_MARKER_7X9K2");
        console.info("DEDICATED_WORKER_INFO_MARKER_7X9K2");
        console.warn("DEDICATED_WORKER_WARN_MARKER_7X9K2");
        console.error("DEDICATED_WORKER_ERROR_MARKER_7X9K2");
        postMessage("done");
        "#,
    );
    let worker = Worker::new(&url).unwrap();
    let result = first_message("worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
    Url::revoke_object_url(&url).unwrap();
}

/// A user `addEventListener("message", ...)` handler coexists with console
/// forwarding.
#[wasm_bindgen_test]
async fn add_event_listener_coexists() {
    let url = blob_url(
        r#"
        console.log("DEDICATED_WORKER_ADD_EVENT_LISTENER_MARKER_5J3R8");
        postMessage("done");
        "#,
    );
    let worker = Worker::new(&url).unwrap();
    let result = first_message("worker completion", |onmessage, _| {
        worker
            .add_event_listener_with_callback("message", onmessage)
            .unwrap()
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
    Url::revoke_object_url(&url).unwrap();
}

/// Logs at every console level from a blob module worker.
#[wasm_bindgen_test]
async fn module_worker_logs() {
    let url = blob_url(
        r#"
        console.debug("MODULE_WORKER_DEBUG_MARKER_4M8N3");
        console.log("MODULE_WORKER_LOG_MARKER_4M8N3");
        console.info("MODULE_WORKER_INFO_MARKER_4M8N3");
        console.warn("MODULE_WORKER_WARN_MARKER_4M8N3");
        console.error("MODULE_WORKER_ERROR_MARKER_4M8N3");
        postMessage("done");
        "#,
    );
    let worker = Worker::new_with_options(&url, &module_worker_options()).unwrap();
    let result = first_message("module worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
    Url::revoke_object_url(&url).unwrap();
}

/// A module worker created from a served URL rather than a blob, which goes
/// through the `await import(...)` wrapper.
#[wasm_bindgen_test]
async fn url_module_worker_logs() {
    let worker =
        Worker::new_with_options("module_worker_script.js", &module_worker_options()).unwrap();
    let result = first_message("URL module worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
}

/// A classic worker created from a served URL rather than a blob, which goes
/// through the `importScripts` wrapper.
#[wasm_bindgen_test]
async fn url_worker_logs() {
    let worker = Worker::new("worker_script.js").unwrap();
    let result = first_message("URL worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
}

/// Logging a non-cloneable value doesn't crash the worker.
#[wasm_bindgen_test]
async fn worker_non_cloneable_log_does_not_crash() {
    let url = blob_url(
        r#"
        console.log(function nonCloneable() {});
        postMessage("after-log");
        "#,
    );
    let worker = Worker::new(&url).unwrap();
    let result = first_message("worker completion", |onmessage, onerror| {
        worker.set_onmessage(Some(onmessage));
        worker.set_onerror(Some(onerror));
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("after-log"));
    worker.terminate();
    Url::revoke_object_url(&url).unwrap();
}

/// Logs from a worker spawned by a worker are forwarded too.
#[wasm_bindgen_test]
async fn nested_worker_logs() {
    let url = blob_url(
        r#"
        const innerBlob = new Blob(
            ['onmessage=function(){console.log("NESTED_WORKER_LOG_MARKER_2P6L1");postMessage("done")};postMessage("ready")'],
            { type: "application/javascript" }
        );
        const inner = new Worker(URL.createObjectURL(innerBlob));
        inner.onmessage = function(e) {
            if (e.data === "ready") {
                inner.postMessage("start");
                return;
            }
            postMessage(e.data);
            inner.terminate();
        };
        "#,
    );
    let worker = Worker::new(&url).unwrap();
    let result = first_message("nested worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
    Url::revoke_object_url(&url).unwrap();
}

/// The non-blob `Worker` wrapper must not splice the URL into its bootstrap
/// source. Escaped correctly, this URL evaluates as the worker script
/// `postMessage("done")//");console.log("DATA_URL_INJECTED_MARKER_6R2M8");//`,
/// which logs nothing; a vulnerable wrapper runs the `console.log`.
#[wasm_bindgen_test]
async fn data_url_worker_does_not_inject() {
    let url = r#"data:text/javascript,postMessage("done")//");console.log("DATA_URL_INJECTED_MARKER_6R2M8");//"#;
    let worker = Worker::new(url).unwrap();
    let result = first_message("data URL worker completion", |onmessage, _| {
        worker.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    worker.terminate();
}

/// Logs at every console level from a shared worker's `connect` handler.
#[wasm_bindgen_test]
async fn shared_worker_logs() {
    let url = blob_url(
        r#"
        onconnect = function(e) {
            console.debug("SPAWNED_SHARED_WORKER_DEBUG_MARKER_3M8P1");
            console.log("SPAWNED_SHARED_WORKER_LOG_MARKER_3M8P1");
            console.info("SPAWNED_SHARED_WORKER_INFO_MARKER_3M8P1");
            console.warn("SPAWNED_SHARED_WORKER_WARN_MARKER_3M8P1");
            console.error("SPAWNED_SHARED_WORKER_ERROR_MARKER_3M8P1");
            e.ports[0].postMessage("done");
        };
        "#,
    );
    let worker = SharedWorker::new(&url).unwrap();
    let port = worker.port();
    let result = first_message("shared worker completion", |onmessage, _| {
        port.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    Url::revoke_object_url(&url).unwrap();
}

/// Shared worker logs emitted before the first port connects are still
/// forwarded.
#[wasm_bindgen_test]
async fn shared_worker_top_level_logs() {
    let url = blob_url(
        r#"
        console.log("SPAWNED_SHARED_WORKER_TOP_LEVEL_MARKER_7Q2H5");
        onconnect = function(e) {
            e.ports[0].postMessage("done");
        };
        "#,
    );
    let worker = SharedWorker::new(&url).unwrap();
    let port = worker.port();
    let result = first_message("shared worker completion", |onmessage, _| {
        port.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
    Url::revoke_object_url(&url).unwrap();
}

/// Logging a non-cloneable value doesn't crash the shared worker.
#[wasm_bindgen_test]
async fn shared_worker_non_cloneable_log_does_not_crash() {
    let url = blob_url(
        r#"
        onconnect = function(e) {
            const port = e.ports[0];
            console.log(function nonCloneable() {});
            port.postMessage("after-log");
        };
        "#,
    );
    let worker = SharedWorker::new(&url).unwrap();
    let port = worker.port();
    let result = first_message("shared worker completion", |onmessage, onerror| {
        port.set_onmessage(Some(onmessage));
        worker.set_onerror(Some(onerror));
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("after-log"));
    Url::revoke_object_url(&url).unwrap();
}

/// The non-blob `SharedWorker` wrapper must not splice the URL into its
/// bootstrap source. Escaped correctly, this URL is an inert shared worker
/// script; a vulnerable wrapper registers a `connect` listener that logs the
/// marker.
#[wasm_bindgen_test]
async fn data_url_shared_worker_does_not_inject() {
    let url = r#"data:text/javascript,addEventListener("connect",e=>e.ports[0].postMessage("done"))//");addEventListener("connect",()=>console.log("SHARED_URL_INJECTED_MARKER_4T1Q6"));//"#;
    let worker = SharedWorker::new(url).unwrap();
    let port = worker.port();
    port.start();
    let result = first_message("data URL shared worker completion", |onmessage, _| {
        port.set_onmessage(Some(onmessage))
    })
    .await;
    assert_eq!(result.as_string().as_deref(), Some("done"));
}
