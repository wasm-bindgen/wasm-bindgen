use js_sys::{JsNullable, JsOption, Null, Undefined};
use js_sys::{JsString, Number, Object};
use wasm_bindgen::convert::Upcast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(inline_js = "
export const js_optional_default = (value = 42) => value;
")]
extern "C" {
    // Imported with an erasable generic param.
    #[wasm_bindgen(js_name = js_optional_default)]
    fn js_optional_default_generic<T>(value: JsOption<T>) -> T;

    // Same JS function but with a concrete Option<i32> param.
    // Both should have the same observable behaviour.
    #[wasm_bindgen(js_name = js_optional_default)]
    fn js_optional_default_concrete(value: Option<i32>) -> i32;
}

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

    #[wasm_bindgen(js_name = return_null)]
    fn js_nullable_return_null() -> JsNullable<Number>;
    #[wasm_bindgen(js_name = return_undefined)]
    fn js_nullable_return_undefined() -> JsNullable<Number>;
    #[wasm_bindgen(js_name = return_number)]
    fn js_nullable_return_number() -> JsNullable<Number>;
    #[wasm_bindgen(js_name = return_string)]
    fn js_nullable_return_string() -> JsNullable<JsString>;

    fn take_js_nullable_null(val: JsNullable<Number>);
    fn take_js_nullable_value(val: JsNullable<Number>);

    fn test_js_nullable_exports();

    fn call_with_null_undefined_and_value(f: &Closure<dyn FnMut(JsNullable<Number>)>);
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
    // Strict semantics: `null` is a present value, not empty.
    let val = return_null();
    assert!(!val.is_empty());
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
    let val = return_undefined();
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
    let val = return_undefined();
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
    let val = return_undefined();
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
    let val = return_undefined();
    val.expect("custom error message");
}

