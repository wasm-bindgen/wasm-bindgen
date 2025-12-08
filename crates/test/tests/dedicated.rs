use wasm_bindgen_test::console_log;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
    console_log!("hello");
}
