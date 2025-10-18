//! Tests for JsValue generic wrapper

use js_sys::{Array, JsString, Object};
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::{prelude::*, JsCast, JsVal, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "/tests/wasm/typed_js_value.js")]
extern "C" {
    #[wasm_bindgen(js_name = acceptsUntypedValue)]
    fn js_accepts_untyped_value(val: JsValue) -> bool;

    #[wasm_bindgen(js_name = acceptsUntypedRef)]
    fn js_accepts_untyped_ref(val: &JsValue) -> bool;

    #[wasm_bindgen(js_name = acceptsTypedValue)]
    fn js_accepts_typed_value(val: JsVal<JsString>) -> JsVal;

    #[wasm_bindgen(js_name = acceptsTypedRef)]
    fn js_accepts_typed_ref(val: &JsVal<JsString>) -> JsVal;
}

#[wasm_bindgen_test]
fn typed_js_value_from_js_value() {
    let val = JsValue::from(42);
    let typed: JsVal<JsValue> = JsVal::new(val.clone());
    assert_eq!(typed.as_ref(), &val);
}

#[wasm_bindgen_test]
fn typed_js_value_from_typed_value() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());
    assert!(typed.upcast().is_string());
}

#[wasm_bindgen_test]
fn typed_js_value_into_js_value() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val);
    let back: JsValue = typed.upcast();
    assert_eq!(back.as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_deref() {
    let str_val = JsString::from("test");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());

    let deref_val: &JsValue = typed.as_ref();
    assert!(deref_val.is_string());
    assert_eq!(deref_val.as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());

    let inner: JsString = typed.unwrap();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
#[should_panic(expected = "JsVal<js_sys::JsString>::unwrap called on value of wrong type")]
fn typed_js_value_unwrap_panic() {
    let num_val = JsVal::new(42);
    let typed: JsVal<JsString> = unsafe { core::mem::transmute(num_val) };
    let _inner: JsString = typed.unwrap();
}

#[wasm_bindgen_test]
fn typed_js_value_try_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());

    let result = typed.try_unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_try_unwrap_failure() {
    let num_val = JsValue::from(42);
    let typed: JsVal<JsString> = num_val.cast_unchecked();

    let result = typed.try_unwrap();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_array() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    arr.push(&JsValue::from(3));

    let typed: JsVal<Array> = JsVal::new(arr.clone());

    assert!(typed.as_ref().is_array());

    let inner = typed.unwrap();
    assert_eq!(inner.length(), 3);
}

#[wasm_bindgen_test]
fn typed_js_value_object() {
    let obj = Object::new();
    let typed: JsVal<Object> = JsVal::new(obj.clone());

    assert!(typed.as_ref().is_object());

    let inner: Object = typed.unwrap();
    assert_eq!(inner, obj);
}

