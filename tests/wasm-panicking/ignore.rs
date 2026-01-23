use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
#[ignore]
fn should_panic() {
    panic!()
}

#[wasm_bindgen_test]
#[ignore = "reason"]
fn should_panic_string() {
    panic!()
}
