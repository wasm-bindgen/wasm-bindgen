use crate::generated::*;
use js_sys::Function;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn multi_op_same_name() {
    let a = CallbackInterface2::new();
    let b = TakeCallbackInterface::new().unwrap();
    b.b(&a);
}

#[wasm_bindgen_test]
fn single_op_function() {
    let a = Function::new_no_args("");
    let b = TakeCallbackInterface::new().unwrap();
    b.a_with_callback(&a);
}

#[wasm_bindgen_test]
fn single_op_dict() {
    let a = CallbackInterface1::new();
    let b = TakeCallbackInterface::new().unwrap();
    b.a_with_callback_interface1(&a);
}

#[wasm_bindgen_test]
fn dict_methods() {
    let a = CallbackInterface1::new();
    a.set_foo(&Function::new_no_args(""));
}

#[wasm_bindgen_test]
fn dict_methods1() {
    let a = CallbackInterface2::new();
    a.set_foo(&Function::new_no_args(""));
    a.set_bar(&Function::new_no_args(""));
}
