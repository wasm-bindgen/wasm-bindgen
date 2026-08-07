//! Runtime coverage for a bare shared reference to a generic type parameter
//! (`&T`) in a per-monomorphisation generic import
//! (`#[wasm_bindgen(generic_per_mono)]`).
//!
//! Each instantiation exercises a distinct referent shape:
//! - `&u32` / `&f64`: a copyable value type, marshalled by value.
//! - `&RefThing`: a JS-handle type, marshalled by handle index.
//! - `&JsValue`: marshalled as an externref.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_ref.js")]
extern "C" {
    // `&T` where `T` monomorphises to a copyable value type: the value is
    // passed by copy, so the JS import just receives the number.
    #[wasm_bindgen(generic_per_mono, js_name = takeRefPrimitive)]
    fn take_ref_primitive<T>(x: &T) -> f64;

    type RefThing;

    #[wasm_bindgen(constructor)]
    fn new(val: f64) -> RefThing;

    // `&T` where `T` monomorphises to a JS-handle type: marshalled via the
    // handle's `IntoWasmAbi for &RefThing` impl.
    #[wasm_bindgen(generic_per_mono, js_name = readRefThing)]
    fn read_ref_thing<T>(x: &T) -> f64;

    // `&T` where `T` monomorphises to `JsValue`: marshalled as an externref.
    #[wasm_bindgen(generic_per_mono, js_name = echoRef)]
    fn echo_ref<T>(x: &T) -> JsValue;
}

#[wasm_bindgen_test]
fn generic_import_shared_ref_primitive() {
    assert_eq!(take_ref_primitive(&5u32), 6.0);
    assert_eq!(take_ref_primitive(&2.5f64), 3.5);
}

#[wasm_bindgen_test]
fn generic_import_shared_ref_handle() {
    let thing = RefThing::new(42.0);
    assert_eq!(read_ref_thing(&thing), 42.0);
}

#[wasm_bindgen_test]
fn generic_import_shared_ref_jsvalue() {
    let value = JsValue::from("hello");
    assert_eq!(echo_ref(&value), JsValue::from("hello"));
}
