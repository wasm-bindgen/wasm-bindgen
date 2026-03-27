/// Benchmark for worker log forwarding performance in the headless browser runner.
///
/// Run with:
///   cargo bench --bench headless_worker_output --target wasm32-unknown-unknown -p wasm-bindgen-test
use std::time::Duration;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_bench, wasm_bindgen_test_configure, Criterion};

wasm_bindgen_test_configure!(run_in_browser);

async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

async fn prewarm(kb: usize) {
    let msg: wasm_bindgen::JsValue = "x".repeat(100).into();
    let n = kb * 10;
    for i in 0..n {
        web_sys::console::log_1(&msg);
        if i % 100 == 0 {
            sleep_ms(0).await;
        }
    }
}

fn performance_now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

fn create_dedicated_worker(script: &str) -> (web_sys::Worker, String) {
    use js_sys::Array;
    use web_sys::{Blob, BlobPropertyBag, Url, Worker};

    let parts = Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(script));
    let opts = BlobPropertyBag::new();
    opts.set_type("application/javascript");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &opts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let worker = Worker::new(&url).unwrap();
    (worker, url)
}

async fn benchmark_worker_log(worker: &web_sys::Worker) {
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let resolve_clone = resolve.clone();
        let onmessage = Closure::once_into_js(move |_e: web_sys::MessageEvent| {
            resolve_clone.call0(&wasm_bindgen::JsValue::NULL).unwrap();
        });
        worker.set_onmessage(Some(onmessage.unchecked_ref()));

        let reject_clone = reject.clone();
        let onerror = Closure::once_into_js(move |e: web_sys::ErrorEvent| {
            reject_clone
                .call1(
                    &wasm_bindgen::JsValue::NULL,
                    &wasm_bindgen::JsValue::from_str(&e.message()),
                )
                .unwrap();
        });
        worker.set_onerror(Some(onerror.unchecked_ref()));

        worker
            .post_message(&wasm_bindgen::JsValue::UNDEFINED)
            .unwrap();
    });

    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_bench]
async fn bench_worker_console_log_batch(c: &mut Criterion) {
    prewarm(1_000).await;

    let (worker, url) = create_dedicated_worker(
        r#"
            onmessage = function() {
                console.log("worker-bench-log");
                postMessage("done");
            };
        "#,
    );

    c.bench_async_function("worker_console_log_after_1mb", |b| {
        let worker = worker.clone();
        Box::pin(async move {
            b.iter_custom_future(|iters| {
                let worker = worker.clone();
                let mut value = 0;
                async move {
                    let mut elapsed = Duration::ZERO;
                    for _ in 0..iters {
                        let start = std::hint::black_box(performance_now());
                        benchmark_worker_log(&worker).await;
                        let elapsed_ms = std::hint::black_box(performance_now()) - start;
                        elapsed += Duration::from_secs_f64(elapsed_ms / 1000.0);
                        value += 1;
                        if value % 100 == 0 {
                            sleep_ms(0).await;
                        }
                    }
                    elapsed
                }
            })
            .await;
        })
    })
    .await;

    worker.terminate();
    web_sys::Url::revoke_object_url(&url).unwrap();
}
