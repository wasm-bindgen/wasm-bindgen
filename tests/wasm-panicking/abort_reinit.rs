use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
#[wasm_bindgen(module = "tests/wasm/abort_reinit.js")]
extern "C" {
    fn catch_panic();
    fn expect_reinit();
}

#[wasm_bindgen]
pub fn maybe_panic(should_panic: bool) -> u32 {
    if should_panic {
        panic!()
    }
    42
}

static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn increment_and_get() -> u32 {
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[wasm_bindgen_test]
#[ignore]
#[cfg(true)]
fn abort_reinit() {
    //catch_panic();
    //expect_reinit();
}
