//! Tests for JsValue generic wrapper

use js_sys::{Array, JsString, Object};
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::{prelude::*, JsCast, JsVal, JsValue};
use wasm_bindgen_test::*;

// Exported Rust function that receives a reference from JavaScript
// This will exercise RefFromWasmAbi
#[wasm_bindgen]
pub fn rust_receives_jsvalue_ref(val: &JsValue) -> bool {
    val.is_string()
}

// Typed version - receives a typed JsValue reference
#[wasm_bindgen]
pub fn rust_receives_typed_jsvalue_ref(val: &JsVal<JsString>) -> bool {
    val.as_ref().is_string()
}

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

    #[wasm_bindgen(js_name = testRefFromWasmAbi)]
    fn js_test_ref_from_wasm_abi(val: &JsValue) -> bool;

    #[wasm_bindgen(js_name = resetRefTracking)]
    fn js_reset_ref_tracking();

    #[wasm_bindgen(js_name = modifyObjectProperty)]
    fn js_modify_object_property(obj: &JsValue, key: &str, value: &JsValue) -> JsValue;
}

#[wasm_bindgen_test]
fn typed_js_value_from_js_value() {
    let val = JsValue::from(42);
    let typed: JsVal<JsValue> = JsVal::wrap(val.clone());
    assert_eq!(typed.as_ref(), &val);
}

#[wasm_bindgen_test]
fn typed_js_value_fromd_value() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());
    assert!(typed.upcast().is_string());
}

#[wasm_bindgen_test]
fn typed_js_value_into_js_value() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val);
    let back: JsValue = typed.upcast();
    assert_eq!(back.as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_deref() {
    let str_val = JsString::from("test");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());

    let deref_val: &JsValue = typed.as_ref();
    assert!(deref_val.is_string());
    assert_eq!(deref_val.as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());

    let inner: JsString = typed.unwrap();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_unchecked_behavior() {
    // unwrap() uses wbg_cast which is an unchecked cast - it doesn't validate types
    // This test demonstrates that unwrap() will succeed even with wrong types
    let num_val = JsVal::wrap(42);
    let typed: JsVal<JsString> = unsafe { core::mem::transmute(num_val) };

    // This succeeds even though the value is not actually a JsString
    let result: JsString = typed.unwrap();
    let as_jsvalue: JsValue = result.into();

    // The returned value is actually the number 42, not a string
    // unwrap() doesn't do runtime type checking - it's an unchecked cast
    assert!(!as_jsvalue.is_string());
    assert_eq!(as_jsvalue.as_f64(), Some(42.0));
}

#[wasm_bindgen_test]
fn typed_js_value_try_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());

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

    let typed: JsVal<Array> = JsVal::wrap(arr.clone());

    assert!(typed.as_ref().is_array());

    let inner = typed.unwrap();
    assert_eq!(inner.length(), 3);
}