#[wasm_bindgen_test]
fn test_unwrap_or_default() {
    let val = return_undefined();
    let num = val.unwrap_or_default();
    // Number::default() is Number::from(0)
    assert_eq!(num.value_of(), 0.0);

    let val = return_number();
    let num = val.unwrap_or_default();
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_unwrap_or_else() {
    let val = return_undefined();
    let num = val.unwrap_or_else(|| Number::from(99));
    assert_eq!(num.value_of(), 99.0);

    let val = return_number();
    let num = val.unwrap_or_else(|| Number::from(99));
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_import_null() {
    // Strict semantics: JS `null` is a present value, not empty.
    let val = return_null();
    assert!(!val.is_empty());
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
    assert!(debug_str.contains("undefined"));
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
fn test_upcast_undefined_to_nullable() {
    // Undefined can upcast to JsOption<T> for any T
    let undef = Undefined::UNDEFINED;
    let nullable: JsOption<Number> = undef.upcast_into();
    assert!(nullable.is_empty());
}

#[wasm_bindgen_test]
fn test_upcast_undefined_to_different_nullable_types() {
    // Undefined upcasts to JsOption of any type
    let nullable_num: JsOption<Number> = Undefined::UNDEFINED.upcast_into();
    assert!(nullable_num.is_empty());

    let nullable_str: JsOption<JsString> = Undefined::UNDEFINED.upcast_into();
    assert!(nullable_str.is_empty());

    let nullable_obj: JsOption<Object> = Undefined::UNDEFINED.upcast_into();
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

    // Pass Undefined via upcast
    let result = accepts_nullable_number(Undefined::UNDEFINED.upcast_into());
    assert_eq!(result, None);
}

// ============================================================================
// JsNullable tests
// ============================================================================

#[wasm_bindgen_test]
fn test_js_nullable_new() {
    let empty: JsNullable<Number> = JsNullable::new();
    assert!(empty.is_empty());
    // Canonical empty is `null`
    assert!(JsValue::from(empty).is_null());
}

#[wasm_bindgen_test]
fn test_js_nullable_wrap() {
    let num = JsNullable::wrap(Number::from(42));
    assert!(!num.is_empty());
    assert_eq!(num.unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_js_nullable_from_option() {
    let nullable = JsNullable::from_option(Some(Number::from(123)));
    assert!(!nullable.is_empty());
    assert_eq!(nullable.unwrap().value_of(), 123.0);

    let nullable = JsNullable::from_option(None::<Number>);
    assert!(nullable.is_empty());
    assert!(JsValue::from(nullable).is_null());
}

#[wasm_bindgen_test]
fn test_js_nullable_null_is_empty() {
    // The WebIDL `T?` contract: JS `null` is absent.
    let val = js_nullable_return_null();
    assert!(val.is_empty());
    assert!(val.into_option().is_none());
}

#[wasm_bindgen_test]
fn test_js_nullable_undefined_is_empty() {
    // WebIDL ES conversion coerces `undefined` to `null` at nullable
    // positions, so `undefined` is also absent.
    let val = js_nullable_return_undefined();
    assert!(val.is_empty());
    assert!(val.as_option().is_none());
}

#[wasm_bindgen_test]
fn test_js_nullable_value() {
    let val = js_nullable_return_number();
    assert!(!val.is_empty());
    assert_eq!(val.as_option().unwrap().value_of(), 42.0);
    assert_eq!(val.into_option().unwrap().value_of(), 42.0);
}

#[wasm_bindgen_test]
fn test_js_nullable_string() {
    let val = js_nullable_return_string();
    assert_eq!(val.unwrap(), "hello");
}

#[wasm_bindgen_test]
#[should_panic(expected = "called `JsNullable::unwrap()` on an empty value")]
fn test_js_nullable_unwrap_panic() {
    js_nullable_return_null().unwrap();
}

#[wasm_bindgen_test]
fn test_js_nullable_unwrap_or() {
    let num = js_nullable_return_null().unwrap_or_default();
    assert_eq!(num.value_of(), 0.0);

    let num = js_nullable_return_undefined().unwrap_or_else(|| Number::from(99));
    assert_eq!(num.value_of(), 99.0);
}

#[wasm_bindgen_test]
fn test_js_nullable_debug() {
    let val = JsNullable::wrap(Number::from(42));
    let debug_str = format!("{:?}", val);
    assert!(debug_str.contains("Number"));
    assert!(debug_str.contains("42"));

    let val: JsNullable<Number> = JsNullable::new();
    let debug_str = format!("{:?}", val);
    assert!(debug_str.contains("null"));
}

#[wasm_bindgen_test]
fn test_js_nullable_default() {
    let val: JsNullable<Number> = Default::default();
    assert!(val.is_empty());
}

#[wasm_bindgen_test]
fn test_js_nullable_upcasts() {
    // A value upcasts to JsNullable of its own type
    let nullable: JsNullable<Number> = Number::from(42).upcast_into();
    assert_eq!(nullable.unwrap().value_of(), 42.0);

    // Null and Undefined both upcast to empty
    let nullable: JsNullable<Number> = Null::NULL.upcast_into();
    assert!(nullable.is_empty());
    let nullable: JsNullable<Number> = Undefined::UNDEFINED.upcast_into();
    assert!(nullable.is_empty());

    // JsOption<T> upcasts to JsNullable<T>
    let opt: JsOption<Number> = JsOption::wrap(Number::from(7));
    let nullable: JsNullable<Number> = opt.upcast_into();
    assert_eq!(nullable.unwrap().value_of(), 7.0);
}

#[wasm_bindgen_test]
fn test_js_nullable_export() {
    take_js_nullable_null(JsNullable::new());
    take_js_nullable_value(JsNullable::wrap(Number::from(321)));
}

#[wasm_bindgen_test]
fn test_js_nullable_js_calls_rust() {
    test_js_nullable_exports();
}

// Exported functions for JS to call
#[wasm_bindgen]
pub fn rust_return_js_nullable_null() -> JsNullable<Number> {
    JsNullable::new()
}

#[wasm_bindgen]
pub fn rust_return_js_nullable_value() -> JsNullable<Number> {
    JsNullable::wrap(Number::from(654))
}

#[wasm_bindgen]
pub fn rust_take_js_nullable_empty(val: JsNullable<Number>) {
    assert!(val.is_empty());
}

#[wasm_bindgen]
pub fn rust_take_js_nullable_value(val: JsNullable<Number>) {
    assert_eq!(val.unwrap().value_of(), 987.0);
}

#[wasm_bindgen_test]
fn test_option_vs_js_option_compat() {
    // A helper to ensure that concrete and generic options behave the same when passed to JS.
    fn test_value(option: Option<i32>, expected_result: i32) {
        let result_option = js_optional_default_concrete(option);
        assert_eq!(result_option, expected_result);

        let js_opt = JsOption::from_option(option.map(Number::from));
        let result_js_option = js_optional_default_generic(js_opt);
        assert_eq!(result_js_option, expected_result);
    }

    // Option<i32> None -> `undefined` -> triggers JS default (42).
    test_value(None, 42);

    // Option<i32> Some(7) -> passes 7 to JS, no default.
    test_value(Some(7), 7);
}

// Closure variance with JsNullable arguments: JsNullable participates in the
// same contravariant argument casts as JsOption.
mod nullable_closure_variance {
    use super::*;
    use std::cell::Cell;
    use std::panic::AssertUnwindSafe;
    use std::rc::Rc;

    #[wasm_bindgen_test]
    fn arg_contravariance_jsvalue_to_nullable() {
        let closure: Closure<dyn Fn(JsValue)> = Closure::new(|_: JsValue| {});
        let _narrower: &Closure<dyn Fn(JsNullable<Number>)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_nullable_jsvalue_to_nullable_number() {
        let closure: Closure<dyn Fn(JsNullable<JsValue>)> =
            Closure::new(|_: JsNullable<JsValue>| {});
        let _narrower: &Closure<dyn Fn(JsNullable<Number>)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_nullable_to_option() {
        // JsNullable emptiness (null | undefined) is a superset of JsOption
        // emptiness (undefined), so a JsNullable-accepting closure can be used
        // where a JsOption-accepting one is expected -- but not vice versa.
        let closure: Closure<dyn Fn(JsNullable<Number>)> = Closure::new(|_: JsNullable<Number>| {});
        let _narrower: &Closure<dyn Fn(JsOption<Number>)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_nullable_jsvalue_to_i32() {
        let closure: Closure<dyn Fn(JsNullable<JsValue>)> =
            Closure::new(|_: JsNullable<JsValue>| {});
        let _narrower: &Closure<dyn Fn(i32)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn return_covariance_number_to_nullable() {
        let closure: Closure<dyn Fn() -> Number> = Closure::new(|| Number::from(42));
        let _wider: &Closure<dyn Fn() -> JsNullable<Number>> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn function_to_nullable_jsvalue() {
        fn assert_upcast<T, U: wasm_bindgen::convert::UpcastFrom<T>>() {}
        assert_upcast::<js_sys::Function<fn() -> JsValue>, JsNullable<JsValue>>();
        assert_upcast::<js_sys::Function<fn() -> JsValue>, JsNullable<Object>>();
        assert_upcast::<js_sys::Function<fn(JsNullable<Number>) -> JsValue>, JsNullable<JsValue>>();
    }

    #[wasm_bindgen_test]
    fn invoke_nullable_arg_closure() {
        let calls = Rc::new(Cell::new((0u32, 0u32)));
        let calls2 = AssertUnwindSafe(calls.clone());
        let closure: Closure<dyn FnMut(JsNullable<Number>)> =
            Closure::new(move |val: JsNullable<Number>| {
                let (empty, present) = calls2.get();
                match val.into_option() {
                    None => calls2.set((empty + 1, present)),
                    Some(num) => {
                        assert_eq!(num.value_of(), 321.0);
                        calls2.set((empty, present + 1));
                    }
                }
            });
        call_with_null_undefined_and_value(&closure);
        // null and undefined are both empty; 321 is present.
        assert_eq!(calls.get(), (2, 1));
    }

    #[wasm_bindgen_test]
    fn invoke_upcast_jsvalue_closure_as_nullable() {
        let calls = Rc::new(Cell::new((0u32, 0u32, 0u32)));
        let calls2 = AssertUnwindSafe(calls.clone());
        let closure: Closure<dyn FnMut(JsValue)> = Closure::new(move |val: JsValue| {
            let (nulls, undefs, values) = calls2.get();
            if val.is_null() {
                calls2.set((nulls + 1, undefs, values));
            } else if val.is_undefined() {
                calls2.set((nulls, undefs + 1, values));
            } else {
                calls2.set((nulls, undefs, values + 1));
            }
        });
        call_with_null_undefined_and_value(closure.upcast());
        // The raw JsValue view distinguishes null from undefined.
        assert_eq!(calls.get(), (1, 1, 1));
    }
}
