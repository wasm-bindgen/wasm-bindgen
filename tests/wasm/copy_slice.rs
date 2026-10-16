//! Tests for the `copy_slice` attribute on imported functions.
//!
//! The attribute lets a `&[T]` (or `Option<&[T]>`) outgoing argument with a
//! primitive element type arrive on the JS side as a *fresh typed array* JS
//! owns outright, rather than the default typed-array view of wasm linear
//! memory. The wire stays the borrowed slice (no Rust-side allocation); the
//! JS glue materialises the copy with `.slice()` and never frees.
//!
//! Coverage:
//! - per-fn `copy_slice` for `&[u8]`, `&[f64]`, `&[i64]` (the `BigInt64Array`
//!   tier included), asserting the JS-visible constructor and values
//! - freshness: JS retains the received array, Rust mutates the source after
//!   the call returns, the retained array is unchanged (the assertion a view
//!   cannot pass)
//! - `Option<&[u8]>` as `Some` and `None` (`None` → `undefined`)
//! - block-level `copy_slice`, with per-arg and per-fn `slice_to_array`
//!   overrides (and the reverse nesting), so each argument gets its own shape
//! - per-mono generics: `&[T]` under `copy_slice`, instantiated at `u8`
//!   (typed copy) and at `JsString` (handle tier, already a fresh `Array`,
//!   where the attribute is a no-op)
//! - fast-path guard: a bare `&[u8]` import still hands JS a view of wasm
//!   memory (no accidental default flip)

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/copy_slice.js")]
extern "C" {
    #[wasm_bindgen(js_name = kindOf)]
    fn kind_of_view(v: &[u8]) -> String;

    // Without any attribute the slice must still arrive as a view of wasm
    // memory: its backing buffer is the whole linear memory, strictly larger
    // than the view itself.
    #[wasm_bindgen(js_name = isWasmMemoryView)]
    fn is_wasm_memory_view(v: &[u8]) -> bool;

    #[wasm_bindgen(copy_slice, js_name = kindOf)]
    fn kind_of_copy_u8(v: &[u8]) -> String;
    #[wasm_bindgen(copy_slice, js_name = kindOf)]
    fn kind_of_copy_f64(v: &[f64]) -> String;
    #[wasm_bindgen(copy_slice, js_name = kindOf)]
    fn kind_of_copy_i64(v: &[i64]) -> String;
    #[wasm_bindgen(copy_slice, js_name = kindOf)]
    fn kind_of_copy_opt_u8(v: Option<&[u8]>) -> String;

    // A fresh copy owns its entire backing buffer.
    #[wasm_bindgen(copy_slice, js_name = isWasmMemoryView)]
    fn copy_is_wasm_memory_view(v: &[u8]) -> bool;

    #[wasm_bindgen(copy_slice)]
    fn js_copy_u8(v: &[u8]) -> Vec<u8>;
    #[wasm_bindgen(copy_slice)]
    fn js_copy_f64(v: &[f64]) -> Vec<f64>;
    #[wasm_bindgen(copy_slice)]
    fn js_copy_i64(v: &[i64]) -> Vec<i64>;
    #[wasm_bindgen(copy_slice)]
    fn js_copy_optional_u8(v: Option<&[u8]>) -> Option<Vec<u8>>;

    // Freshness pair: `retain` stores the received array on the JS side,
    // `retained` reads the stored values back after Rust has mutated (and
    // possibly reallocated) the source buffer.
    #[wasm_bindgen(copy_slice)]
    fn js_retain_u8(v: &[u8]);
    fn js_retained_u8() -> Vec<u8>;

    // Method form: the `this` argument is non-slice and must be left alone.
    pub type Sink;
    #[wasm_bindgen(constructor)]
    fn new() -> Sink;
    #[wasm_bindgen(method, copy_slice, js_name = kindOf)]
    fn kind_of_method(this: &Sink, v: &[u8]) -> String;
}