#[wasm_bindgen_test]
fn typed_js_value_object() {
    let obj = Object::new();
    let typed: JsVal<Object> = JsVal::wrap(obj.clone());

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
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());
    let result = process_typed(typed);

    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_clone() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());

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
    let typed: JsVal<JsString> = JsVal::wrap(js_string);
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
    let val = JsVal::wrap(123u32);
    assert_eq!(val.unwrap(), 123);

    let val2 = JsVal::wrap(true);
    assert_eq!(val2.unwrap(), true);

    let val3 = JsVal::wrap("hello".to_string());
    assert_eq!(val3.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_debug() {
    let str_val = JsString::from("test");
    let typed: JsVal<JsString> = JsVal::wrap(str_val);

    let debug_str = format!("{:?}", typed);
    assert!(debug_str.contains("JsVal"));
    assert!(debug_str.contains("JsString"));
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq() {
    let str_val = JsString::from("hello");
    let typed: JsVal<JsString> = JsVal::wrap(str_val.clone());
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
    let typed_string: JsVal<JsString> = JsVal::wrap(str_val.clone());

    let typed_object: JsVal<Object> = typed_string.clone().cast_unchecked();
    assert!(typed_object.as_ref().is_string());

    let typed_jsvalue: JsVal<JsValue> = typed_string.clone().cast_unchecked();
    assert!(typed_jsvalue.as_ref().is_string());
    let val: JsValue = typed_jsvalue.unwrap();
    assert_eq!(val.as_string().unwrap(), "hello");

    let typed_string2: JsVal<JsString> = JsVal::wrap(str_val);
    let result: Result<Array, _> = typed_string2.upcast().dyn_into();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_convert_upcast() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));

    let typed_array: JsVal<Array> = JsVal::wrap(arr.clone());

    let typed_object: JsVal<Object> = typed_array.cast_unchecked();
    assert!(typed_object.as_ref().is_object());

    let obj: Object = typed_object.unwrap();
    assert_eq!(obj, arr.into());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_same_type() {
    let typed1: JsVal<String> = JsVal::wrap(String::from("hello"));
    let typed2: JsVal<JsString> = JsVal::wrap(JsString::from("hello"));
    let typed3: JsVal<JsString> = JsVal::wrap(JsString::from("world"));

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
    let num1 = JsVal::wrap(42u32);
    let num2: JsVal<u32> = JsVal::wrap(42u32);
    let num3: JsVal<u32> = JsVal::wrap(99u32);

    assert_eq!(num1, num2);
    assert_ne!(num1, num3);

    let bool1: JsVal<bool> = JsVal::wrap(true);
    let bool2: JsVal<bool> = JsVal::wrap(true);
    let bool3: JsVal<bool> = JsVal::wrap(false);

    assert_eq!(bool1, bool2);
    assert_ne!(bool1, bool3);
}

#[wasm_bindgen_test]
fn typed_js_value_doc_example() {
    let typed_string: JsVal<String> = JsVal::wrap(String::from("test"));

    // Checked cast to another type via upcast() + JsCast (returns Result)
    let Ok(str): Result<JsString, _> = typed_string.upcast().dyn_into() else {
        panic!("Invalid input");
    };

    assert_eq!(&str, "test");

    // Unchecked cast (zero-cost, but unsafe if types don't match)
    let typed_string: JsVal<String> = JsVal::wrap(String::from("test2"));
    let typed_jsstring: JsVal<JsString> = typed_string.cast_unchecked();
    // Works here, but would have paniced if the value had not been a string.
    let str: JsString = typed_jsstring.unwrap();

    assert_eq!(str, JsString::from("test2"));
}

#[wasm_bindgen_test]
fn typed_to_untyped_value_passing() {
    let typed_string: JsVal<String> = JsVal::wrap(String::from("test"));
    assert!(js_accepts_untyped_value(typed_string.upcast()));

    let typed_jsstring: JsVal<JsString> = JsVal::wrap(JsString::from("hello"));
    assert!(js_accepts_untyped_value(typed_jsstring.upcast()));

    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let typed_array: JsVal<Array> = JsVal::wrap(arr);
    assert!(!js_accepts_untyped_value(typed_array.upcast()));

    let typed_num: JsVal<u32> = JsVal::wrap(42u32);
    assert!(!js_accepts_untyped_value(typed_num.upcast()));
}

#[wasm_bindgen_test]
fn typed_to_untyped_ref_passing() {
    let obj = Object::new();
    let typed_object: JsVal<Object> = JsVal::wrap(obj);
    assert!(js_accepts_untyped_ref(typed_object.as_ref()));

    let arr = Array::new();
    let typed_array: JsVal<Array> = JsVal::wrap(arr);
    assert!(js_accepts_untyped_ref(typed_array.as_ref()));

    let typed_jsstring: JsVal<JsString> = JsVal::wrap(JsString::from("test"));
    assert!(!js_accepts_untyped_ref(typed_jsstring.as_ref()));

    let typed_bool: JsVal<bool> = JsVal::wrap(true);
    assert!(!js_accepts_untyped_ref(typed_bool.as_ref()));
}

#[wasm_bindgen_test]
fn untyped_to_typed_function_passing() {
    // Cast untyped JsValue to typed JsVal<JsString> to pass to typed function
    let untyped = JsValue::from_str("test");
    //  let typed_ref: &JsVal<JsString> = untyped.cast_ref_unchecked();
    //  let result = js_accepts_typed_ref(typed_ref);
    //  assert!(result.is_string());
    //  assert_eq!(result.as_string().unwrap(), "test");

    // Cast untyped JsValue to typed JsVal<JsString> by value
    let typed_val: JsVal<JsString> = untyped.cast_unchecked();
    let result = js_accepts_typed_value(typed_val);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Typed values can be cast to the expected type
    let typed: JsVal<String> = JsVal::wrap(String::from("hello"));
    let result = js_accepts_typed_ref(&typed.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "hello");

    let typed2: JsVal<String> = JsVal::wrap(String::from("world"));
    let result = js_accepts_typed_value(typed2.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "world");
}

#[wasm_bindgen_test]
fn typed_js_value_ref_from_wasm_abi() {
    use js_sys::Reflect;

    let obj = Object::new();
    let typed_obj: JsVal<Object> = JsVal::wrap(obj);

    assert!(!js_test_ref_from_wasm_abi(typed_obj.as_ref()));

    assert!(js_test_ref_from_wasm_abi(typed_obj.as_ref()));

    assert!(js_test_ref_from_wasm_abi(typed_obj.as_ref()));

    assert!(typed_obj.as_ref().is_object());

    let key = JsValue::from_str("testKey");
    let value = JsValue::from_str("testValue");
    let result = js_modify_object_property(typed_obj.as_ref(), "testKey", &value);
    assert_eq!(result.as_string().unwrap(), "testValue");

    let retrieved = Reflect::get(typed_obj.as_ref(), &key).unwrap();
    assert_eq!(retrieved.as_string().unwrap(), "testValue");

    js_reset_ref_tracking();
    let str_val = JsString::from("hello");
    let typed_str: JsVal<JsString> = JsVal::wrap(str_val);

    let untyped_ref: &JsValue = typed_str.as_ref();
    assert!(!js_test_ref_from_wasm_abi(untyped_ref));
    assert!(js_test_ref_from_wasm_abi(untyped_ref));

    assert!(typed_str.as_ref().is_string());
    assert_eq!(typed_str.as_ref().as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_primitives() {
    // Test with various primitive types

    // u32
    let val: JsVal<u32> = JsVal::wrap(42u32);
    assert_eq!(val.unwrap(), 42u32);

    // i32
    let val: JsVal<i32> = JsVal::wrap(-42i32);
    assert_eq!(val.unwrap(), -42i32);

    // f64
    let val: JsVal<f64> = JsVal::wrap(3.14f64);
    assert_eq!(val.unwrap(), 3.14f64);

    // bool
    let val: JsVal<bool> = JsVal::wrap(true);
    assert_eq!(val.unwrap(), true);

    let val: JsVal<bool> = JsVal::wrap(false);
    assert_eq!(val.unwrap(), false);

    // String
    let val: JsVal<String> = JsVal::wrap(String::from("hello"));
    assert_eq!(val.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_js_types() {
    // Test with JS types

    // JsString
    let js_str = JsString::from("test");
    let val: JsVal<JsString> = JsVal::wrap(js_str.clone());
    assert_eq!(val.unwrap(), js_str);

    // Array
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    let val: JsVal<Array> = JsVal::wrap(arr.clone());
    let unwrapped = val.unwrap();
    assert_eq!(unwrapped.length(), 2);

    // Object
    let obj = Object::new();
    let val: JsVal<Object> = JsVal::wrap(obj.clone());
    assert_eq!(val.unwrap(), obj);
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_vectors() {
    // Test with various vector types

    // Vec<u8>
    let vec_u8 = vec![1u8, 2u8, 3u8, 4u8];
    let val: JsVal<Vec<u8>> = JsVal::wrap(vec_u8.clone());
    assert_eq!(val.unwrap(), vec_u8);

    // Vec<i32>
    let vec_i32 = vec![1i32, -2i32, 3i32, -4i32];
    let val: JsVal<Vec<i32>> = JsVal::wrap(vec_i32.clone());
    assert_eq!(val.unwrap(), vec_i32);

    // Vec<f64>
    let vec_f64 = vec![1.1f64, 2.2f64, 3.3f64];
    let val: JsVal<Vec<f64>> = JsVal::wrap(vec_f64.clone());
    assert_eq!(val.unwrap(), vec_f64);

    // Vec<JsValue>
    let vec_js = vec![
        JsValue::from(1),
        JsValue::from_str("hello"),
        JsValue::from(true),
    ];
    let val: JsVal<Vec<JsValue>> = JsVal::wrap(vec_js.clone());
    let unwrapped = val.unwrap();
    assert_eq!(unwrapped.len(), 3);
    assert_eq!(unwrapped[0].as_f64().unwrap(), 1.0);
    assert_eq!(unwrapped[1].as_string().unwrap(), "hello");
    assert_eq!(unwrapped[2].as_bool().unwrap(), true);
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_box_types() {
    // Test with Box<[T]> types

    // Box<[u8]>
    let boxed: Box<[u8]> = vec![1u8, 2u8, 3u8].into_boxed_slice();
    let val: JsVal<Box<[u8]>> = JsVal::wrap(boxed.clone());
    assert_eq!(val.unwrap(), boxed);

    // Box<[i32]>
    let boxed: Box<[i32]> = vec![10i32, 20i32, 30i32].into_boxed_slice();
    let val: JsVal<Box<[i32]>> = JsVal::wrap(boxed.clone());
    assert_eq!(val.unwrap(), boxed);

    // Box<[f32]>
    let boxed: Box<[f32]> = vec![1.5f32, 2.5f32, 3.5f32].into_boxed_slice();
    let val: JsVal<Box<[f32]>> = JsVal::wrap(boxed.clone());
    assert_eq!(val.unwrap(), boxed);
}

#[wasm_bindgen_test]
#[should_panic]
fn static_unwrap_wrong_type_panic() {
    // Create a JsVal<u32> but try to unwrap as String
    let val = JsVal::wrap(42u32);
    let wrong: JsVal<String> = unsafe { core::mem::transmute(val) };
    let _: String = wrong.unwrap(); // Should panic
}

#[wasm_bindgen_test]
fn static_wrap_round_trip_through_js() {
    // Test that values can round-trip through JavaScript

    let original = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let wrapped: JsVal<Vec<u32>> = JsVal::wrap(original.clone());

    // Pass to JS and back
    let js_val: JsValue = wrapped.upcast();
    let re_wrapped: JsVal<Vec<u32>> = js_val.cast_unchecked();

    let unwrapped = re_wrapped.unwrap();
    assert_eq!(unwrapped, original);
}