#[wasm_bindgen_test]
fn typed_js_value_generic_function() {
    fn process_typed<T>(typed: JsVal<T>) -> JsVal<T>
    where
        T: TryFromJsValue + Into<JsValue> + JsCast,
    {
        assert!(!typed.as_ref().is_undefined());
        typed
    }

    let str_val = JsString::from("test");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());
    let result = process_typed(typed);

    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_clone() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());

    let cloned = typed.clone();
    assert_eq!(cloned.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_unchecked_into() {
    let js_val = JsValue::from_str("test");
    let typed: JsVal<JsString> = js_val.cast_unchecked();

    assert_eq!(typed.as_ref().as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_dyn_into() {
    let js_val = JsValue::from_str("test");
    let js_string: JsString = js_val.dyn_into().unwrap();
    let typed: JsVal<JsString> = JsVal::new(js_string);
    assert_eq!(typed.as_ref().as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_dyn_into_failure() {
    let js_val = JsValue::from(42);
    let result: Result<JsString, _> = js_val.dyn_into();

    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_with_primitives() {
    let val = JsVal::new(123u32);
    assert_eq!(val.unwrap(), 123);

    let val2 = JsVal::new(true);
    assert_eq!(val2.unwrap(), true);

    let val3 = JsVal::new("hello".to_string());
    assert_eq!(val3.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_debug() {
    let str_val = JsString::from("test");
    let typed: JsVal<JsString> = JsVal::new(str_val);

    let debug_str = format!("{:?}", typed);
    assert!(debug_str.contains("JsVal"));
    assert!(debug_str.contains("JsString"));
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::new(str_val.clone());
    let js_val: JsValue = str_val.into();

    assert_eq!(js_val, *typed.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_unchecked_creation() {
    let num_val = JsValue::from(42);
    let wrong_typed: JsVal<JsString> = num_val.cast_unchecked();

    assert!(!wrong_typed.as_ref().is_string());
    assert!(wrong_typed.as_ref().as_f64().is_some());
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_unchecked() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.clone().into();
    let typed: JsVal<JsString> = js_val.cast_unchecked();

    let inner: JsString = typed.unwrap();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_convert_between_types() {
    let str_val = JsString::from("hello");
    let typed_string: JsVal<JsString> = JsVal::new(str_val.clone());

    let typed_object: JsVal<Object> = typed_string.clone().cast_unchecked();
    assert!(typed_object.as_ref().is_string());

    let typed_jsvalue: JsVal<JsValue> = typed_string.clone().cast_unchecked();
    assert!(typed_jsvalue.as_ref().is_string());
    let val: JsValue = typed_jsvalue.unwrap();
    assert_eq!(val.as_string().unwrap(), "hello");

    let typed_string2: JsVal<JsString> = JsVal::new(str_val);
    let result: Result<Array, _> = typed_string2.upcast().dyn_into();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_convert_upcast() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));

    let typed_array: JsVal<Array> = JsVal::new(arr.clone());

    let typed_object: JsVal<Object> = typed_array.cast_unchecked();
    assert!(typed_object.as_ref().is_object());

    let obj: Object = typed_object.unwrap();
    assert_eq!(obj, arr.into());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_same_type() {
    let typed1: JsVal<String> = JsVal::new(String::from("hello"));
    let typed2: JsVal<JsString> = JsVal::new(JsString::from("hello"));
    let typed3: JsVal<JsString> = JsVal::new(JsString::from("world"));

    assert_eq!(&typed1, &typed2);
    assert_ne!(typed1, typed3);
    assert_ne!(typed2, typed3);

    let typed4: JsVal<JsString> = typed2.cast_unchecked();
    assert_eq!(typed1, typed4);

    assert_eq!(typed1.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_different_types() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.into();

    let typed_string: JsVal<JsString> = js_val.clone().cast_unchecked();
    let typed_object: JsVal<Object> = js_val.clone().cast_unchecked();
    let typed_jsvalue: JsVal<JsValue> = js_val.cast_unchecked();

    assert_eq!(typed_string, typed_string.clone());

    assert_eq!(typed_string.as_ref(), typed_object.as_ref());
    assert_eq!(typed_object.as_ref(), typed_jsvalue.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_with_primitives() {
    let num1: JsVal<u32> = JsVal::new(42u32);
    let num2: JsVal<u32> = JsVal::new(42u32);
    let num3: JsVal<u32> = JsVal::new(99u32);

    assert_eq!(num1, num2);
    assert_ne!(num1, num3);

    let bool1: JsVal<bool> = JsVal::new(true);
    let bool2: JsVal<bool> = JsVal::new(true);
    let bool3: JsVal<bool> = JsVal::new(false);

    assert_eq!(bool1, bool2);
    assert_ne!(bool1, bool3);
}

#[wasm_bindgen_test]
fn typed_js_value_doc_example() {
    let typed_string: JsVal<String> = JsVal::from_typed("test");

    // Checked cast to another type via upcast() + JsCast (returns Result)
    let Ok(str): Result<JsString, _> = typed_string.upcast().dyn_into() else {
        panic!("Invalid input");
    };

    assert_eq!(&str, "test");

    // Unchecked cast (zero-cost, but unsafe if types don't match)
    let typed_string: JsVal<String> = JsVal::from_typed("test2");
    let typed_jsstring: JsVal<JsString> = typed_string.cast_unchecked();
    // Works here, but would have paniced if the value had not been a string.
    let str: JsString = typed_jsstring.unwrap();

    assert_eq!(str, JsString::from("test2"));
}

#[wasm_bindgen_test]
fn typed_to_untyped_value_passing() {
    let typed_string: JsVal<String> = JsVal::new(String::from("test"));
    assert!(js_accepts_untyped_value(typed_string.upcast()));

    let typed_jsstring: JsVal<JsString> = JsVal::new(JsString::from("hello"));
    assert!(js_accepts_untyped_value(typed_jsstring.upcast()));

    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let typed_array: JsVal<Array> = JsVal::new(arr);
    assert!(!js_accepts_untyped_value(typed_array.upcast()));

    let typed_num: JsVal<u32> = JsVal::new(42u32);
    assert!(!js_accepts_untyped_value(typed_num.upcast()));
}

#[wasm_bindgen_test]
fn typed_to_untyped_ref_passing() {
    let obj = Object::new();
    let typed_object: JsVal<Object> = JsVal::new(obj);
    assert!(js_accepts_untyped_ref(typed_object.as_ref()));

    let arr = Array::new();
    let typed_array: JsVal<Array> = JsVal::new(arr);
    assert!(js_accepts_untyped_ref(typed_array.as_ref()));

    let typed_jsstring: JsVal<JsString> = JsVal::new(JsString::from("test"));
    assert!(!js_accepts_untyped_ref(typed_jsstring.as_ref()));

    let typed_bool: JsVal<bool> = JsVal::new(true);
    assert!(!js_accepts_untyped_ref(typed_bool.as_ref()));
}

#[wasm_bindgen_test]
fn untyped_to_typed_function_passing() {
    // Cast untyped JsValue to typed JsVal<JsString> to pass to typed function
    let untyped = JsValue::from_str("test");
    let typed_ref: &JsVal<JsString> = unsafe { core::mem::transmute(&untyped) };
    let result = js_accepts_typed_ref(typed_ref);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Cast untyped JsValue to typed JsVal<JsString> by value
    let typed_val: JsVal<JsString> = untyped.cast_unchecked();
    let result = js_accepts_typed_value(typed_val);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Typed values can be cast to the expected type
    let typed: JsVal<String> = JsVal::from_typed("hello");
    let result = js_accepts_typed_ref(&typed.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "hello");

    let typed2: JsVal<String> = JsVal::from_typed("world");
    let result = js_accepts_typed_value(typed2.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "world");
}
