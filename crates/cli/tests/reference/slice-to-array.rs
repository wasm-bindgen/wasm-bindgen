// Reference test for the `slice_to_array` attribute.
//
// `slice_to_array` keeps the user-facing argument type as `&[T]` while
// rewriting the ABI path so the slice is cloned into a freshly
// allocated buffer that JS owns and frees, with the JS-visible type
// landing as a plain `Array` rather than a typed array. For primitive
// element kinds the typed-array view is wrapped in `Array.from(...)`
// before being handed to the JS shim.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_u16_as_array(v: &[u16]);
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_string_as_array(v: &[String]);
    #[wasm_bindgen(slice_to_array)]
    fn js_slice_optional_u16_as_array(v: Option<&[u16]>);
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

    js_block_slice_u16(&v);
}
