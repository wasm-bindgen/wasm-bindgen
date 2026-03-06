use crate::generated::*;
use wasm_bindgen_test::*;

/// Test that [WbgGeneric] dictionaries use typed generics (e.g. &[Number])
/// instead of &JsValue, even for stable APIs.
#[wasm_bindgen_test]
fn wbg_generic_dict_uses_typed_generics() {
    let dict = GenericDict::new("hello");
    // sequence<long> field should accept &[Number] (typed slice).
    // This would fail to compile if generics weren't enabled,
    // because in legacy mode slice types aren't supported.
    let nums: Vec<js_sys::Number> = vec![1.into(), 2.into(), 3.into()];
    dict.set_items(&nums);
}

/// Test that non-generic dictionaries do NOT use typed generics in stable mode.
/// The sequence<long> field should take &JsValue (legacy).
#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn non_generic_dict_uses_legacy_types() {
    let dict = NonGenericDict::new("hello");
    // In stable mode without [WbgGeneric], sequence<long> fields
    // are collapsed to &JsValue.
    let arr = js_sys::Array::new();
    arr.push(&1.into());
    dict.set_items(&arr);
}
