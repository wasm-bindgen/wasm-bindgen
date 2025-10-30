//! Compile-time test: verify that unsafe-single-threaded-traits is incompatible with atomics
//!
//! # Purpose
//!
//! This test verifies that the `unsafe-single-threaded-traits` feature cannot be
//! used when the `atomics` target feature is enabled. The incompatibility is enforced
//! by a `compile_error!` in `src/lib.rs`.
//!
//! # How It Works
//!
//! - When building WITHOUT atomics (normal case): This test compiles fine
//! - When building WITH atomics (CI tests): This test will still compile fine because
//!   the `unsafe-single-threaded-traits` feature won't be enabled
//! - When building WITH BOTH atomics AND the feature enabled: Compilation fails with:
//!   "The `unsafe-single-threaded-traits` feature cannot be used with the `atomics` target feature..."
//!
//! CI runs tests with the following matrix:
//! - mvp target (no atomics)
//! - atomics target (`-Ctarget-feature=+atomics`)
//!
//! If someone attempts to enable `unsafe-single-threaded-traits` in a build that has
//! atomics, the compile_error! in src/lib.rs:52-57 will trigger and prevent compilation.

use wasm_bindgen::prelude::*;

#[test]
fn test_basic_usage() {
    // This test verifies that basic wasm-bindgen usage works.
    // The real test is that this file compiles at all.
    let _ = JsValue::NULL;
}

#[cfg(feature = "unsafe-single-threaded-traits")]
#[test]
fn test_feature_enabled_without_atomics() {
    // If this compiles, the feature is enabled but atomics are not.
    // This is the intended use case.
    fn assert_send<T: Send>() {}
    assert_send::<JsValue>();
}

#[cfg(all(feature = "unsafe-single-threaded-traits", target_feature = "atomics"))]
compile_error!(
    "ERROR: This test should never compile!\n\
     If you see this error, the compile_error! in src/lib.rs failed to trigger.\n\
     The unsafe-single-threaded-traits feature MUST NOT be used with atomics."
);
