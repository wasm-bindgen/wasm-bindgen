//! Compile-time test: verify that unsafe_single_threaded_traits is incompatible with atomics
//!
//! # Purpose
//!
//! This test verifies that the `unsafe_single_threaded_traits` cfg cannot be
//! used when the `atomics` target feature is enabled. The incompatibility is enforced
//! by a `compile_error!` in `src/lib.rs`.
//!
//! # How It Works
//!
//! - When building WITHOUT atomics (normal case): This test compiles fine
//! - When building WITH atomics (CI tests): This test will still compile fine because
//!   the `unsafe_single_threaded_traits` cfg won't be enabled
//! - When building WITH BOTH atomics AND the cfg enabled: Compilation fails with:
//!   "The `unsafe_single_threaded_traits` cfg cannot be used with the `atomics` target feature..."
//!
//! CI runs tests with the following matrix:
//! - mvp target (no atomics)
//! - atomics target (`-Ctarget-feature=+atomics`)
//!
//! If someone attempts to enable `unsafe_single_threaded_traits` in a build that has
//! atomics, the compile_error! in src/lib.rs will trigger and prevent compilation.

use wasm_bindgen::prelude::*;

#[test]
fn test_basic_usage() {
    // This test verifies that basic wasm-bindgen usage works.
    // The real test is that this file compiles at all.
    let _ = JsValue::NULL;
}

#[cfg(unsafe_single_threaded_traits)]
#[test]
fn test_cfg_enabled_without_atomics() {
    // If this compiles, the cfg is enabled but atomics are not.
    // This is the intended use case.
    fn assert_send<T: Send>() {}
    assert_send::<JsValue>();
}

#[cfg(all(unsafe_single_threaded_traits, target_feature = "atomics"))]
compile_error!(
    "ERROR: This test should never compile!\n\
     If you see this error, the compile_error! in src/lib.rs failed to trigger.\n\
     The unsafe_single_threaded_traits cfg MUST NOT be used with atomics."
);
