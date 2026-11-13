//! A value JS passes that its glue rejects leaves every other value of the
//! call with JS, and a sequence Rust gives is released however far JS got in
//! reading it.

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

    #[wasm_bindgen(js_name = whileBorrowed)]
    pub fn while_borrowed(&self, f: &js_sys::Function) -> Result<JsValue, JsValue> {
        f.call0(&JsValue::NULL)
    }

    #[wasm_bindgen(js_name = consumeWith)]
    pub fn consume_with(self, _names: Vec<String>) {}

    pub fn merge(self, _other: Kept) -> String {
        String::new()
    }

    #[wasm_bindgen(js_name = mergeAll)]
    pub fn merge_all(self, _others: Vec<Kept>, _names: Vec<String>) -> String {
        String::new()
    }

    #[wasm_bindgen(js_name = mergeMaybe)]
    pub fn merge_maybe(self, _other: Option<Kept>) -> String {
        String::new()
    }

    pub fn label(&self, _other: &Kept) -> String {
        String::new()
    }

    pub fn join(_a: Kept, _names: Vec<String>) -> String {
        String::new()
    }
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
pub fn rejected_nested_names_then_point(_a: Vec<Vec<String>>, _b: Kept) {}

#[wasm_bindgen]
pub fn rejected_points_then_point(_a: Vec<Kept>, _b: Kept) {}

#[wasm_bindgen]
pub fn rejected_points_then_chars(_a: Vec<Kept>, _b: &Kept, _c: Vec<char>) {}

#[wasm_bindgen]
pub fn rejected_points_then_maybe_chars(_a: Vec<Kept>, _b: Vec<Option<char>>) {}

#[wasm_bindgen]
pub fn rejected_names(v: Vec<String>) -> usize {
    v.len()
}

#[wasm_bindgen]
pub fn rejected_boxed_names(v: Box<[String]>) -> usize {
    v.len()
}

#[wasm_bindgen]
pub fn rejected_maybe_names(v: Option<Vec<String>>) -> usize {
    v.map_or(0, |v| v.len())
}

#[wasm_bindgen]
pub fn rejected_returned_names() -> bool {
    js_not_names().is_err()
}

#[wasm_bindgen]
pub fn rejected_named(_a: Kept, _names: Vec<String>) -> String {
    String::new()
}

/// Where the shadow stack is, by a frame of the same size each call.
#[wasm_bindgen]
pub fn rejected_stack() -> u32 {
    let frame = [0u8; 64];
    core::hint::black_box(frame.as_ptr()) as u32
}

#[wasm_bindgen]
pub fn rejected_handles() -> u32 {
    wasm_bindgen::externref_heap_live_count()
}

#[wasm_bindgen]
pub fn given_names(n: usize) -> Vec<String> {
    (0..n).map(|_| "a".repeat(200 * 1024)).collect()
}

#[wasm_bindgen]
pub fn given_nested_names(n: usize) -> Vec<Vec<String>> {
    (0..n).map(|_| given_names(2)).collect()
}

#[wasm_bindgen]
pub fn given_points(n: u32) -> Vec<Kept> {
    (0..n).map(Kept::new).collect()
}

#[wasm_bindgen]
pub fn given_maybe_points(n: u32) -> Vec<Option<Kept>> {
    (0..n).map(|i| Some(Kept::new(i))).collect()
}

#[wasm_bindgen]
pub fn given_values(n: u32) -> Vec<JsValue> {
    (0..n).map(|i| JsValue::from(format!("v{i}"))).collect()
}

#[wasm_bindgen]
pub fn given_maybe_values(n: u32) -> Vec<Option<JsValue>> {
    (0..n)
        .map(|i| Some(JsValue::from(format!("v{i}"))))
        .collect()
}

/// Whether JS failed to take a call's values.
#[wasm_bindgen]
pub fn given_mixed() -> bool {
    js_takes_mixed(
        given_points(2),
        &|| {},
        given_names(2),
        given_values(2),
        "a".repeat(200 * 1024),
        vec![0; 200 * 1024],
    )
    .is_err()
}

#[wasm_bindgen(module = "tests/wasm/rejected_values.js")]
extern "C" {
    #[wasm_bindgen(js_name = rejectedArguments)]
    fn rejected_arguments(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedInstances)]
    fn rejected_instances();
    #[wasm_bindgen(js_name = rejectedByRust)]
    fn rejected_by_rust();
    #[wasm_bindgen(js_name = rejectedKeepsTheStack)]
    fn rejected_keeps_the_stack();
    #[wasm_bindgen(js_name = rejectedChangedValues)]
    fn rejected_changed_values(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedNotArrays)]
    fn rejected_not_arrays();
    #[wasm_bindgen(catch, js_name = jsNotNames)]
    fn js_not_names() -> Result<Vec<String>, JsValue>;
    #[wasm_bindgen(js_name = failedDecodes)]
    fn failed_decodes(memory: JsValue);
    #[wasm_bindgen(catch, js_name = jsTakesMixed)]
    fn js_takes_mixed(
        a: Vec<Kept>,
        f: &dyn Fn(),
        b: Vec<String>,
        c: Vec<JsValue>,
        d: String,
        e: Vec<u8>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, js_name = jsBadNames)]
    fn js_bad_names() -> Result<Vec<String>, JsValue>;
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
fn arguments_freed_or_repeated() {
    rejected_instances();
}

#[wasm_bindgen_test]
fn arguments_rust_rejects() {
    rejected_by_rust();
}

#[wasm_bindgen_test]
fn arguments_keep_the_stack() {
    rejected_keeps_the_stack();
}

#[wasm_bindgen_test]
fn arguments_read_again() {
    rejected_changed_values(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn arguments_not_arrays() {
    rejected_not_arrays();
}

#[wasm_bindgen_test]
fn returns() {
    // A value an import with `catch` returns is rejected as the import's
    // error.
    assert!(js_bad_names().is_err());

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

#[wasm_bindgen_test]
fn decodes() {
    failed_decodes(wasm_bindgen::memory());
}
