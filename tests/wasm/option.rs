use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/option.js")]
extern "C" {
    pub type MyType;
    #[wasm_bindgen(constructor)]
    fn new() -> MyType;

    fn take_none_byval(t: Option<MyType>);
    fn take_some_byval(t: Option<MyType>);
    fn return_undef_byval() -> Option<MyType>;
    fn return_null_byval() -> Option<MyType>;
    fn return_some_byval() -> Option<MyType>;
    fn test_option_values();

    #[wasm_bindgen(js_name = take_none_byval)]
    fn take_none_byref(t: Option<&MyType>);
    #[wasm_bindgen(js_name = take_some_byval)]
    fn take_some_byref(t: Option<&MyType>);

    fn take_option_jsvalue_none(val: Option<JsValue>);
    fn take_option_jsvalue_some(val: Option<JsValue>);
    fn return_option_jsvalue_none() -> Option<JsValue>;
    fn return_option_jsvalue_some() -> Option<JsValue>;
    fn test_option_jsvalue_values();
}

#[wasm_bindgen_test]
fn import_by_value() {
    take_none_byval(None);
    take_some_byval(Some(MyType::new()));
    assert!(return_null_byval().is_none());
    assert!(return_undef_byval().is_none());
    assert!(return_some_byval().is_some());
}

#[wasm_bindgen_test]
fn export_by_value() {
    test_option_values();
}

#[wasm_bindgen]
pub fn rust_take_none_byval(t: Option<MyType>) {
    assert!(t.is_none());
}

#[wasm_bindgen]
pub fn rust_take_some_byval(t: Option<MyType>) {
    assert!(t.is_some());
}

#[wasm_bindgen]
pub fn rust_return_none_byval() -> Option<MyType> {
    None
}

#[wasm_bindgen]
pub fn rust_return_some_byval() -> Option<MyType> {
    Some(MyType::new())
}

#[wasm_bindgen_test]
fn import_by_ref() {
    take_none_byref(None);
    take_some_byref(Some(&MyType::new()));
}

#[wasm_bindgen_test]
fn option_jsvalue() {
    take_option_jsvalue_none(None);
    take_option_jsvalue_some(Some(JsValue::from("test")));

    let none_val = return_option_jsvalue_none();
    assert!(none_val.is_none());

    let some_val = return_option_jsvalue_some();
    assert!(some_val.is_some());

    test_option_jsvalue_values();
}

#[wasm_bindgen]
pub fn rust_take_option_jsvalue_none(val: Option<JsValue>) {
    assert!(val.is_none());
}

#[wasm_bindgen]
pub fn rust_take_option_jsvalue_some(val: Option<JsValue>) {
    assert!(val.is_some());
}

#[wasm_bindgen]
pub fn rust_return_option_jsvalue_none() -> Option<JsValue> {
    None
}

#[wasm_bindgen]
pub fn rust_return_option_jsvalue_some() -> Option<JsValue> {
    Some(JsValue::from("rust value"))
}
