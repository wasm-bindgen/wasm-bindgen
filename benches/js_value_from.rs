use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen_bench]
fn bench_js_value_from(c: &mut Criterion) {
    c.bench_function("bench_js_value_from_str", |b| {
        b.iter(|| JsValue::from_str("42"));
    });
}
