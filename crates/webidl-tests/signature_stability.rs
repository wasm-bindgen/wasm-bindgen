// This tests that a stable interface method can have an unstable overload added
// via a partial interface in the unstable directory.

use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn stable_method_is_available() {
    let sig = SignatureStability::new().unwrap();
    let result = sig.process();
    assert_eq!(result, "stable");
}
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_overload_is_available_with_flag() {
    let sig = SignatureStability::new().unwrap();

    let mut options = SignatureStabilityOptions::new();
    options.mode(SignatureStabilityMode::Fast);

    let result = sig.process_with_options(&options);
    assert_eq!(result, "fast");
}

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_types_available_with_flag() {
    let _mode = SignatureStabilityMode::Safe;
    let _options = SignatureStabilityOptions::new();
}
