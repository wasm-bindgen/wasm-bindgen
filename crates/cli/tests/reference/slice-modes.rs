// Reference test for the `slice = view | array | copy` import argument
// modes, primarily `slice = copy`.
//
// `slice = copy` keeps the user-facing argument type as `&[T]` and its
// default borrowed wire format, while the JS glue materialises a fresh
// typed array with `.slice()` and never frees — the wire is a borrow of
// the caller's slice, so there is no allocation to release.
//
// `js_view_u8` pins the unchanged default alongside: a bare `&[u8]`
// still hands JS the zero-copy typed-array view.

use wasm_bindgen::prelude::*;

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
}
