//! A single failing test that logs, run in the default mode (Node.js).

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn failing_test() {
    console_log!("FAILING_TEST_LOG_MARKER");
    panic!()
}
