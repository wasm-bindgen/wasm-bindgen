// FLAGS: --target=module --experimental-reset-state-function
// FLAGS: --target=nodejs --experimental-reset-state-function
// FLAGS: --target=web --experimental-reset-state-function

use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen(pre_reinit_hook)]
pub fn my_pre_reinit() {
    // Called on the old instance before teardown
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

#[wasm_bindgen(post_reinit_hook)]
pub fn my_post_reinit() {
    // Called on the new instance after re-instantiation
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn get_counter() -> u32 {
    COUNTER.load(Ordering::Relaxed)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
