// Reference test for the `slice = view | copy | array` and `vec = copy |
// array` import argument modes, primarily `slice = copy`.
//
// `slice = copy` keeps the user-facing argument type as `&[T]` and its
// default borrowed wire format, while the JS glue materialises a fresh
// typed array with `.slice()` and never frees — the wire is a borrow of
// the caller's slice, so there is no allocation to release.
//
// `js_view_u8` pins the unchanged default alongside: a bare `&[u8]`
// still hands JS the zero-copy typed-array view.

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(slice = copy)]
    fn js_copy_u8(v: &[u8]);
    #[wasm_bindgen(slice = copy)]
    fn js_copy_f64(v: &[f64]);
    #[wasm_bindgen(slice = copy)]
    fn js_copy_optional_u16(v: Option<&[u16]>);

    fn js_view_u8(v: &[u8]);
}

// Block-level form, with per-arg `slice = array` and `slice = view`
// overrides: each argument gets its own shape, and `view` opts back out
// to the default glue.
#[wasm_bindgen(slice = copy)]
extern "C" {
    fn js_block_mixed(
        copied: &[u16],
        #[wasm_bindgen(slice = array)] as_array: &[u16],
        #[wasm_bindgen(slice = view)] as_view: &[u16],
    );
}

// Each key acts only on its own shape: under `slice = array` owned data stays
// a fresh typed array, `Clamped` gets the same treatment as any typed slice,
// and `&[String]`, which has no typed form, is a fresh `Array` regardless.
#[wasm_bindgen(slice = array)]
extern "C" {
    fn js_shapes(owned: Vec<u8>, boxed: Box<[f64]>, clamped: Clamped<&[u8]>, strings: &[String]);
}

// Under `vec = array` borrowed data stays a view, and a per-arg `vec = copy`
// opts back out. A fn-level mode overrides the block's for its own key.
#[wasm_bindgen(vec = array)]
extern "C" {
    fn js_vec_block(borrowed: &[u8], owned: Vec<u8>, #[wasm_bindgen(vec = copy)] opted_out: Vec<u8>);
    #[wasm_bindgen(slice = copy, vec = copy)]
    fn js_fn_keys(borrowed: &[u8], owned: Vec<u8>);
}

// Both keys are inert on exports.
#[wasm_bindgen]
pub fn rust_inert(
    #[wasm_bindgen(slice = copy)] _borrowed: &[u8],
    #[wasm_bindgen(vec = array)] _owned: Vec<u8>,
) {
}

// On the per-mono path each instantiation crosses as the concrete slice
// would: with no mode a numeric instantiation keeps the zero-copy view glue,
// while `String` is decoded from its buffer of `(ptr, len)` pairs.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn js_generic_slice<T>(v: &[T]);
    #[wasm_bindgen(experimental_generic_mono, js_name = js_generic_slice)]
    fn js_generic_opt<T>(#[wasm_bindgen(slice = copy)] v: Option<&[T]>);
}

#[wasm_bindgen]
pub fn driver() {
    let v = vec![1u8, 2, 3];
    js_copy_u8(&v);
    js_view_u8(&v);

    js_copy_f64(&[1.5, 2.5]);

    let v16 = vec![1u16, 2];
    js_copy_optional_u16(Some(&v16));
    js_copy_optional_u16(None);

    js_block_mixed(&v16, &v16, &v16);

    js_shapes(v.clone(), Box::new([1.5]), Clamped(&v), &["a".to_string()]);
    js_vec_block(&v, v.clone(), v.clone());
    js_fn_keys(&v, v.clone());

    js_generic_slice(&[1u16, 2]);
    js_generic_slice(&["a".to_string()]);
    js_generic_opt(Some(&[1u8][..]));
}
