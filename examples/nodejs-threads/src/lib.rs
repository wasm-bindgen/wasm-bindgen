use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

// A static atomic counter that can be safely accessed from multiple threads
static COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn increment() -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn get_counter() -> u32 {
    COUNTER.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// Test that we can grow memory and still access it correctly
#[wasm_bindgen]
pub fn allocate_and_sum(size: usize) -> u32 {
    let v: Vec<u32> = (0..size as u32).collect();
    v.iter().sum()
}
