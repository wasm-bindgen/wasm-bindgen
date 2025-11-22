//! Tests for JsValue generic wrapper

use js_sys::{Array, JsString, Object};
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::{prelude::*, JsCast, JsRef, JsValue};
use wasm_bindgen_test::*;

// Exported Rust function that receives a reference from JavaScript
// This will exercise RefFromWasmAbi
#[wasm_bindgen]
pub fn rust_receives_jsvalue_ref(val: &JsValue) -> bool {
    val.is_string()
}

// Typed version - receives a typed JsValue reference
#[wasm_bindgen]
pub fn rust_receives_typed_jsvalue_ref(val: &JsRef<JsString>) -> bool {
    val.as_ref().is_string()
}

#[wasm_bindgen(module = "/tests/wasm/typed_js_value.js")]
extern "C" {
    #[wasm_bindgen(js_name = acceptsUntypedValue)]
    fn js_accepts_untyped_value(val: JsValue) -> bool;

    #[wasm_bindgen(js_name = acceptsUntypedRef)]
    fn js_accepts_untyped_ref(val: &JsValue) -> bool;

    #[wasm_bindgen(js_name = acceptsTypedValue)]
    fn js_accepts_typed_value(val: JsRef<JsString>) -> JsRef;

    #[wasm_bindgen(js_name = acceptsTypedRef)]
    fn js_accepts_typed_ref(val: &JsRef<JsString>) -> JsRef;

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
    let typed: JsRef<JsValue> = JsRef::to_js(val.clone());
    assert_eq!(typed.as_ref(), &val);
}

#[wasm_bindgen_test]
fn typed_js_value_fromd_value() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());
    assert!(typed.into_value().is_string());
}

#[wasm_bindgen_test]
fn typed_js_value_into_js_value() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val);
    let back: JsValue = typed.into_value();
    assert_eq!(back.as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_as_value() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val);

    // as_value() borrows and returns &JsValue
    let value_ref: &JsValue = typed.as_value();
    assert!(value_ref.is_string());
    assert_eq!(value_ref.as_string().unwrap(), "hello");

    // Original typed value is still usable
    assert_eq!(typed.from_js(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_to_value() {
    let str_val = JsString::from("world");
    let typed: JsRef<JsString> = JsRef::to_js(str_val);

    // to_value() clones and returns JsValue
    let cloned: JsValue = typed.to_value();
    assert!(cloned.is_string());
    assert_eq!(cloned.as_string().unwrap(), "world");

    // Original typed value is still usable
    assert_eq!(typed.from_js(), "world");
}

#[wasm_bindgen_test]
fn typed_js_value_deref() {
    let str_val = JsString::from("test");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());

    let deref_val: &JsValue = typed.as_ref();
    assert!(deref_val.is_string());
    assert_eq!(deref_val.as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_success() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());

    let inner: JsString = typed.from_js();
    assert_eq!(inner, str_val);
}

#[cfg(not(target_feature = "exception-handling"))]
#[wasm_bindgen_test]
#[should_panic]
fn typed_js_value_unwrap_unchecked_behavior() {
    // from_js() uses wbg_cast which does perform runtime validation
    // This test demonstrates that from_js() will panic when types don't match
    let num_val = JsRef::to_js(42);
    let typed: JsRef<String> = unsafe { core::mem::transmute(num_val) };

    // This will panic because wbg_cast validates the type at runtime
    // Note: The error message is logged to stderr but not part of the panic message
    let _result: String = typed.from_js();
}

#[wasm_bindgen_test]
fn typed_js_value_try_from_js_success() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());

    let result = typed.try_from_js();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_try_from_js_failure() {
    let num_val = JsValue::from(42);
    let typed: JsRef<JsString> = num_val.cast_unchecked();

    let result = typed.try_from_js();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_array() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    arr.push(&JsValue::from(3));

    let typed: JsRef<Array> = JsRef::to_js(arr.clone());

    assert!(typed.as_ref().is_array());

    let inner = typed.from_js();
    assert_eq!(inner.length(), 3);
}

#[wasm_bindgen_test]
fn typed_js_value_object() {
    let obj = Object::new();
    let typed: JsRef<Object> = JsRef::to_js(obj.clone());

    assert!(typed.as_ref().is_object());

    let inner: Object = typed.from_js();
    assert_eq!(inner, obj);
}

