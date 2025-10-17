//! Tests for JsValue generic wrapper

use js_sys::{Array, JsString, Object};
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::{JsCast, JsType, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn typed_js_value_from_js_value() {
    let val = JsValue::from(42);
    let typed: JsValue<JsValue> = JsValue::new(val.clone());
    assert_eq!(typed.as_ref(), &val);
}

#[wasm_bindgen_test]
fn typed_js_value_from_typed_value() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());
    assert!(typed.upcast().is_string());
}

#[wasm_bindgen_test]
fn typed_js_value_into_js_value() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val);
    let back: JsValue = typed.upcast();
    assert_eq!(back.as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_deref() {
    let str_val = JsString::from("test");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());

    let deref_val: &JsValue = typed.as_ref();
    assert!(deref_val.is_string());
    assert_eq!(deref_val.as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());

    let inner: JsString = typed.unwrap();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
#[should_panic(expected = "JsValue<js_sys::JsString>::unwrap called on value of wrong type")]
fn typed_js_value_unwrap_panic() {
    let num_val = JsValue::new(42);
    let typed: JsValue<JsString> = unsafe { core::mem::transmute(num_val) };
    let _inner: JsString = typed.unwrap();
}

#[wasm_bindgen_test]
fn typed_js_value_try_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());

    let result = typed.try_unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_try_unwrap_failure() {
    let num_val = JsValue::from(42);
    let typed: JsValue<JsString> = num_val.cast_unchecked();

    let result = typed.try_unwrap();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_array() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    arr.push(&JsValue::from(3));

    let typed: JsValue<Array> = JsValue::new(arr.clone());

    assert!(typed.as_ref().is_array());

    let inner = typed.unwrap();
    assert_eq!(inner.length(), 3);
}

#[wasm_bindgen_test]
fn typed_js_value_object() {
    let obj = Object::new();
    let typed: JsValue<Object> = JsValue::new(obj.clone());

    assert!(typed.as_ref().is_object());

    let inner: Object = typed.unwrap();
    assert_eq!(inner, obj);
}

