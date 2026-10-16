//! Tests for the `slice = view | array | copy` modes on imported functions,
//! primarily `slice = copy`.
//!
//! `slice = copy` lets a `&[T]` (or `Option<&[T]>`) outgoing argument with a
//! primitive element type arrive on the JS side as a *fresh typed array* JS
//! owns outright, rather than the default typed-array view of wasm linear
//! memory. The wire stays the borrowed slice (no Rust-side allocation); the
//! JS glue materialises the copy with `.slice()` and never frees.
//!
//! Coverage:
//! - per-fn `slice = copy` for `&[u8]`, `&[f64]`, `&[i64]` (the
//!   `BigInt64Array` tier included), asserting the JS-visible constructor and
//!   values
//! - freshness: JS retains the received array, Rust mutates the source after
//!   the call returns, the retained array is unchanged (the assertion a view
//!   cannot pass)
//! - `Option<&[u8]>` as `Some` and `None` (`None` → `undefined`)
//! - block-level `slice = copy`, with per-arg and per-fn overrides to each of
//!   the other two modes — including `slice = view`, the opt-out back to the
//!   default that only the valued syntax can spell — and the reverse
//!   nesting under a block-level `slice_to_array` alias
//! - `slice = array` and its released `slice_to_array` alias produce the same
//!   shape
//! - every other sequence shape: `Clamped<&[u8]>`, owned `Vec<T>` /
//!   `Box<[T]>` / `Option<Vec<T>>` / `Clamped<Vec<u8>>` (already fresh, so
//!   `copy` is a no-op and `array` converts), and `&[String]`, which has no
//!   typed form and crosses as a fresh `Array` under either mode
//! - per-mono generics: `&[T]` under `slice = copy`, instantiated at `u8`
//!   (typed copy) and at `JsString` (handle tier, already a fresh `Array`,
//!   where the mode is a no-op)
//! - fast-path guard: a bare `&[u8]` import still hands JS a view of wasm
//!   memory (no accidental default flip)

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/slice_modes.js")]
extern "C" {
    #[wasm_bindgen(js_name = kindOf)]
    fn kind_of_view(v: &[u8]) -> String;

    // Without any attribute the slice must still arrive as a view of wasm
    // memory: its backing buffer is the whole linear memory, strictly larger
    // than the view itself.
    #[wasm_bindgen(js_name = isWasmMemoryView)]
    fn is_wasm_memory_view(v: &[u8]) -> bool;

    #[wasm_bindgen(slice = copy, js_name = kindOf)]
    fn kind_of_copy_u8(v: &[u8]) -> String;
    #[wasm_bindgen(slice = copy, js_name = kindOf)]
    fn kind_of_copy_f64(v: &[f64]) -> String;
    #[wasm_bindgen(slice = copy, js_name = kindOf)]
    fn kind_of_copy_i64(v: &[i64]) -> String;
    #[wasm_bindgen(slice = copy, js_name = kindOf)]
    fn kind_of_copy_opt_u8(v: Option<&[u8]>) -> String;

    // The valued syntax and its released alias are one shape.
    #[wasm_bindgen(slice = array, js_name = kindOf)]
    fn kind_of_array(v: &[u16]) -> String;
    #[wasm_bindgen(slice_to_array, js_name = kindOf)]
    fn kind_of_array_alias(v: &[u16]) -> String;

    // A fresh copy owns its entire backing buffer.
    #[wasm_bindgen(slice = copy, js_name = isWasmMemoryView)]
    fn copy_is_wasm_memory_view(v: &[u8]) -> bool;

    #[wasm_bindgen(slice = copy)]
    fn js_copy_u8(v: &[u8]) -> Vec<u8>;
    #[wasm_bindgen(slice = copy)]
    fn js_copy_f64(v: &[f64]) -> Vec<f64>;
    #[wasm_bindgen(slice = copy)]
    fn js_copy_i64(v: &[i64]) -> Vec<i64>;
    #[wasm_bindgen(slice = copy)]
    fn js_copy_optional_u8(v: Option<&[u8]>) -> Option<Vec<u8>>;

    // Freshness pair: `retain` stores the received array on the JS side,
    // `retained` reads the stored values back after Rust has mutated (and
    // possibly reallocated) the source buffer.
    #[wasm_bindgen(slice = copy)]
    fn js_retain_u8(v: &[u8]);
    fn js_retained_u8() -> Vec<u8>;

    // Method form: the `this` argument is non-slice and must be left alone.
    pub type Sink;
    #[wasm_bindgen(constructor)]
    fn new() -> Sink;
    #[wasm_bindgen(method, slice = copy, js_name = kindOf)]
    fn kind_of_method(this: &Sink, v: &[u8]) -> String;
}

