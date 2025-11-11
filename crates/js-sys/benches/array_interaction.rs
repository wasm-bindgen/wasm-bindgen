use js_sys::Uint8Array;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen_bench]
fn bench_array_interaction(c: &mut Criterion) {
    c.bench_function("bench_to_uint8_array", |b| {
        b.iter(|| Uint8Array::new_from_slice(&[0; 4096]))
    })
    .bench_function("bench_to_vec", |b| {
        b.iter(|| Uint8Array::new_with_length(4096).to_vec())
    });
}
