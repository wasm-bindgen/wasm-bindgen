//! A value JS passes that its glue rejects leaves every other value of the
//! call with JS.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use core::sync::atomic::{AtomicU32, Ordering};

static LIVE: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub struct Kept(u32);

#[wasm_bindgen]
impl Kept {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> Kept {
        LIVE.fetch_add(1, Ordering::Relaxed);
        Kept(value)
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.0
    }

    #[wasm_bindgen(js_name = liveCount)]
    pub fn live_count() -> u32 {
        LIVE.load(Ordering::Relaxed)
    }

    #[wasm_bindgen(js_name = consumeWith)]
    pub fn consume_with(self, _names: Vec<String>) {}
}

impl Drop for Kept {
    fn drop(&mut self) {
        LIVE.fetch_sub(1, Ordering::Relaxed);
    }
}

#[wasm_bindgen]
pub fn rejected_points(_v: Vec<Kept>) {}

#[wasm_bindgen]
pub fn rejected_maybe_points(_v: Vec<Option<Kept>>) {}

#[wasm_bindgen]
pub fn rejected_point_then_names(_a: Kept, _b: Vec<String>) {}

#[wasm_bindgen]
pub fn rejected_point_then_point(_a: Kept, _b: Kept) {}

#[wasm_bindgen]
pub fn rejected_point_then_maybe_point(_a: Kept, _b: Option<Kept>) {}

#[wasm_bindgen]
pub fn rejected_point_then_nested(_a: Kept, _b: Vec<Vec<String>>) {}

#[wasm_bindgen]
pub fn rejected_nested_points(_v: Vec<Vec<Kept>>) {}

#[wasm_bindgen]
pub fn rejected_maybe_nested(_v: Vec<Option<Vec<String>>>) -> usize {
    _v.len()
}

#[wasm_bindgen]
pub fn rejected_nested_numbers(v: Vec<Vec<u32>>) -> usize {
    v.len()
}

#[wasm_bindgen]
pub fn rejected_names_then_point(_a: Vec<String>, _b: Kept) {}

#[wasm_bindgen]
pub fn rejected_handles() -> u32 {
    wasm_bindgen::externref_heap_live_count()
}

#[wasm_bindgen(module = "tests/wasm/rejected_values.js")]
extern "C" {
    #[wasm_bindgen(js_name = rejectedArguments)]
    fn rejected_arguments(memory: JsValue);
    #[wasm_bindgen(catch, js_name = jsBadPoints)]
    fn js_bad_points() -> Result<Vec<Kept>, JsValue>;
    #[wasm_bindgen(catch, js_name = jsBadNestedPoints)]
    fn js_bad_nested_points() -> Result<Vec<Vec<Kept>>, JsValue>;
    /// The sum of the points the last return left with JS, which it frees.
    #[wasm_bindgen(js_name = jsFreeKept)]
    fn js_free_kept() -> u32;
}

#[wasm_bindgen_test]
fn arguments() {
    rejected_arguments(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn returns() {
    let live = Kept::live_count();
    assert!(js_bad_points().is_err());
    assert_eq!(Kept::live_count(), live + 2);
    assert_eq!(js_free_kept(), 3);
    assert_eq!(Kept::live_count(), live);

    assert!(js_bad_nested_points().is_err());
    assert_eq!(Kept::live_count(), live + 2);
    assert_eq!(js_free_kept(), 3);
    assert_eq!(Kept::live_count(), live);
}