// Block-level `slice = copy` propagates to every fn in the block; a per-arg
// or per-fn mode overrides it for exactly its own scope, including back to
// the `view` default.
#[wasm_bindgen(module = "tests/wasm/slice_modes.js", slice = copy)]
extern "C" {
    #[wasm_bindgen(js_name = kindOf)]
    fn kind_of_block_copy(v: &[u16]) -> String;

    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_copy_block_mixed(
        copied: &[u8],
        #[wasm_bindgen(slice = array)] as_array: &[u8],
        #[wasm_bindgen(slice = view)] as_view: &[u8],
    ) -> String;

    #[wasm_bindgen(slice = array, js_name = kindOf)]
    fn kind_of_fn_array_override(v: &[u16]) -> String;

    #[wasm_bindgen(slice = view, js_name = isWasmMemoryView)]
    fn fn_view_override_is_wasm_memory_view(v: &[u8]) -> bool;

    // Per-mono generics on a `slice = copy` block: the mode resolves per
    // instantiation. At a primitive element it is a typed copy; at a handle
    // element (`JsString`) it is a no-op, and JS receives the fresh `Array`
    // the handle tier always produces.
    #[wasm_bindgen(experimental_generic_mono, js_name = kindOf)]
    fn kind_of_generic<T>(v: &[T]) -> String;
}

// The reverse nesting: block-level `slice_to_array` (the released alias) with
// per-arg overrides from the valued syntax.
#[wasm_bindgen(module = "tests/wasm/slice_modes.js", slice_to_array)]
extern "C" {
    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_array_block_mixed(
        as_array: &[u8],
        #[wasm_bindgen(slice = copy)] copied: &[u8],
        #[wasm_bindgen(slice = view)] as_view: &[u8],
    ) -> String;
}

// The mode applies by shape, to every sequence an argument can be.
#[wasm_bindgen(module = "tests/wasm/slice_modes.js")]
extern "C" {
    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_clamped(
        #[wasm_bindgen(slice = copy)] copied: Clamped<&[u8]>,
        #[wasm_bindgen(slice = array)] as_array: Clamped<&[u8]>,
        view: Clamped<&[u8]>,
    ) -> String;

    #[wasm_bindgen(js_name = kindsOf)]
    fn kinds_of_owned(
        #[wasm_bindgen(slice = copy)] copied: Vec<u8>,
        #[wasm_bindgen(slice = array)] as_array: Vec<u8>,
        #[wasm_bindgen(slice = array)] boxed: Box<[f64]>,
        #[wasm_bindgen(slice = array)] optional: Option<Vec<u8>>,
        #[wasm_bindgen(slice = array)] clamped: Clamped<Vec<u8>>,
    ) -> String;

    #[wasm_bindgen(slice = array)]
    fn joined(v: Vec<u16>) -> String;
    #[wasm_bindgen(slice = copy, js_name = joined)]
    fn joined_strings_copy(v: &[String]) -> String;
    #[wasm_bindgen(slice = array, js_name = joined)]
    fn joined_strings_array(v: Option<&[String]>) -> String;
}

#[wasm_bindgen_test]
fn every_sequence_shape() {
    assert_eq!(
        kinds_of_clamped(Clamped(&[1]), Clamped(&[2]), Clamped(&[3])),
        "Uint8ClampedArray:fresh,Array,Uint8ClampedArray:view"
    );
    assert_eq!(
        kinds_of_owned(
            vec![1],
            vec![2],
            Box::new([3.5]),
            Some(vec![4]),
            Clamped(vec![5])
        ),
        "Uint8Array:fresh,Array,Array,Array,Array"
    );
    assert_eq!(joined(vec![1, 2, 3]), "Array:1,2,3");
    let strings = ["a".to_string(), "b".to_string()];
    assert_eq!(joined_strings_copy(&strings), "Array:a,b");
    assert_eq!(joined_strings_array(Some(&strings)), "Array:a,b");
    assert_eq!(joined_strings_array(None), "undefined");
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
fn array_mode_and_alias_agree() {
    assert_eq!(kind_of_array(&[1, 2]), "Array");
    assert_eq!(kind_of_array_alias(&[1, 2]), "Array");
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
        kinds_of_copy_block_mixed(&[1], &[2], &[3]),
        "Uint8Array:fresh,Array,Uint8Array:view"
    );
    assert_eq!(kind_of_fn_array_override(&[1, 2]), "Array");
    assert!(fn_view_override_is_wasm_memory_view(&[1, 2]));
    assert_eq!(
        kinds_of_array_block_mixed(&[1], &[2], &[3]),
        "Array,Uint8Array:fresh,Uint8Array:view"
    );
}

#[wasm_bindgen_test]
fn per_mono_generic_resolves_per_instantiation() {
    assert_eq!(kind_of_generic(&[1u8, 2]), "Uint8Array");
    assert_eq!(kind_of_generic(&[1.5f64]), "Float64Array");
    let strings = [JsString::from("a"), JsString::from("b")];
    assert_eq!(kind_of_generic(&strings), "Array");
}
