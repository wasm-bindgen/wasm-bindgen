//! A single failing test that logs, run in a shared worker.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_shared_worker);

#[wasm_bindgen_test]
fn failing_test() {
    console_log!("FAILING_TEST_LOG_MARKER");
    panic!()
}
