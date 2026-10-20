// Reference test for the `slice_to_array` attribute.
//
// `slice_to_array` keeps the user-facing argument type as `&[T]` and its
// ABI, so the JS-visible type lands as a plain `Array` rather than a typed
// array: for primitive elements the typed-array view of the caller's slice
// is wrapped in `Array.from(...)`, with no allocation and no free, while
// elements with no typed form are already a fresh `Array`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_u16_as_array(v: &[u16]);
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_string_as_array(v: &[String]);
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_optional_u16_as_array(v: Option<&[u16]>);
    // `Option` + an externref-shaped element kind. This is the one
    // combination where the JS loader takes its owned-buffer branch,
    // so it is the only case that pins that arm's codegen.
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_optional_string_as_array(v: Option<&[String]>);
}

// Block-level form: the attribute applies to every imported fn in the
// block. Mixed in here to lock the block-form codegen into the
// reference snapshot too.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn js_block_slice_u16(v: &[u16]);
}

#[wasm_bindgen]
pub fn driver() {
    let v = vec![1u16, 2, 3];
    js_slice_u16_as_array(&v);

    let s = vec!["a".to_string(), "b".to_string()];
    js_slice_string_as_array(&s);

    js_slice_optional_u16_as_array(Some(&v));
    js_slice_optional_u16_as_array(None);

    js_slice_optional_string_as_array(Some(&s));
    js_slice_optional_string_as_array(None);

    js_block_slice_u16(&v);
}
