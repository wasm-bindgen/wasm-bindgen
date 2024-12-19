use alloc::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(wasm_bindgen_unstable_test_coverage)]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Option<Vec<u8>> {
    let mut coverage = Vec::new();
    // SAFETY: this function is not thread-safe, but our whole test runner is running single-threaded.
    unsafe {
        minicov::capture_coverage(&mut coverage).unwrap();
    }
    if coverage.is_empty() {
        console_error!(
            "Empty coverage data received. Make sure you compile the tests with
        RUSTFLAGS=\"-Cinstrument-coverage -Zno-profile-runtime --emit=llvm-ir\"",
        );
    }
    Some(coverage)
}

#[cfg(not(wasm_bindgen_unstable_test_coverage))]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Option<Vec<u8>> {
    None
}

#[cfg(wasm_bindgen_unstable_test_coverage)]
#[wasm_bindgen]
pub fn __wbgtest_module_signature() -> Option<u64> {
    Some(minicov::module_signature())
}

#[cfg(not(wasm_bindgen_unstable_test_coverage))]
#[wasm_bindgen]
pub fn __wbgtest_module_signature() -> Option<u64> {
    None
}
