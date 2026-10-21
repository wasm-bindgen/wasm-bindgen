//! A single passing test that logs, run in a dedicated worker.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_dedicated_worker);

#[wasm_bindgen_test]
fn passing_test() {
    console_log!("PASSING_TEST_LOG_MARKER");
}