// Block-level `copy_slice` propagates to every fn in the block; a per-arg or
// per-fn `slice_to_array` overrides it for exactly its own scope.
#[wasm_bindgen(module = "tests/wasm/copy_slice.js", copy_slice)]
extern "C" {
    #[wasm_bindgen(js_name = kindOf)]
    fn kind_of_block_copy(v: &[u16]) -> String;

    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_copy_block_mixed(
        copied: &[u8],
        #[wasm_bindgen(slice_to_array)] as_array: &[u8],
    ) -> String;

    #[wasm_bindgen(slice_to_array, js_name = kindOf)]
    fn kind_of_fn_array_override(v: &[u16]) -> String;

    // Per-mono generics on a `copy_slice` block: the mode resolves per
    // instantiation. At a primitive element it is a typed copy; at a handle
    // element (`JsString`) it is a no-op, and JS receives the fresh `Array`
    // the handle tier always produces.
    #[wasm_bindgen(experimental_generic_mono, js_name = kindOf)]
    fn kind_of_generic<T>(v: &[T]) -> String;
}

// The reverse nesting: block-level `slice_to_array` with a per-arg
// `copy_slice` override.
#[wasm_bindgen(module = "tests/wasm/copy_slice.js", slice_to_array)]
extern "C" {
    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_array_block_mixed(
        as_array: &[u8],
        #[wasm_bindgen(copy_slice)] copied: &[u8],
    ) -> String;
}

#[wasm_bindgen_test]
fn typed_copy_constructors_and_values() {
    assert_eq!(kind_of_copy_u8(&[1, 2, 3]), "Uint8Array");
    assert_eq!(kind_of_copy_f64(&[1.5, 2.5]), "Float64Array");
    assert_eq!(kind_of_copy_i64(&[-1, 1]), "BigInt64Array");

    assert_eq!(js_copy_u8(&[1, 2, 3]), vec![2u8, 4, 6]);
    assert_eq!(js_copy_f64(&[1.5, 2.5]), vec![3.0f64, 5.0]);
    assert_eq!(js_copy_i64(&[-1, 0, 1]), vec![-2i64, 0, 2]);
}

#[wasm_bindgen_test]
fn fresh_copy_owns_its_buffer() {
    let v = vec![9u8, 8, 7];
    assert!(is_wasm_memory_view(&v));
    assert!(!copy_is_wasm_memory_view(&v));
    // The default view path is unchanged by the attribute existing.
    assert_eq!(kind_of_view(&v), "Uint8Array");
}

#[wasm_bindgen_test]
fn retained_copy_survives_source_mutation() {
    let mut v = vec![1u8, 2, 3];
    js_retain_u8(&v);
    // Mutate and reallocate the source after the call returns. A retained
    // *view* would observe the mutation (or detach on memory growth); the
    // retained copy must not.
    v[0] = 99;
    v.extend_from_slice(&[0; 4096]);
    assert_eq!(js_retained_u8(), vec![1u8, 2, 3]);
}

#[wasm_bindgen_test]
fn optional() {
    assert_eq!(kind_of_copy_opt_u8(Some(&[1])), "Uint8Array");
    assert_eq!(kind_of_copy_opt_u8(None), "undefined");
    assert_eq!(js_copy_optional_u8(Some(&[5, 6])), Some(vec![10u8, 12]));
    assert_eq!(js_copy_optional_u8(None), None);
}

#[wasm_bindgen_test]
fn method_receiver_untouched() {
    let sink = Sink::new();
    assert_eq!(sink.kind_of_method(&[1, 2]), "Uint8Array:fresh");
}

#[wasm_bindgen_test]
fn block_level_and_overrides() {
    assert_eq!(kind_of_block_copy(&[1, 2]), "Uint16Array");
    assert_eq!(
        kinds_of_copy_block_mixed(&[1], &[2]),
        "Uint8Array:fresh,Array"
    );
    assert_eq!(kind_of_fn_array_override(&[1, 2]), "Array");
    assert_eq!(
        kinds_of_array_block_mixed(&[1], &[2]),
        "Array,Uint8Array:fresh"
    );
}

#[wasm_bindgen_test]
fn per_mono_generic_resolves_per_instantiation() {
    assert_eq!(kind_of_generic(&[1u8, 2]), "Uint8Array");
    assert_eq!(kind_of_generic(&[1.5f64]), "Float64Array");
    let strings = [JsString::from("a"), JsString::from("b")];
    assert_eq!(kind_of_generic(&strings), "Array");
}
