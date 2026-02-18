use js_sys::{JsOption, Null, Undefined};
use js_sys::{JsString, Number, Object};
use wasm_bindgen::convert::Upcast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/nullable.js")]
extern "C" {
    fn return_null() -> JsOption<Number>;
    fn return_undefined() -> JsOption<Number>;
    fn return_number() -> JsOption<Number>;
    fn return_string() -> JsOption<JsString>;

    fn take_nullable_null(val: JsOption<Number>);
    fn take_nullable_value(val: JsOption<Number>);
    fn take_nullable_number(val: JsOption<Number>);
    fn take_nullable_string(val: JsOption<JsString>);

    fn test_nullable_exports();
}

#[wasm_bindgen_test]
fn test_new() {
    let empty: JsOption<Number> = JsOption::new();
    assert!(empty.is_empty());
}

#[wasm_bindgen_test]
fn test_wrap() {
    let num = JsOption::wrap(Number::from(42));
    assert!(!num.is_empty());
}

#[wasm_bindgen_test]
fn test_from_option_some() {
    let opt = Some(Number::from(123));
    let nullable = JsOption::from_option(opt);
    assert!(!nullable.is_empty());
    assert_eq!(nullable.unwrap().value_of(), 123.0);
}

#[wasm_bindgen_test]
fn test_from_option_none() {
    let opt: Option<Number> = None;
    let nullable = JsOption::from_option(opt);
    assert!(nullable.is_empty());
}

