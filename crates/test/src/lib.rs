//! Runtime support for the `#[wasm_bindgen_test]` attribute
//!
//! More documentation can be found in the README for this crate!

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(wasm_bindgen_unstable_test_coverage, feature(coverage_attribute))]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![deny(missing_docs)]

extern crate alloc;

pub use wasm_bindgen_test_macro::{wasm_bindgen_bench, wasm_bindgen_test};

// Custom allocator that only returns pointers in the 2GB-4GB range
// To ensure we actually support more than 2GB of memory
#[cfg(all(test, feature = "gg-alloc"))]
#[global_allocator]
static A: gg_alloc::GgAlloc<std::alloc::System> = gg_alloc::GgAlloc::new(std::alloc::System);

/// Helper macro which acts like `println!` only routes to `console.error`
/// instead.
#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => (
        $crate::__rt::console_error(&format_args!($($arg)*))
    )
}

/// Helper macro which acts like `println!` only routes to `console.log`
/// instead.
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => (
        $crate::__rt::console_log(&format_args!($($arg)*))
    )
}

/// A macro used to configured how this test is executed by the
/// `wasm-bindgen-test-runner` harness.
///
/// This macro is invoked as:
///
/// ```ignore
/// wasm_bindgen_test_configure!(foo bar baz);
/// ```
///
/// where all of `foo`, `bar`, and `baz`, would be recognized options to this
/// macro. The currently known options to this macro are:
///
/// * `run_in_browser` - requires that this test is run in a browser rather than
///   node.js, which is the default for executing tests.
/// * `run_in_dedicated_worker` - requires that this test is run in a web worker rather than
///   node.js, which is the default for executing tests.
/// * `run_in_shared_worker` - requires that this test is run in a shared worker rather than
///   node.js, which is the default for executing tests.
/// * `run_in_service_worker` - requires that this test is run in a service worker rather than
///   node.js, which is the default for executing tests.
///
/// This macro may be invoked at most one time per test suite (an entire binary
/// like `tests/foo.rs`, not per module)
#[macro_export]
macro_rules! wasm_bindgen_test_configure {
    (run_in_browser $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_RUN_IN_BROWSER: [u8; 1] = [0x01];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    (run_in_worker $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_RUN_IN_DEDICATED_WORKER: [u8; 1] = [0x02];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    (run_in_dedicated_worker $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_RUN_IN_DEDICATED_WORKER: [u8; 1] = [0x02];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    (run_in_shared_worker $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_RUN_IN_SHARED_WORKER: [u8; 1] = [0x03];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    (run_in_service_worker $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_RUN_IN_SERVICE_WORKER: [u8; 1] = [0x04];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    (run_in_node_experimental $($others:tt)*) => (
        const _: () = {
            #[link_section = "__wasm_bindgen_test_unstable"]
            #[cfg(target_arch = "wasm32")]
            pub static __WBG_TEST_run_in_node_experimental: [u8; 1] = [0x05];
            $crate::wasm_bindgen_test_configure!($($others)*);
        };
    );
    () => ()
}

#[path = "rt/mod.rs"]
pub mod __rt;

// Make this only available to wasm32 so that we don't
// import minicov on other archs.
// That way you can use normal cargo test without minicov
#[cfg(target_arch = "wasm32")]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
mod coverage;

// <https://github.com/bheisler/criterion.rs>
//
// A modified `criterion.rs`, retaining only the basic benchmark capabilities.
pub use __rt::criterion::Criterion;

// web_time Instant
pub use __rt::web_time::Instant;