#[wasm_bindgen_test]
fn typed_js_value_generic_function() {
    fn process_typed<T>(typed: JsRef<T>) -> JsRef<T>
    where
        T: TryFromJsValue + Into<JsValue> + JsCast,
    {
        assert!(!typed.as_ref().is_undefined());
        typed
    }

    let str_val = JsString::from("test");
    let typed: JsRef<JsString> = JsRef::to_js_as("test");
    let result = process_typed(typed);

    assert_eq!(result.from_js(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_wrap_as() {
    // to_js_as allows conversion from types that implement Into<T>
    // For example, &str -> JsString
    let typed: JsRef<JsString> = JsRef::to_js_as("hello");
    assert!(typed.as_ref().is_string());
    assert_eq!(typed.as_ref().as_string().unwrap(), "hello");

    // Compare with direct to_js which requires exact type
    let js_str = JsString::from("world");
    let typed2: JsRef<JsString> = JsRef::to_js(js_str);
    assert_eq!(typed2.as_ref().as_string().unwrap(), "world");
}

#[wasm_bindgen_test]
fn typed_js_value_clone() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());

    let cloned = typed.clone();
    assert_eq!(cloned.from_js(), str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_unchecked_into() {
    let js_val = JsValue::from_str("test");
    let typed: JsRef<JsString> = js_val.cast_unchecked();

    assert_eq!(typed.as_ref().as_string().unwrap(), "test");
}

#[wasm_bindgen_test]
fn typed_js_value_jscast_dyn_into() {
    let js_val = JsValue::from_str("test");
    let js_string: JsString = js_val.dyn_into().unwrap();
    let typed: JsRef<JsString> = JsRef::to_js(js_string);
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
    let val = JsRef::to_js(123u32);
    assert_eq!(val.from_js(), 123);

    let val2 = JsRef::to_js(true);
    assert_eq!(val2.from_js(), true);

    let val3 = JsRef::to_js("hello".to_string());

    // TODO: Investigate for exception handling
    #[cfg(not(target_feature = "exception-handling"))]
    assert_eq!(val3.from_js(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_debug() {
    let str_val = JsString::from("test");
    let typed: JsRef<JsString> = JsRef::to_js(str_val);

    let debug_str = format!("{:?}", typed);
    assert!(debug_str.contains("JsRef"));
    assert!(debug_str.contains("JsString"));
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq() {
    let str_val = JsString::from("hello");
    let typed: JsRef<JsString> = JsRef::to_js(str_val.clone());
    let js_val: JsValue = str_val.into();

    assert_eq!(js_val, *typed.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_unchecked_creation() {
    let num_val = JsValue::from(42);
    let wrong_typed: JsRef<JsString> = num_val.cast_unchecked();

    assert!(!wrong_typed.as_ref().is_string());
    assert!(wrong_typed.as_ref().as_f64().is_some());
}

#[wasm_bindgen_test]
fn typed_js_value_unwrap_unchecked() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.clone().into();
    let typed: JsRef<JsString> = js_val.cast_unchecked();

    let inner: JsString = typed.from_js();
    assert_eq!(inner, str_val);
}

#[wasm_bindgen_test]
fn typed_js_value_convert_between_types() {
    let str_val = JsString::from("hello");
    let typed_string: JsRef<JsString> = str_val.clone().to_js();

    let typed_object: JsRef<Object> = typed_string.clone().cast_unchecked();
    assert!(typed_object.as_ref().is_string());

    let typed_jsvalue: JsRef<JsValue> = typed_string.clone().cast_unchecked();
    assert!(typed_jsvalue.as_ref().is_string());
    let val: JsValue = typed_jsvalue.from_js();
    assert_eq!(val.as_string().unwrap(), "hello");

    let typed_string2: JsRef<JsString> = str_val.to_js();
    let result: Result<Array, _> = typed_string2.into_value().dyn_into();
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn typed_js_value_convert_upcast() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));

    let typed_array: JsRef<Array> = arr.clone().to_js();

    let typed_object: JsRef<Object> = typed_array.cast_unchecked();
    assert!(typed_object.as_ref().is_object());

    let obj: Object = typed_object.from_js();
    assert_eq!(obj, arr.into());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_same_type() {
    let typed1: JsRef<String> = JsRef::to_js(String::from("hello"));
    let typed2: JsRef<JsString> = JsRef::to_js(JsString::from("hello"));
    let typed3: JsRef<JsString> = JsRef::to_js(JsString::from("world"));

    assert_eq!(&typed1, &typed2);
    assert_ne!(typed1, typed3);
    assert_ne!(typed2, typed3);

    let typed4: JsRef<JsString> = typed2.cast_unchecked();
    assert_eq!(typed1, typed4);

    // TODO: Investigate for exception handling
    #[cfg(not(target_feature = "exception-handling"))]
    assert_eq!(typed1.from_js(), "hello");
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_different_types() {
    let str_val = JsString::from("test");
    let js_val: JsValue = str_val.into();

    let typed_string: JsRef<JsString> = js_val.clone().cast_unchecked();
    let typed_object: JsRef<Object> = js_val.clone().cast_unchecked();
    let typed_jsvalue: JsRef<JsValue> = js_val.cast_unchecked();

    assert_eq!(typed_string, typed_string.clone());

    assert_eq!(typed_string.as_ref(), typed_object.as_ref());
    assert_eq!(typed_object.as_ref(), typed_jsvalue.as_ref());
}

#[wasm_bindgen_test]
fn typed_js_value_partial_eq_with_primitives() {
    let num1 = JsRef::to_js(42u32);
    let num2: JsRef<u32> = JsRef::to_js(42u32);
    let num3: JsRef<u32> = JsRef::to_js(99u32);

    assert_eq!(num1, num2);
    assert_ne!(num1, num3);

    let bool1: JsRef<bool> = JsRef::to_js(true);
    let bool2: JsRef<bool> = JsRef::to_js(true);
    let bool3: JsRef<bool> = JsRef::to_js(false);

    assert_eq!(bool1, bool2);
    assert_ne!(bool1, bool3);
}

#[wasm_bindgen_test]
fn typed_js_value_doc_example() {
    let typed_string: JsRef<String> = JsRef::to_js(String::from("test"));

    // Checked cast to another type via into_value() + JsCast (returns Result)
    let Ok(str): Result<JsString, _> = typed_string.into_value().dyn_into() else {
        panic!("Invalid input");
    };

    assert_eq!(&str, "test");

    // Unchecked cast (zero-cost, but unsafe if types don't match)
    let typed_string: JsRef<String> = JsRef::to_js(String::from("test2"));
    let typed_jsstring: JsRef<JsString> = typed_string.cast_unchecked();
    // Works here, but would have paniced if the value had not been a string.
    let str: JsString = typed_jsstring.from_js();

    assert_eq!(str, JsString::from("test2"));
}

#[wasm_bindgen_test]
fn typed_to_untyped_value_passing() {
    let typed_string: JsRef<String> = JsRef::to_js(String::from("test"));
    assert!(js_accepts_untyped_value(typed_string.into_value()));

    let typed_jsstring: JsRef<JsString> = JsRef::to_js(JsString::from("hello"));
    assert!(js_accepts_untyped_value(typed_jsstring.into_value()));

    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let typed_array: JsRef<Array> = JsRef::to_js(arr);
    assert!(!js_accepts_untyped_value(typed_array.into_value()));

    let typed_num: JsRef<u32> = JsRef::to_js(42u32);
    assert!(!js_accepts_untyped_value(typed_num.into_value()));
}

#[wasm_bindgen_test]
fn typed_to_untyped_ref_passing() {
    let obj = Object::new();
    let typed_object: JsRef<Object> = JsRef::to_js(obj);
    assert!(js_accepts_untyped_ref(typed_object.as_ref()));

    let arr = Array::new();
    let typed_array: JsRef<Array> = JsRef::to_js(arr);
    assert!(js_accepts_untyped_ref(typed_array.as_ref()));

    let typed_jsstring: JsRef<JsString> = JsRef::to_js(JsString::from("test"));
    assert!(!js_accepts_untyped_ref(typed_jsstring.as_ref()));

    let typed_bool: JsRef<bool> = JsRef::to_js(true);
    assert!(!js_accepts_untyped_ref(typed_bool.as_ref()));
}

#[wasm_bindgen_test]
fn untyped_to_typed_function_passing() {
    // Cast untyped JsValue to typed JsRef<JsString> to pass to typed function
    let untyped = JsValue::from_str("test");
    let typed_ref: &JsRef<JsString> = untyped.cast_ref_unchecked();
    let result = js_accepts_typed_ref(typed_ref);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Cast untyped JsValue to typed JsRef<JsString> by value
    let typed_val: JsRef<JsString> = untyped.cast_unchecked();
    let result = js_accepts_typed_value(typed_val);
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "test");

    // Typed values can be cast to the expected type
    let typed: JsRef<String> = String::from("hello").to_js();
    let result = js_accepts_typed_ref(&typed.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "hello");

    let typed2: JsRef<String> = String::from("world").to_js();
    let result = js_accepts_typed_value(typed2.cast_unchecked());
    assert!(result.is_string());
    assert_eq!(result.as_string().unwrap(), "world");
}

#[wasm_bindgen_test]
fn typed_js_value_ref_from_wasm_abi() {
    use js_sys::Reflect;

    let obj = Object::new();
    let typed_obj: JsRef<Object> = obj.to_js();

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
    let typed_str: JsRef<JsString> = str_val.to_js();

    let untyped_ref: &JsValue = typed_str.as_ref();
    assert!(!js_test_ref_from_wasm_abi(untyped_ref));
    assert!(js_test_ref_from_wasm_abi(untyped_ref));

    assert!(typed_str.as_ref().is_string());
    assert_eq!(typed_str.as_ref().as_string().unwrap(), "hello");
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_primitives() {
    let val: JsRef<u32> = 42u32.to_js();
    assert_eq!(val.from_js(), 42u32);

    let val: JsRef<i32> = (-42i32).to_js();
    assert_eq!(val.from_js(), -42i32);

    #[cfg(not(target_feature = "exception-handling"))]
    {
        let val: JsRef<f64> = 3.14f64.to_js();
        assert_eq!(val.from_js(), 3.14f64);
    }

    let val: JsRef<bool> = true.to_js();
    assert_eq!(val.from_js(), true);

    let val: JsRef<bool> = false.to_js();
    assert_eq!(val.from_js(), false);

    #[cfg(not(target_feature = "exception-handling"))]
    {
        let val: JsRef<String> = String::from("hello").to_js();
        assert_eq!(val.from_js(), "hello");
    }
}

#[wasm_bindgen_test]
fn static_wrap_unwrap_js_types() {
    let js_str = JsString::from("test");
    let val: JsRef<JsString> = js_str.clone().to_js();
    assert_eq!(val.from_js(), js_str);

    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    let val: JsRef<Array> = arr.clone().to_js();
    let unwrapped = val.from_js();
    assert_eq!(unwrapped.length(), 2);

    let obj = Object::new();
    let val: JsRef<Object> = obj.clone().to_js();
    assert_eq!(val.from_js(), obj);
}

// TODO: Investigate for exception handling
#[cfg(not(target_feature = "exception-handling"))]
#[wasm_bindgen_test]
fn static_wrap_unwrap_vectors() {
    let vec_u8 = vec![1u8, 2u8, 3u8, 4u8];
    let val: JsRef<Vec<u8>> = vec_u8.clone().to_js();
    assert_eq!(val.from_js(), vec_u8);

    let vec_i32 = vec![1i32, -2i32, 3i32, -4i32];
    let val: JsRef<Vec<i32>> = vec_i32.clone().to_js();
    assert_eq!(val.from_js(), vec_i32);

    let vec_f64 = vec![1.1f64, 2.2f64, 3.3f64];
    let val: JsRef<Vec<f64>> = vec_f64.clone().to_js();
    assert_eq!(val.from_js(), vec_f64);

    let vec_js = vec![
        JsValue::from(1),
        JsValue::from_str("hello"),
        JsValue::from(true),
    ];
    let val: JsRef<Vec<JsValue>> = vec_js.clone().to_js();
    let unwrapped = val.from_js();
    assert_eq!(unwrapped.len(), 3);
    assert_eq!(unwrapped[0].as_f64().unwrap(), 1.0);
    assert_eq!(unwrapped[1].as_string().unwrap(), "hello");
    assert_eq!(unwrapped[2].as_bool().unwrap(), true);
}

// TODO: Investigate for exception handling
#[cfg(not(target_feature = "exception-handling"))]
#[wasm_bindgen_test]
fn static_wrap_unwrap_box_types() {
    let boxed: Box<[u8]> = vec![1u8, 2u8, 3u8].into_boxed_slice();
    let val: JsRef<Box<[u8]>> = boxed.clone().to_js();
    assert_eq!(val.from_js(), boxed);

    let boxed: Box<[i32]> = vec![10i32, 20i32, 30i32].into_boxed_slice();
    let val: JsRef<Box<[i32]>> = boxed.clone().to_js();
    assert_eq!(val.from_js(), boxed);

    let boxed: Box<[f32]> = vec![1.5f32, 2.5f32, 3.5f32].into_boxed_slice();
    let val: JsRef<Box<[f32]>> = boxed.clone().to_js();
    assert_eq!(val.from_js(), boxed);
}

// TODO: Investigate for exception handling
#[cfg(not(target_feature = "exception-handling"))]
#[wasm_bindgen_test]
#[should_panic]
fn static_unwrap_wrong_type_panic() {
    let val = 42u32.to_js();
    let wrong: JsRef<String> = unsafe { core::mem::transmute(val) };
    let _: String = wrong.from_js(); // Should panic
}

// TODO: Investigate for exception handling
#[cfg(not(target_feature = "exception-handling"))]
#[wasm_bindgen_test]
fn static_wrap_round_trip_through_js() {
    // Test that values can round-trip through JavaScript

    let original = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let wrapped: JsRef<Vec<u32>> = original.clone().to_js();

    // Pass to JS and back
    let js_val: JsValue = wrapped.into_value();
    let re_wrapped: JsRef<Vec<u32>> = js_val.cast_unchecked();

    let unwrapped = re_wrapped.from_js();
    assert_eq!(unwrapped, original);
}