#[wasm_bindgen_test]
fn test_is_empty_null() {
    let val = return_null();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_is_empty_undefined() {
    let val = return_undefined();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_is_empty_value() {
    let val = return_number();
    assert!(!val.is_empty());
}

#[wasm_bindgen_test]
fn test_as_option_some() {
    let val = return_number();
    let opt = val.as_option();
    assert!(opt.is_some());
    assert_eq!(opt.unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_as_option_none() {
    let val = return_null();
    let opt = val.as_option();
    assert!(opt.is_none());
}

#[wasm_bindgen_test]
fn test_into_option_some() {
    let val = return_number();
    let opt = val.into_option();
    assert!(opt.is_some());
    assert_eq!(opt.unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_into_option_none() {
    let val = return_null();
    let opt = val.into_option();
    assert!(opt.is_none());
}

#[wasm_bindgen_test]
fn test_unwrap_success() {
    let val = return_number();
    let num = val.unwrap();
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
#[should_panic(expected = "called `JsOption::unwrap()` on an empty value")]
fn test_unwrap_panic() {
    let val = return_null();
    val.unwrap();
}

#[wasm_bindgen_test]
fn test_expect_success() {
    let val = return_number();
    let num = val.expect("should have value");
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
#[should_panic(expected = "custom error message")]
fn test_expect_panic() {
    let val = return_null();
    val.expect("custom error message");
}

#[wasm_bindgen_test]
fn test_unwrap_or_default() {
    let val = return_null();
    let num = val.unwrap_or_default();
    // Number::default() is Number::from(0)
    assert_eq!(num.value_of(), 0.0);

    let val = return_number();
    let num = val.unwrap_or_default();
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_unwrap_or_else() {
    let val = return_null();
    let num = val.unwrap_or_else(|| Number::from(99));
    assert_eq!(num.value_of(), 99.0);

    let val = return_number();
    let num = val.unwrap_or_else(|| Number::from(99));
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_import_null() {
    let val = return_null();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_import_undefined() {
    let val = return_undefined();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_import_value() {
    let val = return_number();
    assert!(!val.is_empty());
    assert_eq!(val.unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_import_string() {
    let val = return_string();
    assert!(!val.is_empty());
    assert_eq!(val.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn test_export_null() {
    take_nullable_null(JsOption::new());
}

#[wasm_bindgen_test]
fn test_export_value() {
    take_nullable_value(JsOption::wrap(Number::from(123)));
}

#[wasm_bindgen_test]
fn test_js_calls_rust() {
    test_nullable_exports();
}

// Exported functions for JS to call
#[wasm_bindgen]
pub fn rust_return_nullable_null() -> JsOption<Number> {
    JsOption::new()
}

#[wasm_bindgen]
pub fn rust_return_nullable_value() -> JsOption<Number> {
    JsOption::wrap(Number::from(456))
}

#[wasm_bindgen]
pub fn rust_take_nullable_null(val: JsOption<Number>) {
    assert!(val.is_empty());
}

#[wasm_bindgen]
pub fn rust_take_nullable_value(val: JsOption<Number>) {
    assert!(!val.is_empty());
    assert_eq!(val.unwrap().value_of(), 789.0);
}

#[wasm_bindgen_test]
fn test_debug_value() {
    let val = JsOption::wrap(Number::from(42));
    let debug_str = format!("{:?}", val);
    assert!(debug_str.contains("Number"));
    assert!(debug_str.contains("42"));
}

#[wasm_bindgen_test]
fn test_debug_null() {
    let val: JsOption<Number> = JsOption::new();
    let debug_str = format!("{:?}", val);
    assert!(debug_str.contains("Number"));
    assert!(debug_str.contains("null"));
}

#[wasm_bindgen_test]
fn test_default() {
    let val: JsOption<Number> = Default::default();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_nullable_in_generic_context() {
    fn process<T: wasm_bindgen::convert::JsGeneric>(nullable: JsOption<T>) -> bool {
        nullable.is_empty()
    }

    let empty: JsOption<Number> = JsOption::new();
    assert!(process(empty));

    let filled = JsOption::wrap(Number::from(1));
    assert!(!process(filled));
}

// ============================================================================
// Upcast tests
// ============================================================================

#[wasm_bindgen_test]
fn test_upcast_value_to_nullable() {
    // A Number can upcast to JsOption<Number>
    let num = Number::from(42);
    let nullable: JsOption<Number> = num.upcast_into();
    assert!(!nullable.is_empty());
    assert_eq!(nullable.unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_upcast_string_to_nullable() {
    // A JsString can upcast to JsOption<JsString>
    let s = JsString::from("hello");
    let nullable: JsOption<JsString> = s.upcast_into();
    assert!(!nullable.is_empty());
    assert_eq!(nullable.unwrap(), "hello");
}

#[wasm_bindgen_test]
fn test_upcast_null_to_nullable() {
    // Null can upcast to JsOption<T> for any T
    let null = Null::NULL;
    let nullable: JsOption<Number> = null.upcast_into();
    assert!(nullable.is_empty());
}

#[wasm_bindgen_test]
fn test_upcast_undefined_to_nullable() {
    // Undefined can upcast to JsOption<T> for any T
    let undef = Undefined::UNDEFINED;
    let nullable: JsOption<Number> = undef.upcast_into();
    assert!(nullable.is_empty());
}

#[wasm_bindgen_test]
fn test_upcast_null_to_different_nullable_types() {
    // Null upcasts to JsOption of any type
    let null = Null::NULL;

    let nullable_num: JsOption<Number> = null.upcast_into();
    assert!(nullable_num.is_empty());

    let null = Null::NULL;
    let nullable_str: JsOption<JsString> = null.upcast_into();
    assert!(nullable_str.is_empty());

    let null = Null::NULL;
    let nullable_obj: JsOption<Object> = null.upcast_into();
    assert!(nullable_obj.is_empty());
}

#[wasm_bindgen_test]
fn test_upcast_in_function_call() {
    // Test using upcast to pass a value to a function expecting JsOption
    let num = Number::from(123);
    take_nullable_number(num.upcast_into());

    let s = JsString::from("test");
    take_nullable_string(s.upcast_into());
}

#[wasm_bindgen_test]
fn test_upcast_null_in_function_call() {
    // Test using upcast to pass Null to a function expecting JsOption
    take_nullable_null(Null::NULL.upcast_into());
}

#[wasm_bindgen_test]
fn test_upcast_undefined_in_function_call() {
    // Test using upcast to pass Undefined to a function expecting JsOption
    take_nullable_null(Undefined::UNDEFINED.upcast_into());
}

// Helper function that accepts JsOption via upcast
fn accepts_nullable_number(val: JsOption<Number>) -> Option<f64> {
    val.into_option().map(|n| n.value_of())
}

#[wasm_bindgen_test]
fn test_upcast_with_helper_function() {
    // Pass a Number directly via upcast
    let result = accepts_nullable_number(Number::from(99).upcast_into());
    assert_eq!(result, Some(99.0));

    // Pass Null via upcast
    let result = accepts_nullable_number(Null::NULL.upcast_into());
    assert_eq!(result, None);

    // Pass Undefined via upcast
    let result = accepts_nullable_number(Undefined::UNDEFINED.upcast_into());
    assert_eq!(result, None);
}
