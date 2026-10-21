//! A single passing test that logs, run in the browser main thread.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn passing_test() {
    console_log!("PASSING_TEST_LOG_MARKER");
}
