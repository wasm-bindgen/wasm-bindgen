use js_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn new() {
    let inner = Error::new("some error");
    let error = AggregateError::new(&[inner.into()]);
    assert!(error.is_instance_of::<AggregateError>());
    assert!(error.is_instance_of::<Error>());
    assert!(error.is_instance_of::<Object>());
    let _: &Error = error.as_ref();
    let _: &Object = error.as_ref();
}

#[wasm_bindgen_test]
fn new_with_message() {
    let inner = Error::new("some error");
    let error = AggregateError::new_with_message(&[inner.into()], "Hello");
    let base_error: &Error = error.dyn_ref().unwrap();
    assert_eq!(JsValue::from(base_error.message()), "Hello");
    assert_eq!(JsValue::from(base_error.name()), "AggregateError");
}

#[wasm_bindgen_test]
fn new_with_options() {
    let inner = Error::new("some error");
    let cause = Error::new("the cause");
    let options = Object::new();
    Reflect::set(&options, &"cause".into(), &cause).unwrap();
    let error = AggregateError::new_with_options(&[inner.into()], "Hello", &options);
    let base_error: &Error = error.dyn_ref().unwrap();
    assert_eq!(JsValue::from(base_error.message()), "Hello");
    assert!(base_error.cause().is_instance_of::<Error>());
}

#[wasm_bindgen_test]
fn errors() {
    let a = Error::new("a");
    let b = Error::new("b");
    let error = AggregateError::new(&[a.into(), b.into()]);
    let errors = error.errors();
    assert_eq!(errors.length(), 2);
    let first: Error = errors.get(0).dyn_into().unwrap();
    assert_eq!(JsValue::from(first.message()), "a");
    let second: Error = errors.get(1).dyn_into().unwrap();
    assert_eq!(JsValue::from(second.message()), "b");
}

#[wasm_bindgen_test]
fn empty_errors() {
    let error = AggregateError::new(&[]);
    assert_eq!(error.errors().length(), 0);
}

#[wasm_bindgen_test]
fn name() {
    let error = AggregateError::new(&[]);
    let base_error: &Error = error.dyn_ref().unwrap();
    assert_eq!(JsValue::from(base_error.name()), "AggregateError");
}
