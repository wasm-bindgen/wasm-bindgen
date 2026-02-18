use crate::generated::*;
use js_sys::*;
use wasm_bindgen_test::*;

// Regression test: callback interface types must exist when next-unstable is disabled
#[cfg(not(wbg_next_unstable))]
#[allow(dead_code)]
fn _assert_callback_interface_type_exists(_: &CallbackInterface1) {}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn callback_interface_as_typed_function() {
    // In next-unstable mode, callback interfaces are replaced with typed functions
    // (e.g., &VoidFunction instead of &CallbackInterface1 dict type)
    // Method names are simple (no _with_X suffix) since there's only one variant
    let f = Function::new_no_args("");
    let iface = TakeCallbackInterface::new().unwrap();
    iface.a_with_callback(&f);
}

#[cfg(wbg_next_unstable)]
#[wasm_bindgen_test]
fn callback_interface_as_typed_function() {
    // In next-unstable mode, callback interfaces are replaced with typed functions
    // Function<fn() -> Undefined> instead of VoidFunction
    // Method names are simple (no _with_X suffix) since there's only one variant
    let f: Function<fn() -> js_sys::Undefined> = Function::new_no_args_typed("");
    let iface = TakeCallbackInterface::new().unwrap();
    iface.a(&f);
}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn single_op_function() {
    let a = Function::new_no_args("");
    let b = TakeCallbackInterface::new().unwrap();
    b.a_with_callback(&a);
}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn single_op_dict() {
    let a = CallbackInterface1::new();
    let b = TakeCallbackInterface::new().unwrap();
    b.a_with_callback_interface1(&a);
}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn dict_methods() {
    let a = CallbackInterface1::new();
    a.set_foo(&Function::new_no_args(""));
}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn multi_op_same_name() {
    let a = CallbackInterface2::new();
    let b = TakeCallbackInterface::new().unwrap();
    b.b(&a);
}

#[cfg(not(wbg_next_unstable))]
#[wasm_bindgen_test]
fn dict_methods1() {
    let a = CallbackInterface2::new();
    a.set_foo(&Function::new_no_args(""));
    // two operations on a callback interface is not valid!
    // a.set_bar(&Function::new_no_args(""));
}
