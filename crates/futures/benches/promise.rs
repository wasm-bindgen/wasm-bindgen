use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen_bench]
async fn bench_promise(c: &mut Criterion) {
    c.bench_async_function("bench_promise_to_future", |b| {
        let f = b.iter_future(|| JsFuture::from(Promise::resolve(&JsValue::from(42))));
        Box::pin(f)
    })
    .await;
}
