//! A single failing test that logs, run in the browser main thread.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn failing_test() {
    console_log!("FAILING_TEST_LOG_MARKER");
    panic!()
}
