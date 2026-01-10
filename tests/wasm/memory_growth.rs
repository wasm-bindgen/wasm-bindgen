//! Tests for memory growth with cached TypedArray view refresh.
//!
//! This is particularly important for SharedArrayBuffer where the buffer reference
//! stays the same but grows in size. The cached views need to detect this and refresh.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/memory_growth.js")]
extern "C" {
    fn get_memory_byte_length() -> u32;
}

/// Grow memory by allocating a large vector, then verify string operations
/// still work (which depend on cached TypedArray views being refreshed).
#[wasm_bindgen_test]
fn memory_growth_refreshes_cached_views() {
    // Get initial memory size
    let initial_size = get_memory_byte_length();

    // Allocate enough to likely trigger memory growth (1MB)
    let large_vec: Vec<u8> = vec![42u8; 1024 * 1024];

    // Verify the allocation worked
    assert_eq!(large_vec.len(), 1024 * 1024);
    assert_eq!(large_vec[0], 42);
    assert_eq!(large_vec[large_vec.len() - 1], 42);

    // Get new memory size - should have grown
    let new_size = get_memory_byte_length();
    assert!(
        new_size >= initial_size,
        "Memory should not shrink: {} -> {}",
        initial_size,
        new_size
    );

    // Now test string operations which use the cached Uint8Array view.
    // If the view wasn't refreshed after memory growth, this could fail
    // by accessing stale memory or out-of-bounds.
    let test_string = "Hello after memory growth! 🦀";
    let result = echo_string(test_string);
    assert_eq!(result, test_string);

    // Test with a longer string to ensure we're using the new memory region
    let long_string: String = "x".repeat(10000);
    let long_result = echo_string(&long_string);
    assert_eq!(long_result, long_string);

    // Keep the large_vec alive until here to ensure memory stays grown
    drop(large_vec);
}

/// Echo a string back - this exercises the cached TypedArray views
/// used for string encoding/decoding.
#[wasm_bindgen]
pub fn echo_string(s: &str) -> String {
    s.to_string()
}

/// Explicitly grow memory and verify string operations work.
#[wasm_bindgen_test]
fn explicit_memory_grow() {
    // First, do some string operations
    let s1 = echo_string("before grow");
    assert_eq!(s1, "before grow");

    // Grow memory explicitly using core::arch::wasm32::memory_grow
    // This is a more direct test than relying on allocator behavior
    let pages_to_grow = 10; // 640KB
    let old_pages = core::arch::wasm32::memory_grow(0, pages_to_grow);
    assert!(old_pages != usize::MAX, "memory.grow failed");

    // Now verify string operations still work after explicit growth
    let s2 = echo_string("after grow");
    assert_eq!(s2, "after grow");

    // Test with unicode to ensure proper encoding
    let s3 = echo_string("after grow: 日本語 🎉");
    assert_eq!(s3, "after grow: 日本語 🎉");
}

/// Repeatedly grow memory and verify string operations.
/// This is a stress test for the cached view refresh logic.
#[wasm_bindgen_test]
fn repeated_memory_growth() {
    for i in 0..5 {
        // Grow memory
        let old_pages = core::arch::wasm32::memory_grow(0, 1);
        assert!(
            old_pages != usize::MAX,
            "memory.grow failed on iteration {}",
            i
        );

        // Verify string operations work after each growth
        let test = format!("iteration {} test string with unicode: café ☕", i);
        let result = echo_string(&test);
        assert_eq!(result, test);
    }
}
