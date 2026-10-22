//! `Nullable<T>`: `null`, `undefined` and a value cross the boundary intact
//! in both directions, for every `T` category and in every position. The JS
//! side checks with `=== null` and `=== undefined`.

use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::ptr::NonNull;
use std::rc::Rc;

use js_sys::{JsNullable, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/nullable_type.js")]
extern "C" {
    pub type NullableJsType;

    fn nullable_type_js_exports();
    fn nullable_type_closure_driver(
        f: &Closure<dyn FnMut(Nullable<u32>, Nullable<String>) -> Nullable<String>>,
    );

    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_str_ref(v: Nullable<&str>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_slice_ref(v: Nullable<&[u8]>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_string_slice_ref(v: Nullable<&[String]>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_jsvalue_ref(v: Nullable<&JsValue>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_closure_ref(v: Nullable<&Closure<dyn Fn()>>) -> String;
}

/// The imports for one `T`: `describe` reports which of the three states JS
/// received, and the `js_*` functions return each state.
macro_rules! imports {
    ($($module:ident: $ty:ty => $value:ident),* $(,)?) => {$(
        mod $module {
            use super::*;

            #[wasm_bindgen(module = "tests/wasm/nullable_type.js")]
            extern "C" {
                #[wasm_bindgen(js_name = nullable_type_describe)]
                pub fn describe(v: $ty) -> String;
                #[wasm_bindgen(js_name = nullable_type_js_null)]
                pub fn js_null() -> $ty;
                #[wasm_bindgen(js_name = nullable_type_js_undefined)]
                pub fn js_undefined() -> $ty;
                #[wasm_bindgen(js_name = $value)]
                pub fn js_value() -> $ty;
            }
        }
    )*};
}

imports! {
    n_u8: Nullable<u8> => nullable_type_js_number,
    n_i32: Nullable<i32> => nullable_type_js_number,
    n_u32: Nullable<u32> => nullable_type_js_number,
    n_usize: Nullable<usize> => nullable_type_js_number,
    n_f32: Nullable<f32> => nullable_type_js_float,
    n_f64: Nullable<f64> => nullable_type_js_float,
    n_i64: Nullable<i64> => nullable_type_js_bigint,
    n_u128: Nullable<u128> => nullable_type_js_bigint,
    n_nonnull: Nullable<NonNull<u32>> => nullable_type_js_number,
    n_ptr: Nullable<*const u8> => nullable_type_js_number,
    n_bool: Nullable<bool> => nullable_type_js_true,
    n_char: Nullable<char> => nullable_type_js_string,
    n_string: Nullable<String> => nullable_type_js_string,
    n_vec_u8: Nullable<Vec<u8>> => nullable_type_js_bytes,
    n_vec_bool: Nullable<Vec<bool>> => nullable_type_js_bools,
    n_vec_jsvalue: Nullable<Vec<JsValue>> => nullable_type_js_values,
    n_enum: Nullable<NullableColor> => nullable_type_js_number,
    n_string_enum: Nullable<NullableName> => nullable_type_js_beta,
    n_struct: Nullable<NullableThing> => nullable_type_js_thing,
    n_jsvalue: Nullable<JsValue> => nullable_type_js_object,
    n_js_type: Nullable<NullableJsType> => nullable_type_js_object,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NullableColor {
    Red = 0,
    Green = 1,
    Blue = 7,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NullableName {
    Alpha = "alpha",
    Beta = "beta",
}

#[wasm_bindgen]
pub struct NullableThing(pub u32);

#[wasm_bindgen]
impl NullableThing {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u32) -> NullableThing {
        NullableThing(n)
    }

    #[wasm_bindgen(getter)]
    pub fn n(&self) -> u32 {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct NullableFields {
    pub n: Nullable<u32>,
    #[wasm_bindgen(getter_with_clone)]
    pub s: Nullable<String>,
}

#[wasm_bindgen]
impl NullableFields {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NullableFields {
        NullableFields::default()
    }
}

/// An exported identity function per `T`, covering the export argument and
/// return positions.
macro_rules! exports {
    ($($name:ident: $ty:ty),* $(,)?) => {$(
        #[wasm_bindgen]
        pub fn $name(a: $ty) -> $ty {
            a
        }
    )*};
}

exports! {
    nullable_type_u8: Nullable<u8>,
    nullable_type_i32: Nullable<i32>,
    nullable_type_u32: Nullable<u32>,
    nullable_type_usize: Nullable<usize>,
    nullable_type_f32: Nullable<f32>,
    nullable_type_f64: Nullable<f64>,
    nullable_type_i64: Nullable<i64>,
    nullable_type_u128: Nullable<u128>,
    nullable_type_nonnull: Nullable<NonNull<u32>>,
    nullable_type_bool: Nullable<bool>,
    nullable_type_char: Nullable<char>,
    nullable_type_string: Nullable<String>,
    nullable_type_vec_u8: Nullable<Vec<u8>>,
    nullable_type_vec_bool: Nullable<Vec<bool>>,
    nullable_type_vec_jsvalue: Nullable<Vec<JsValue>>,
    nullable_type_enum: Nullable<NullableColor>,
    nullable_type_string_enum: Nullable<NullableName>,
    nullable_type_struct: Nullable<NullableThing>,
    nullable_type_jsvalue: Nullable<JsValue>,
    nullable_type_js_type: Nullable<NullableJsType>,
}

#[wasm_bindgen]
pub fn nullable_type_string_enum_name(a: Nullable<NullableName>) -> String {
    match a {
        Nullable::Null => "null",
        Nullable::Undefined => "undefined",
        Nullable::Value(NullableName::Alpha) => "alpha",
        Nullable::Value(NullableName::Beta) => "beta",
        Nullable::Value(_) => "invalid",
    }
    .into()
}

#[wasm_bindgen_test]
fn exports() {
    nullable_type_js_exports();
}

#[wasm_bindgen_test]
fn closure_args_and_return() {
    let calls = Rc::new(Cell::new(0));
    let f = {
        let calls = AssertUnwindSafe(calls.clone());
        Closure::new(move |n: Nullable<u32>, s: Nullable<String>| {
            calls.set(calls.get() + 1);
            n.map(|n| format!("{n} {s:?}"))
        })
    };
    nullable_type_closure_driver(&f);
    assert_eq!(calls.get(), 5);
}

fn states<T>(
    describe: fn(Nullable<T>) -> String,
    js_null: fn() -> Nullable<T>,
    js_undefined: fn() -> Nullable<T>,
    value: T,
) {
    assert_eq!(describe(Nullable::Null), "null");
    assert_eq!(describe(Nullable::Undefined), "undefined");
    assert_eq!(describe(Nullable::Value(value)), "value");
    assert!(js_null().is_null());
    assert!(js_undefined().is_undefined());
}

#[wasm_bindgen_test]
fn import_args_and_returns() {
    states(n_u8::describe, n_u8::js_null, n_u8::js_undefined, 3);
    assert_eq!(n_u8::js_value(), Nullable::Value(7));
    states(n_i32::describe, n_i32::js_null, n_i32::js_undefined, -3);
    assert_eq!(n_i32::js_value(), Nullable::Value(7));
    states(n_u32::describe, n_u32::js_null, n_u32::js_undefined, 3);
    assert_eq!(n_u32::js_value(), Nullable::Value(7));
    states(
        n_usize::describe,
        n_usize::js_null,
        n_usize::js_undefined,
        3,
    );
    assert_eq!(n_usize::js_value(), Nullable::Value(7));
    states(n_f32::describe, n_f32::js_null, n_f32::js_undefined, 1.5);
    assert_eq!(n_f32::js_value(), Nullable::Value(2.5));
    states(n_f64::describe, n_f64::js_null, n_f64::js_undefined, 1.5);
    assert_eq!(n_f64::js_value(), Nullable::Value(2.5));
    states(n_i64::describe, n_i64::js_null, n_i64::js_undefined, -3);
    assert_eq!(n_i64::js_value(), Nullable::Value(7));
    states(n_u128::describe, n_u128::js_null, n_u128::js_undefined, 3);
    assert_eq!(n_u128::js_value(), Nullable::Value(7));

    let ptr = NonNull::new(8 as *mut u32).unwrap();
    states(
        n_nonnull::describe,
        n_nonnull::js_null,
        n_nonnull::js_undefined,
        ptr,
    );
    assert_eq!(
        n_nonnull::js_value().unwrap(),
        NonNull::new(7 as *mut u32).unwrap()
    );
    states(
        n_ptr::describe,
        n_ptr::js_null,
        n_ptr::js_undefined,
        8 as *const u8,
    );
    assert_eq!(n_ptr::js_value().unwrap(), 7 as *const u8);

    states(
        n_bool::describe,
        n_bool::js_null,
        n_bool::js_undefined,
        false,
    );
    assert_eq!(n_bool::js_value(), Nullable::Value(true));
    states(n_char::describe, n_char::js_null, n_char::js_undefined, 'x');
    assert_eq!(n_char::js_value(), Nullable::Value('h'));
    states(
        n_string::describe,
        n_string::js_null,
        n_string::js_undefined,
        "hi".to_string(),
    );
    assert_eq!(n_string::js_value().unwrap(), "hi");

    states(
        n_vec_u8::describe,
        n_vec_u8::js_null,
        n_vec_u8::js_undefined,
        vec![1, 2],
    );
    assert_eq!(n_vec_u8::js_value().unwrap(), vec![1, 2, 3]);
    states(
        n_vec_bool::describe,
        n_vec_bool::js_null,
        n_vec_bool::js_undefined,
        vec![true],
    );
    assert_eq!(n_vec_bool::js_value().unwrap(), vec![true, false]);
    states(
        n_vec_jsvalue::describe,
        n_vec_jsvalue::js_null,
        n_vec_jsvalue::js_undefined,
        vec![JsValue::from(1)],
    );
    assert_eq!(n_vec_jsvalue::js_value().unwrap().len(), 2);

    states(
        n_enum::describe,
        n_enum::js_null,
        n_enum::js_undefined,
        NullableColor::Green,
    );
    assert_eq!(n_enum::js_value(), Nullable::Value(NullableColor::Blue));
    states(
        n_string_enum::describe,
        n_string_enum::js_null,
        n_string_enum::js_undefined,
        NullableName::Alpha,
    );
    assert_eq!(
        n_string_enum::js_value(),
        Nullable::Value(NullableName::Beta)
    );

    states(
        n_struct::describe,
        n_struct::js_null,
        n_struct::js_undefined,
        NullableThing(4),
    );
    assert_eq!(n_struct::js_value().map(|t| t.0), Nullable::Value(5));

    states(
        n_jsvalue::describe,
        n_jsvalue::js_null,
        n_jsvalue::js_undefined,
        JsValue::from(1),
    );
    assert!(n_jsvalue::js_value().unwrap().is_object());
    // `null` and `undefined` are JS values too, but a `Nullable` keeps them
    // as its empty states rather than as `Value(null)`.
    assert_eq!(n_jsvalue::describe(Nullable::Value(JsValue::NULL)), "null");
    assert_eq!(
        n_jsvalue::describe(Nullable::Value(JsValue::UNDEFINED)),
        "undefined"
    );
    states(
        n_js_type::describe,
        n_js_type::js_null,
        n_js_type::js_undefined,
        JsValue::from(1).unchecked_into(),
    );
    assert!(n_js_type::js_value().is_value());
}

#[wasm_bindgen_test]
fn import_ref_args() {
    assert_eq!(describe_str_ref(Nullable::Null), "null");
    assert_eq!(describe_str_ref(Nullable::Undefined), "undefined");
    assert_eq!(describe_str_ref(Nullable::Value("hi")), "value");
    assert_eq!(describe_slice_ref(Nullable::Null), "null");
    assert_eq!(describe_slice_ref(Nullable::Undefined), "undefined");
    assert_eq!(describe_slice_ref(Nullable::Value(&[1, 2])), "value");
    assert_eq!(describe_string_slice_ref(Nullable::Null), "null");
    assert_eq!(describe_string_slice_ref(Nullable::Undefined), "undefined");
    assert_eq!(
        describe_string_slice_ref(Nullable::Value(&["a".to_string()])),
        "value"
    );
    assert_eq!(describe_jsvalue_ref(Nullable::Null), "null");
    assert_eq!(describe_jsvalue_ref(Nullable::Undefined), "undefined");
    assert_eq!(
        describe_jsvalue_ref(Nullable::Value(&JsValue::from(1))),
        "value"
    );
    let f = Closure::new(|| ());
    assert_eq!(describe_closure_ref(Nullable::Null), "null");
    assert_eq!(describe_closure_ref(Nullable::Undefined), "undefined");
    assert_eq!(describe_closure_ref(Nullable::Value(&f)), "value");
}

#[wasm_bindgen_test]
fn js_nullable_conversions() {
    let null: JsNullable<Number> = Nullable::<Number>::Null.into();
    assert!(JsValue::is_null(&null));
    let undefined: JsNullable<Number> = Nullable::<Number>::Undefined.into();
    assert!(JsValue::is_undefined(&undefined));
    let value: JsNullable<Number> = Nullable::Value(Number::from(3)).into();
    assert_eq!(value.as_option().unwrap().value_of(), 3.0);

    assert!(Nullable::<Number>::from(null).is_null());
    assert!(Nullable::<Number>::from(undefined).is_undefined());
    assert_eq!(Nullable::<Number>::from(value).unwrap().value_of(), 3.0);
    assert!(Nullable::<Number>::from(JsNullable::new()).is_null());
}

#[wasm_bindgen_test]
fn api() {
    assert_eq!(Nullable::<u32>::default(), Nullable::Null);
    assert_eq!(Nullable::from(3), Nullable::Value(3));
    assert_eq!(Nullable::from(Some(3)), Nullable::Value(3));
    assert_eq!(Nullable::<u32>::from(None), Nullable::Null);
    assert_eq!(Option::from(Nullable::Value(3)), Some(3));
    assert_eq!(Option::<u32>::from(Nullable::Undefined), None);
    assert_eq!(Nullable::Value(3).value(), Some(3));
    assert_eq!(Nullable::<u32>::Null.into_option(), None);

    let mut v = Nullable::Value(3);
    assert!(v.is_value() && !v.is_empty());
    assert!(Nullable::<u32>::Null.is_empty() && Nullable::<u32>::Undefined.is_empty());
    assert_eq!(v.as_ref(), Nullable::Value(&3));
    *v.as_mut().unwrap() = 4;
    assert_eq!(v.map(|n| n * 2), Nullable::Value(8));
    assert_eq!(
        Nullable::<u32>::Undefined.map(|n| n * 2),
        Nullable::Undefined
    );
    assert_eq!(Nullable::Value(1).unwrap_or(2), 1);
    assert_eq!(Nullable::<u32>::Null.unwrap_or(2), 2);
    assert_eq!(Nullable::<u32>::Undefined.unwrap_or_else(|| 3), 3);
    assert_eq!(Nullable::<u32>::Null.unwrap_or_default(), 0);
    assert_eq!(Nullable::Value(1).expect("a value"), 1);
}
