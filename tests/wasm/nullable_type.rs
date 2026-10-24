//! `Nullable<T>` is `T | null` and `Option<Nullable<T>>` is
//! `T | null | undefined`: every state crosses the boundary intact in both
//! directions, for every `T` category and in every position. `undefined` into
//! a plain `Nullable<T>` goes to `T`'s own conversion, as `null` does into a
//! strict `Option<T>`. The JS side checks with `=== null` and `=== undefined`.

use std::cell::Cell;
use std::fmt::Debug;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use js_sys::{JsNullable, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/nullable_type.js")]
extern "C" {
    pub type NullableJsType;

    fn nullable_type_js_exports();
    fn nullable_type_js_debug() -> bool;
    fn nullable_type_closure_driver(
        f: &Closure<dyn FnMut(Nullable<u32>, Option<Nullable<String>>) -> String>,
    );
    fn nullable_type_nullable_closure_driver(
        f: &Closure<dyn FnMut(Nullable<String>) -> Nullable<String>>,
    );
    fn nullable_type_optional_closure_driver(
        f: &Closure<dyn FnMut(Option<Nullable<u32>>) -> Option<Nullable<u32>>>,
    );

    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_str_ref(v: Nullable<&str>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_opt_str_ref(v: Option<Nullable<&str>>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_slice_ref(v: Nullable<&[u8]>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_opt_slice_ref(v: Option<Nullable<&[u8]>>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_string_slice_ref(v: Nullable<&[String]>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_opt_string_slice_ref(v: Option<Nullable<&[String]>>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_jsvalue_ref(v: Nullable<&JsValue>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_opt_jsvalue_ref(v: Option<Nullable<&JsValue>>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_closure_ref(v: Nullable<&Closure<dyn Fn()>>) -> String;
    #[wasm_bindgen(js_name = nullable_type_describe)]
    fn describe_opt_closure_ref(v: Option<Nullable<&Closure<dyn Fn()>>>) -> String;
}

/// The imports for one `T`: `describe` reports which state JS received, and
/// the `js_*` functions return each state, `undefined` into a plain
/// `Nullable<T>` with `catch` since the glue may throw for it.
macro_rules! imports {
    ($($module:ident: $ty:ty => $value:ident),* $(,)?) => {$(
        mod $module {
            use super::*;

            #[wasm_bindgen(module = "tests/wasm/nullable_type.js")]
            extern "C" {
                #[wasm_bindgen(js_name = nullable_type_describe)]
                pub fn describe(v: Nullable<$ty>) -> String;
                #[wasm_bindgen(js_name = nullable_type_describe)]
                pub fn describe_opt(v: Option<Nullable<$ty>>) -> String;
                #[wasm_bindgen(js_name = nullable_type_js_null)]
                pub fn js_null() -> Nullable<$ty>;
                #[wasm_bindgen(js_name = nullable_type_js_null)]
                pub fn js_null_opt() -> Option<Nullable<$ty>>;
                #[wasm_bindgen(js_name = nullable_type_js_undefined)]
                pub fn js_undefined_opt() -> Option<Nullable<$ty>>;
                #[wasm_bindgen(js_name = nullable_type_js_undefined, catch)]
                pub fn js_undefined() -> Result<Nullable<$ty>, JsValue>;
                #[wasm_bindgen(js_name = $value)]
                pub fn js_value() -> Nullable<$ty>;
                #[wasm_bindgen(js_name = $value)]
                pub fn js_value_opt() -> Option<Nullable<$ty>>;
            }
        }
    )*};
}

imports! {
    n_u8: u8 => nullable_type_js_number,
    n_i32: i32 => nullable_type_js_number,
    n_u32: u32 => nullable_type_js_number,
    n_usize: usize => nullable_type_js_number,
    n_f32: f32 => nullable_type_js_float,
    n_f64: f64 => nullable_type_js_float,
    n_i64: i64 => nullable_type_js_bigint,
    n_u128: u128 => nullable_type_js_bigint,
    n_ptr: *const u8 => nullable_type_js_number,
    n_bool: bool => nullable_type_js_true,
    n_char: char => nullable_type_js_string,
    n_string: String => nullable_type_js_string,
    n_vec_u8: Vec<u8> => nullable_type_js_bytes,
    n_vec_bool: Vec<bool> => nullable_type_js_bools,
    n_vec_jsvalue: Vec<JsValue> => nullable_type_js_values,
    n_enum: NullableColor => nullable_type_js_number,
    n_string_enum: NullableName => nullable_type_js_beta,
    n_struct: NullableThing => nullable_type_js_thing,
    n_jsvalue: JsValue => nullable_type_js_object,
    n_js_type: NullableJsType => nullable_type_js_object,
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
    pub o: Option<Nullable<u32>>,
    #[wasm_bindgen(getter_with_clone)]
    pub s: Nullable<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub t: Option<Nullable<String>>,
}

#[wasm_bindgen]
impl NullableFields {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NullableFields {
        NullableFields::default()
    }
}

/// Exported identity functions per `T`, covering the export argument and
/// return positions for `Nullable<T>` and `Option<Nullable<T>>`.
macro_rules! exports {
    ($($name:ident / $opt_name:ident: $ty:ty),* $(,)?) => {$(
        #[wasm_bindgen]
        pub fn $name(a: Nullable<$ty>) -> Nullable<$ty> {
            a
        }

        #[wasm_bindgen]
        pub fn $opt_name(a: Option<Nullable<$ty>>) -> Option<Nullable<$ty>> {
            a
        }
    )*};
}

exports! {
    nullable_type_u8 / nullable_type_opt_u8: u8,
    nullable_type_i32 / nullable_type_opt_i32: i32,
    nullable_type_u32 / nullable_type_opt_u32: u32,
    nullable_type_usize / nullable_type_opt_usize: usize,
    nullable_type_f32 / nullable_type_opt_f32: f32,
    nullable_type_f64 / nullable_type_opt_f64: f64,
    nullable_type_i64 / nullable_type_opt_i64: i64,
    nullable_type_u128 / nullable_type_opt_u128: u128,
    nullable_type_ptr / nullable_type_opt_ptr: *const u8,
    nullable_type_bool / nullable_type_opt_bool: bool,
    nullable_type_char / nullable_type_opt_char: char,
    nullable_type_string / nullable_type_opt_string: String,
    nullable_type_vec_u8 / nullable_type_opt_vec_u8: Vec<u8>,
    nullable_type_vec_bool / nullable_type_opt_vec_bool: Vec<bool>,
    nullable_type_vec_jsvalue / nullable_type_opt_vec_jsvalue: Vec<JsValue>,
    nullable_type_enum / nullable_type_opt_enum: NullableColor,
    nullable_type_string_enum / nullable_type_opt_string_enum: NullableName,
    nullable_type_struct / nullable_type_opt_struct: NullableThing,
    nullable_type_jsvalue / nullable_type_opt_jsvalue: JsValue,
    nullable_type_js_type / nullable_type_opt_js_type: NullableJsType,
}

fn fallible<T>(a: T, fail: bool) -> Result<T, JsValue> {
    if fail {
        Err(JsValue::from_str("failed"))
    } else {
        Ok(a)
    }
}

macro_rules! result_exports {
    ($($name:ident / $opt_name:ident: $ty:ty),* $(,)?) => {$(
        #[wasm_bindgen]
        pub fn $name(a: Nullable<$ty>, fail: bool) -> Result<Nullable<$ty>, JsValue> {
            fallible(a, fail)
        }

        #[wasm_bindgen]
        pub fn $opt_name(
            a: Option<Nullable<$ty>>,
            fail: bool,
        ) -> Result<Option<Nullable<$ty>>, JsValue> {
            fallible(a, fail)
        }
    )*};
}

result_exports! {
    nullable_type_result_u32 / nullable_type_result_opt_u32: u32,
    nullable_type_result_i64 / nullable_type_result_opt_i64: i64,
    nullable_type_result_string / nullable_type_result_opt_string: String,
    nullable_type_result_struct / nullable_type_result_opt_struct: NullableThing,
    nullable_type_result_jsvalue / nullable_type_result_opt_jsvalue: JsValue,
}

#[wasm_bindgen]
pub fn nullable_type_string_enum_name(a: Nullable<NullableName>) -> String {
    match a {
        Nullable::Null => "null",
        Nullable::Value(NullableName::Alpha) => "alpha",
        Nullable::Value(NullableName::Beta) => "beta",
        Nullable::Value(_) => "invalid",
    }
    .into()
}

#[wasm_bindgen]
pub fn nullable_type_opt_string_enum_name(a: Option<Nullable<NullableName>>) -> String {
    match a {
        None => "undefined".into(),
        Some(a) => nullable_type_string_enum_name(a),
    }
}

#[wasm_bindgen_test]
fn exports() {
    nullable_type_js_exports();
}

#[wasm_bindgen_test]
fn closure_args_and_returns() {
    let calls = Rc::new(Cell::new(0));
    let f = {
        let calls = AssertUnwindSafe(calls.clone());
        Closure::new(move |n: Nullable<u32>, s: Option<Nullable<String>>| {
            calls.set(calls.get() + 1);
            format!("{n:?} {s:?}")
        })
    };
    nullable_type_closure_driver(&f);
    assert!(calls.get() >= 3);
    let g = Closure::new(|s: Nullable<String>| s);
    nullable_type_nullable_closure_driver(&g);
    let h = Closure::new(|n: Option<Nullable<u32>>| n);
    nullable_type_optional_closure_driver(&h);
}

/// Each state crosses an import argument and return as itself.
macro_rules! states {
    ($m:ident, $value:expr) => {{
        assert_eq!($m::describe(Nullable::Null), "null");
        assert_eq!($m::describe(Nullable::Value($value)), "value");
        assert_eq!($m::describe_opt(None), "undefined");
        assert_eq!($m::describe_opt(Some(Nullable::Null)), "null");
        assert_eq!($m::describe_opt(Some(Nullable::Value($value))), "value");
        assert!($m::js_null().is_null());
        assert!($m::js_null_opt().is_some_and(|v| v.is_null()));
        assert!($m::js_undefined_opt().is_none());
        assert!($m::js_value().is_value());
        assert!($m::js_value_opt().is_some_and(|v| v.is_value()));
    }};
}

/// What `undefined` does for a plain `Nullable<T>` import return the glue
/// coerces: debug glue throws its type assertion, release glue hands Rust
/// `coerced`.
fn undefined_coerces<T: PartialEq + Debug>(r: Result<Nullable<T>, JsValue>, coerced: T) {
    if nullable_type_js_debug() {
        assert!(r.is_err());
    } else {
        assert_eq!(r.unwrap(), Nullable::Value(coerced));
    }
}

fn undefined_coerces_to_nan<T: Copy + Debug>(
    r: Result<Nullable<T>, JsValue>,
    is_nan: fn(T) -> bool,
) {
    if nullable_type_js_debug() {
        assert!(r.is_err());
    } else {
        assert!(is_nan(r.unwrap().unwrap()));
    }
}

fn undefined_throws<T: Debug>(r: Result<Nullable<T>, JsValue>) {
    assert!(r.is_err());
}

#[wasm_bindgen_test]
fn import_args_and_returns() {
    states!(n_u8, 3);
    assert_eq!(n_u8::js_value(), Nullable::Value(7));
    undefined_coerces(n_u8::js_undefined(), 0);
    states!(n_i32, -3);
    assert_eq!(n_i32::js_value(), Nullable::Value(7));
    undefined_coerces(n_i32::js_undefined(), 0);
    states!(n_u32, 3);
    assert_eq!(n_u32::js_value(), Nullable::Value(7));
    undefined_coerces(n_u32::js_undefined(), 0);
    states!(n_usize, 3);
    assert_eq!(n_usize::js_value(), Nullable::Value(7));
    undefined_coerces(n_usize::js_undefined(), 0);
    states!(n_f32, 1.5);
    assert_eq!(n_f32::js_value(), Nullable::Value(2.5));
    undefined_coerces_to_nan(n_f32::js_undefined(), f32::is_nan);
    states!(n_f64, 1.5);
    assert_eq!(n_f64::js_value(), Nullable::Value(2.5));
    undefined_coerces_to_nan(n_f64::js_undefined(), f64::is_nan);
    states!(n_i64, -3);
    assert_eq!(n_i64::js_value(), Nullable::Value(7));
    undefined_throws(n_i64::js_undefined());
    states!(n_u128, 3);
    assert_eq!(n_u128::js_value(), Nullable::Value(7));
    undefined_throws(n_u128::js_undefined());

    states!(n_ptr, 8 as *const u8);
    assert_eq!(n_ptr::js_value().unwrap(), 7 as *const u8);
    undefined_coerces(n_ptr::js_undefined(), std::ptr::null());

    states!(n_bool, false);
    assert_eq!(n_bool::js_value(), Nullable::Value(true));
    undefined_coerces(n_bool::js_undefined(), false);
    states!(n_char, 'x');
    assert_eq!(n_char::js_value(), Nullable::Value('h'));
    undefined_throws(n_char::js_undefined());
    states!(n_string, "hi".to_string());
    assert_eq!(n_string::js_value().unwrap(), "hi");
    undefined_throws(n_string::js_undefined());

    states!(n_vec_u8, vec![1, 2]);
    assert_eq!(n_vec_u8::js_value().unwrap(), vec![1, 2, 3]);
    undefined_throws(n_vec_u8::js_undefined());
    states!(n_vec_bool, vec![true]);
    assert_eq!(n_vec_bool::js_value().unwrap(), vec![true, false]);
    undefined_throws(n_vec_bool::js_undefined());
    states!(n_vec_jsvalue, vec![JsValue::from(1)]);
    assert_eq!(n_vec_jsvalue::js_value().unwrap().len(), 2);
    undefined_throws(n_vec_jsvalue::js_undefined());

    states!(n_enum, NullableColor::Green);
    assert_eq!(n_enum::js_value(), Nullable::Value(NullableColor::Blue));
    // Release glue hands Rust an invalid discriminant, which throws from
    // inside Rust rather than from JS, so only the debug assertion is caught.
    if nullable_type_js_debug() {
        assert!(n_enum::js_undefined().is_err());
    }
    states!(n_string_enum, NullableName::Alpha);
    assert_eq!(
        n_string_enum::js_value(),
        Nullable::Value(NullableName::Beta)
    );
    // An unknown string is the invalid variant.
    assert!(matches!(
        n_string_enum::js_undefined(),
        Ok(Nullable::Value(v)) if v != NullableName::Alpha && v != NullableName::Beta
    ));

    states!(n_struct, NullableThing(4));
    assert_eq!(n_struct::js_value().map(|t| t.0), Nullable::Value(5));
    undefined_throws(n_struct::js_undefined().map(|t| t.map(|t| t.0)));

    states!(n_jsvalue, JsValue::from(1));
    assert!(n_jsvalue::js_value().unwrap().is_object());
    // `undefined` is a JS value like any other for `Nullable<JsValue>`, and
    // `null` sent as a value is still `null`.
    assert!(n_jsvalue::js_undefined().unwrap().unwrap().is_undefined());
    assert_eq!(n_jsvalue::describe(Nullable::Value(JsValue::NULL)), "null");
    assert_eq!(
        n_jsvalue::describe(Nullable::Value(JsValue::UNDEFINED)),
        "undefined"
    );
    states!(n_js_type, JsValue::from(1).unchecked_into());
    assert!(n_js_type::js_value().is_value());
    assert!(n_js_type::js_undefined().unwrap().unwrap().is_undefined());
}

#[wasm_bindgen_test]
fn import_ref_args() {
    assert_eq!(describe_str_ref(Nullable::Null), "null");
    assert_eq!(describe_str_ref(Nullable::Value("hi")), "value");
    assert_eq!(describe_opt_str_ref(None), "undefined");
    assert_eq!(describe_opt_str_ref(Some(Nullable::Null)), "null");
    assert_eq!(describe_opt_str_ref(Some(Nullable::Value("hi"))), "value");

    assert_eq!(describe_slice_ref(Nullable::Null), "null");
    assert_eq!(describe_slice_ref(Nullable::Value(&[1, 2])), "value");
    assert_eq!(describe_opt_slice_ref(None), "undefined");
    assert_eq!(describe_opt_slice_ref(Some(Nullable::Null)), "null");
    assert_eq!(
        describe_opt_slice_ref(Some(Nullable::Value(&[1, 2]))),
        "value"
    );

    let strings = ["a".to_string()];
    assert_eq!(describe_string_slice_ref(Nullable::Null), "null");
    assert_eq!(
        describe_string_slice_ref(Nullable::Value(&strings)),
        "value"
    );
    assert_eq!(describe_opt_string_slice_ref(None), "undefined");
    assert_eq!(describe_opt_string_slice_ref(Some(Nullable::Null)), "null");
    assert_eq!(
        describe_opt_string_slice_ref(Some(Nullable::Value(&strings))),
        "value"
    );

    let one = JsValue::from(1);
    assert_eq!(describe_jsvalue_ref(Nullable::Null), "null");
    assert_eq!(describe_jsvalue_ref(Nullable::Value(&one)), "value");
    assert_eq!(describe_opt_jsvalue_ref(None), "undefined");
    assert_eq!(describe_opt_jsvalue_ref(Some(Nullable::Null)), "null");
    assert_eq!(
        describe_opt_jsvalue_ref(Some(Nullable::Value(&one))),
        "value"
    );

    let f = Closure::new(|| ());
    assert_eq!(describe_closure_ref(Nullable::Null), "null");
    assert_eq!(describe_closure_ref(Nullable::Value(&f)), "value");
    assert_eq!(describe_opt_closure_ref(None), "undefined");
    assert_eq!(describe_opt_closure_ref(Some(Nullable::Null)), "null");
    assert_eq!(describe_opt_closure_ref(Some(Nullable::Value(&f))), "value");
}

#[wasm_bindgen_test]
fn js_nullable_conversions() {
    let null: JsNullable<Number> = Nullable::<Number>::Null.into();
    assert!(JsValue::is_null(&null));
    let value: JsNullable<Number> = Nullable::Value(Number::from(3)).into();
    assert_eq!(value.as_option().unwrap().value_of(), 3.0);
    let undefined: JsNullable<Number> = None::<Nullable<Number>>.into();
    assert!(JsValue::is_undefined(&undefined));
    let some_null: JsNullable<Number> = Some(Nullable::<Number>::Null).into();
    assert!(JsValue::is_null(&some_null));

    assert!(matches!(
        Option::<Nullable<Number>>::from(null),
        Some(Nullable::Null)
    ));
    assert!(Option::<Nullable<Number>>::from(undefined).is_none());
    match Option::<Nullable<Number>>::from(value) {
        Some(Nullable::Value(n)) => assert_eq!(n.value_of(), 3.0),
        other => panic!("{other:?}"),
    }
    assert!(matches!(
        Option::<Nullable<Number>>::from(JsNullable::<Number>::new()),
        Some(Nullable::Null)
    ));
}

#[wasm_bindgen_test]
fn api() {
    assert_eq!(Nullable::<u32>::default(), Nullable::Null);
    assert_eq!(Nullable::from(3), Nullable::Value(3));
    assert_eq!(Nullable::from(Some(3)), Nullable::Value(3));
    assert_eq!(Nullable::<u32>::from(None), Nullable::Null);
    assert_eq!(Option::from(Nullable::Value(3)), Some(3));
    assert_eq!(Option::<u32>::from(Nullable::Null), None);
    assert_eq!(Nullable::Value(3).value(), Some(3));
    assert_eq!(Nullable::<u32>::Null.into_option(), None);
    // The union collapses to `Option<T>`.
    assert_eq!(Some(Nullable::Value(3)).and_then(Nullable::value), Some(3));
    assert_eq!(Some(Nullable::<u32>::Null).and_then(Nullable::value), None);
    assert_eq!(None::<Nullable<u32>>.and_then(Nullable::value), None);

    let mut v = Nullable::Value(3);
    assert!(v.is_value() && !v.is_null());
    assert!(Nullable::<u32>::Null.is_null());
    assert_eq!(v.as_ref(), Nullable::Value(&3));
    *v.as_mut().unwrap() = 4;
    assert_eq!(v.map(|n| n * 2), Nullable::Value(8));
    assert_eq!(Nullable::<u32>::Null.map(|n| n * 2), Nullable::Null);
    assert_eq!(Nullable::Value(1).unwrap_or(2), 1);
    assert_eq!(Nullable::<u32>::Null.unwrap_or(2), 2);
    assert_eq!(Nullable::<u32>::Null.unwrap_or_else(|| 3), 3);
    assert_eq!(Nullable::<u32>::Null.unwrap_or_default(), 0);
    assert_eq!(Nullable::Value(1).expect("a value"), 1);

    assert!(Nullable::Value(3).is_value_and(|n| n > 2));
    assert!(!Nullable::Value(1).is_value_and(|n| n > 2));
    assert!(!Nullable::<u32>::Null.is_value_and(|n| n > 2));
    assert!(Nullable::<u32>::Null.is_null_or(|n| n > 2));
    assert!(Nullable::Value(3).is_null_or(|n| n > 2));
    assert!(!Nullable::Value(1).is_null_or(|n| n > 2));

    assert_eq!(Nullable::Value(3).ok_or("null"), Ok(3));
    assert_eq!(Nullable::<u32>::Null.ok_or("null"), Err("null"));
    assert_eq!(Nullable::<u32>::Null.ok_or_else(|| "null"), Err("null"));
    assert_eq!(
        Nullable::Value(3).and_then(|n| Nullable::Value(n + 1)),
        Nullable::Value(4)
    );
    assert_eq!(
        Nullable::Value(3).and_then(|_| Nullable::<u32>::Null),
        Nullable::Null
    );
    assert_eq!(
        Nullable::<u32>::Null.and_then(|n| Nullable::Value(n + 1)),
        Nullable::Null
    );
    assert_eq!(Nullable::Value(3).filter(|n| *n > 2), Nullable::Value(3));
    assert_eq!(Nullable::Value(1).filter(|n| *n > 2), Nullable::Null);

    let mut v = Nullable::Value(3);
    assert_eq!(v.take(), Nullable::Value(3));
    assert_eq!(v, Nullable::Null);
    assert_eq!(v.replace(5), Nullable::Null);
    assert_eq!(v.replace(6), Nullable::Value(5));
    for n in &mut v {
        *n += 1;
    }
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), vec![7]);
    assert_eq!(Nullable::<u32>::Null.iter().count(), 0);
    assert_eq!(v.into_iter().collect::<Vec<_>>(), vec![7]);
    assert_eq!(
        vec![Nullable::Value(1), Nullable::Null, Nullable::Value(2)]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec![1, 2]
    );

    let s = Nullable::Value(String::from("x"));
    assert_eq!(s.as_deref(), Nullable::Value("x"));
    assert_eq!(Nullable::<String>::Null.as_deref(), Nullable::Null);
    assert_eq!(s.as_ref().cloned(), s);
    assert_eq!(Nullable::Value(3).as_ref().copied(), Nullable::Value(3));
    let mut n = Nullable::Value(3);
    *n.as_mut().unwrap() = 4;
    assert_eq!(n.as_mut().copied(), Nullable::Value(4));
    let mut b = Nullable::Value(Box::new(3));
    *b.as_deref_mut().unwrap() = 4;
    assert_eq!(b.as_deref(), Nullable::Value(&4));
    assert_eq!(
        Nullable::Value(String::from("y"))
            .as_mut()
            .cloned()
            .as_deref(),
        Nullable::Value("y")
    );
}

// Also written without `?` so the module compiles on stable without the cfg.
#[cfg(wasm_bindgen_unstable_try_trait)]
#[wasm_bindgen_test]
fn try_trait() {
    fn nullable_from_nullable(n: Nullable<u32>) -> Nullable<u32> {
        Nullable::Value(n? + 1)
    }
    fn option_from_nullable(n: Nullable<u32>) -> Option<u32> {
        Some(n? + 1)
    }
    fn nullable_from_option(n: Option<u32>) -> Nullable<u32> {
        Nullable::Value(n? + 1)
    }
    assert_eq!(
        nullable_from_nullable(Nullable::Value(1)),
        Nullable::Value(2)
    );
    assert_eq!(nullable_from_nullable(Nullable::Null), Nullable::Null);
    assert_eq!(option_from_nullable(Nullable::Value(1)), Some(2));
    assert_eq!(option_from_nullable(Nullable::Null), None);
    assert_eq!(nullable_from_option(Some(1)), Nullable::Value(2));
    assert_eq!(nullable_from_option(None), Nullable::Null);
}
