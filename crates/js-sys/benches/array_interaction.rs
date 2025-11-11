use js_sys::Uint8Array;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

#[wasm_bindgen_bench]
fn bench_array_interaction(c: &mut Criterion) {
    c.iter("[u8] to Uint8Array", || {
        Uint8Array::new_from_slice(&[0; 4096])
    });
}
