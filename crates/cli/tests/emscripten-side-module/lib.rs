use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

static DROPS: AtomicU32 = AtomicU32::new(0);
struct DropMarker;
impl Drop for DropMarker {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::Relaxed);
    }
}

#[wasm_bindgen]
pub struct Counter {
    callback: AssertUnwindSafe<Closure<dyn FnMut(i32) -> i32>>,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(start: i32) -> Self {
        let marker = DropMarker;
        let mut value = start;
        Self {
            callback: AssertUnwindSafe(Closure::new(move |delta: i32| {
                let _ = &marker;
                value += delta;
                value
            })),
        }
    }
    pub fn callback(&self) -> JsValue {
        self.callback.0.as_ref().clone()
    }
}

#[wasm_bindgen]
pub fn drop_count() -> u32 {
    DROPS.load(Ordering::Relaxed)
}

#[wasm_bindgen]
pub fn call_from_rust(callback: &js_sys::Function, value: i32) -> Result<JsValue, JsValue> {
    callback.call1(&JsValue::UNDEFINED, &JsValue::from(value))
}

#[wasm_bindgen]
pub fn panic_for_test() {
    panic!("side module panic");
}
