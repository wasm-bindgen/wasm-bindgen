#![cfg(not(target_family = "wasm"))]

use std::sync::{Condvar, Mutex};
use std::time::Duration;

use wasm_bindgen_test::wasm_bindgen_test;

static TEST: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());

// Sorts before `test_wait` so libtest runs it first under `--test-threads=1`.
#[wasm_bindgen_test(unsupported = test)]
fn test_success() {
    let mut success = TEST.0.lock().unwrap();
    *success = true;
    TEST.1.notify_one();
}

#[test]
fn test_wait() {
    let success = TEST.0.lock().unwrap();
    let (success, result) = TEST
        .1
        .wait_timeout_while(success, Duration::from_secs(30), |success| !*success)
        .unwrap();
    assert!(
        *success && !result.timed_out(),
        "`unsupported = test` did not run `test_success`"
    );
}