#[wasm_bindgen_test]
fn typed_js_value_generic_function() {
    fn process_typed<T>(typed: JsValue<T>) -> JsValue<T>
    where
        T: TryFromJsValue + Into<JsValue> + JsCast,
    {
        assert!(!typed.as_ref().is_undefined());
        typed
    }

    let str_val = JsString::from("test");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());
    let result = process_typed(typed);

    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_clone() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());

    let cloned = typed.clone();
    assert_eq!(cloned.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_unchecked_into() {
    let js_val = JsValue::from_str("test");
    let typed: JsValue<JsString> = js_val.cast_unchecked();

    assert_eq!(typed.as_ref().as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_dyn_into() {
    let js_val = JsValue::from_str("test");
    let js_string: JsString = js_val.dyn_into().unwrap();
    let typed: JsValue<JsString> = JsValue::new(js_string);
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
    let val = JsValue::new(123u32);
    assert_eq!(val.unwrap(), 123);

    let val2 = JsValue::new(true);
    assert_eq!(val2.unwrap(), true);

    let val3 = JsValue::new("hello".to_string());
    assert_eq!(val3.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_debug() {
    let str_val = JsString::from("test");
    let typed: JsValue<JsString> = JsValue::new(str_val);

    let debug_str = format!("{:?}", typed);
    assert!(debug_str.contains("JsValue"));
    assert!(debug_str.contains("JsString"));
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq() {
    let str_val = JsString::from("hello");
    let typed: JsValue<JsString> = JsValue::new(str_val.clone());
    let js_val: JsValue = str_val.into();

    assert_eq!(js_val, *typed.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_unchecked_creation() {
    let num_val = JsValue::from(42);
    let wrong_typed: JsValue<JsString> = num_val.cast_unchecked();

    assert!(!wrong_typed.as_ref().is_string());
    assert!(wrong_typed.as_ref().as_f64().is_some());
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_unchecked() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.clone().into();
    let typed: JsValue<JsString> = js_val.cast_unchecked();

    let inner: JsString = typed.unwrap();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_convert_between_types() {
    let str_val = JsString::from("hello");
    let typed_string: JsValue<JsString> = JsValue::new(str_val.clone());

    let typed_object: JsValue<Object> = typed_string.clone().cast_unchecked();
    assert!(typed_object.as_ref().is_string());

    let typed_jsvalue: JsValue<JsValue> = typed_string.clone().cast_unchecked();
    assert!(typed_jsvalue.as_ref().is_string());
    let val: JsValue = typed_jsvalue.unwrap();
    assert_eq!(val.as_string().unwrap(), "hello");

    let typed_string2: JsValue<JsString> = JsValue::new(str_val);
    let result: Result<Array, _> = typed_string2.upcast().dyn_into();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_convert_upcast() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));

    let typed_array: JsValue<Array> = JsValue::new(arr.clone());

    let typed_object: JsValue<Object> = typed_array.cast_unchecked();
    assert!(typed_object.as_ref().is_object());

    let obj: Object = typed_object.unwrap();
    assert_eq!(obj, arr.into());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_same_type() {
    let typed1: JsValue<String> = JsValue::new(String::from("hello"));
    let typed2: JsValue<JsString> = JsValue::new(JsString::from("hello"));
    let typed3: JsValue<JsString> = JsValue::new(JsString::from("world"));

    assert_eq!(&typed1, &typed2);
    assert_ne!(typed1, typed3);
    assert_ne!(typed2, typed3);

    let typed4: JsValue<JsString> = typed2.cast_unchecked();
    assert_eq!(typed1, typed4);

    assert_eq!(typed1.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_different_types() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.into();

    let typed_string: JsValue<JsString> = js_val.clone().cast_unchecked();
    let typed_object: JsValue<Object> = js_val.clone().cast_unchecked();
    let typed_jsvalue: JsValue<JsValue> = js_val.cast_unchecked();

    assert_eq!(typed_string, typed_string.clone());

    assert_eq!(typed_string.as_ref(), typed_object.as_ref());
    assert_eq!(typed_object.as_ref(), typed_jsvalue.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_with_primitives() {
    let num1: JsValue<u32> = JsValue::new(42u32);
    let num2: JsValue<u32> = JsValue::new(42u32);
    let num3: JsValue<u32> = JsValue::new(99u32);

    assert_eq!(num1, num2);
    assert_ne!(num1, num3);

    let bool1: JsValue<bool> = JsValue::new(true);
    let bool2: JsValue<bool> = JsValue::new(true);
    let bool3: JsValue<bool> = JsValue::new(false);

    assert_eq!(bool1, bool2);
    assert_ne!(bool1, bool3);
}

#[wasm_bindgen_test]
fn typed_js_value_doc_example() {
    let typed_string: JsValue<String> = JsValue::from_typed("test");

    // Checked cast to another type via upcast() + JsCast (returns Result)
    let Ok(str): Result<JsString, _> = typed_string.upcast().dyn_into() else {
        panic!("Invalid input");
    };

    assert_eq!(&str, "test");

    // Unchecked cast (zero-cost, but unsafe if types don't match)
    let typed_string: JsValue<String> = JsValue::from_typed("test2");
    let typed_jsstring: JsValue<JsString> = typed_string.cast_unchecked();
    // Works here, but would have paniced if the value had not been a string.
    let str: JsString = typed_jsstring.unwrap();

    assert_eq!(str, JsString::from("test2"));
}

#[wasm_bindgen_test]
fn typed_to_untyped_value_passing() {
    fn accepts_untyped_value(val: JsValue) -> bool {
        val.is_string()
    }

    let typed_string: JsValue<String> = JsValue::new(String::from("test"));
    assert!(accepts_untyped_value(typed_string.upcast()));

    let typed_jsstring: JsValue<JsString> = JsValue::new(JsString::from("hello"));
    assert!(accepts_untyped_value(typed_jsstring.upcast()));

    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let typed_array: JsValue<Array> = JsValue::new(arr);
    assert!(!accepts_untyped_value(typed_array.upcast()));

    let typed_num: JsValue<u32> = JsValue::new(42u32);
    assert!(!accepts_untyped_value(typed_num.upcast()));
}

#[wasm_bindgen_test]
fn typed_to_untyped_ref_passing() {
    fn accepts_untyped_ref(val: &JsValue) -> bool {
        val.is_object()
    }

    let obj = Object::new();
    let typed_object: JsValue<Object> = JsValue::new(obj);
    assert!(accepts_untyped_ref(typed_object.as_ref()));

    let arr = Array::new();
    let typed_array: JsValue<Array> = JsValue::new(arr);
    assert!(accepts_untyped_ref(typed_array.as_ref()));

    let typed_jsstring: JsValue<JsString> = JsValue::new(JsString::from("test"));
    assert!(!accepts_untyped_ref(typed_jsstring.as_ref()));

    let typed_bool: JsValue<bool> = JsValue::new(true);
    assert!(!accepts_untyped_ref(typed_bool.as_ref()));
}

#[wasm_bindgen_test]
fn untyped_to_typed_function_passing() {
    fn accepts_typed_value<T: JsType>(val: JsValue<T>) -> JsValue<T> {
        val
    }

    fn accepts_typed_ref<T: JsType>(val: &JsValue<T>) -> JsValue<T> {
        val.clone()
    }

    // Can pass to &JsValue to a &JsValue<T>, and treat as a JsValue
    let untyped = JsValue::from_str("test");
    let result = accepts_typed_ref(&untyped);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Can pass a JsValue to a JsValue<T>, and treat as a JsValue
    let result = accepts_typed_value(untyped);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Backwards compat works above, while typed values behave as typed
    let typed: JsValue<String> = JsValue::from_typed("hello");
    let result = accepts_typed_ref(&typed);
    assert!(result.as_ref().is_string());
    assert_eq!(result.unwrap(), "hello");

    let result = accepts_typed_value(typed);
    assert!(result.as_ref().is_string());
    assert_eq!(result.unwrap(), "hello");
}
