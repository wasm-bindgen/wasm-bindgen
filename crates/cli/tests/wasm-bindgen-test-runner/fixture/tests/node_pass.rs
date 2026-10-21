//! A single passing test that logs, run in the default mode (Node.js).

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn passing_test() {
    console_log!("PASSING_TEST_LOG_MARKER");
}
