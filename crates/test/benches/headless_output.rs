/// Benchmark for headless browser output performance.
///
/// This benchmark measures the time it takes to append output in the browser,
/// testing the O(1) append optimization and display:none layout skip.
///
/// Run with:
///   cargo bench --bench headless_output --target wasm32-unknown-unknown -p wasm-bindgen-test
use wasm_bindgen_test::{wasm_bindgen_bench, wasm_bindgen_test_configure, Criterion};

wasm_bindgen_test_configure!(run_in_browser);

/// Sleep for the given number of milliseconds
async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

/// Pre-warm the console with `kb` kilobytes of log data, yielding periodically
async fn prewarm(kb: usize) {
    let msg: wasm_bindgen::JsValue = "x".repeat(100).into(); // 100 bytes per message
    let n = kb * 10; // 10 messages per KB
    for i in 0..n {
        web_sys::console::log_1(&msg);
        //give the browser a chance to breathe
        if i % 100 == 0 {
            sleep_ms(0).await;
        }
    }
}

fn performance_now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

#[wasm_bindgen_bench]
async fn bench_console_log_1mb(c: &mut Criterion) {
    use std::time::Duration;

    let msg: wasm_bindgen::JsValue = "y".repeat(100).into();
    prewarm(1_000).await;
    c.bench_async_function("console_log_after_1mb", |b| {
        let msg = msg.clone();
        Box::pin(async move {
            b.iter_custom_future(|iters| {
                let msg = msg.clone();
                let mut value = 0;
                async move {
                    let mut elapsed = Duration::ZERO;
                    for _ in 0..iters {
                        let start = std::hint::black_box(performance_now());
                        std::hint::black_box(web_sys::console::log_1(&msg));
                        let elapsed_ms = std::hint::black_box(performance_now()) - start;
                        elapsed += Duration::from_secs_f64(elapsed_ms / 1000.0);
                        // Track iterations to yield periodically, letting the browser process events
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
}
