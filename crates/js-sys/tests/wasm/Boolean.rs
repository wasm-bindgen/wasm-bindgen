use js_sys::*;
use wasm_bindgen::JsCast;
#[cfg(not(js_sys_unstable_apis))]
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[cfg(not(js_sys_unstable_apis))]
#[allow(deprecated)]
#[wasm_bindgen_test]
fn new_undefined() {
    assert!(!Boolean::new(&JsValue::undefined()).value_of());
}

#[cfg(not(js_sys_unstable_apis))]
#[allow(deprecated)]
#[wasm_bindgen_test]
fn new_truly() {
    assert!(Boolean::new(&JsValue::from("foo")).value_of());
}

#[cfg(not(js_sys_unstable_apis))]
#[allow(deprecated)]
#[wasm_bindgen_test]
fn boolean_inheritance() {
    let b = Boolean::new(&JsValue::from(true));
    assert!(b.is_instance_of::<Boolean>());
    assert!(b.is_instance_of::<Object>());
    let _: &Object = b.as_ref();
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn boolean_from() {
    // In unstable mode, use Boolean::from instead of Boolean::new
    assert!(!Boolean::from(false).value_of());
    assert!(Boolean::from(true).value_of());
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn boolean_inheritance_unstable() {
    let b = Boolean::from(true);
    assert!(!b.is_instance_of::<Boolean>());
    assert!(!b.is_instance_of::<Object>());
    assert_eq!(b.value_of(), true);
}
