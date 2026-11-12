use wasm_bindgen::prelude::*;
use wasm_bindgen::SafeInteger;

#[wasm_bindgen]
extern "C" {
    type Metrics;
    #[wasm_bindgen(method, getter, js_name = backlogCount)]
    fn backlog_count(this: &Metrics) -> SafeInteger<u64>;
    #[wasm_bindgen(method, getter)]
    fn delta(this: &Metrics) -> Option<SafeInteger<i64>>;
    #[wasm_bindgen(catch)]
    fn fallible(n: SafeInteger<i64>) -> Result<SafeInteger<u64>, JsValue>;
    fn counts(a: Vec<SafeInteger<u64>>) -> Vec<SafeInteger<i64>>;
}

#[wasm_bindgen]
pub fn backlog(metrics: &Metrics) -> SafeInteger<u64> {
    metrics.backlog_count()
}

#[wasm_bindgen]
pub fn optional(a: Option<SafeInteger<u64>>) -> Option<SafeInteger<i64>> {
    a.map(|n| (*n as i64).into())
}

#[wasm_bindgen]
pub fn checked(n: SafeInteger<i64>) -> Result<SafeInteger<u64>, JsError> {
    let _ = fallible(n);
    Err(JsError::new("nope"))
}

#[wasm_bindgen]
pub fn many(a: &[SafeInteger<u64>], b: Vec<SafeInteger<i64>>) -> Vec<SafeInteger<u64>> {
    let _ = counts(a.to_vec());
    let _ = b;
    a.to_vec()
}

#[wasm_bindgen]
pub fn mixed(a: SafeInteger<u64>, b: u64) -> u64 {
    *a + b
}
