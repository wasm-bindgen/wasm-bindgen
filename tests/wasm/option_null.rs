//! JS → Rust `Option<T>` reads both JS empties as `None`: `null` as well as
//! `undefined`, for every `T` and in every position. Rust → JS `None` is
//! always `undefined`; a slot that has to carry `null` out, or tell the two
//! apart coming in, is `Nullable<T>` / `Option<Nullable<T>>`.

use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::ptr::NonNull;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/option_null.js")]
extern "C" {
    pub type NullJsType;

    fn option_null_js_exports();
    fn option_null_closure_driver(f: &Closure<dyn FnMut(Option<u32>, Option<String>) -> String>);

    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_u32() -> Option<u32>;
    #[wasm_bindgen(js_name = option_null_js_value)]
    fn js_value_u32() -> Option<u32>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_f64() -> Option<f64>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_i64() -> Option<i64>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_bool() -> Option<bool>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_char() -> Option<char>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_string() -> Option<String>;
    #[wasm_bindgen(js_name = option_null_js_string)]
    fn js_string_string() -> Option<String>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_vec() -> Option<Vec<u8>>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_struct() -> Option<NullThing>;
    #[wasm_bindgen(js_name = option_null_js_thing)]
    fn js_thing_struct() -> Option<NullThing>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_enum() -> Option<NullColor>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_string_enum() -> Option<NullName>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_jsvalue() -> Option<JsValue>;
    #[wasm_bindgen(js_name = option_null_js_value)]
    fn js_value_jsvalue() -> Option<JsValue>;
    #[wasm_bindgen(js_name = option_null_js_null)]
    fn js_null_js_type() -> Option<NullJsType>;
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NullColor {
    Red = 0,
    Green = 1,
    Blue = 7,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NullName {
    Alpha = "alpha",
    Beta = "beta",
}

#[wasm_bindgen]
pub struct NullThing(pub u32);

#[wasm_bindgen]
impl NullThing {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u32) -> NullThing {
        NullThing(n)
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct NullFields {
    pub n: Option<u32>,
    #[wasm_bindgen(getter_with_clone)]
    pub s: Option<String>,
}

#[wasm_bindgen]
impl NullFields {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NullFields {
        NullFields::default()
    }
}

#[wasm_bindgen]
pub fn option_null_i32(a: Option<i32>) -> Option<i32> {
    a
}

#[wasm_bindgen]
pub fn option_null_u8(a: Option<u8>) -> Option<u8> {
    a
}

#[wasm_bindgen]
pub fn option_null_f64(a: Option<f64>) -> Option<f64> {
    a
}

#[wasm_bindgen]
pub fn option_null_i64(a: Option<i64>) -> Option<i64> {
    a
}

#[wasm_bindgen]
pub fn option_null_u128(a: Option<u128>) -> Option<u128> {
    a
}

#[wasm_bindgen]
pub fn option_null_nonnull(a: Option<NonNull<u32>>) -> bool {
    a.is_some()
}

#[wasm_bindgen]
pub fn option_null_bool(a: Option<bool>) -> Option<bool> {
    a
}

#[wasm_bindgen]
pub fn option_null_char(a: Option<char>) -> Option<char> {
    a
}

#[wasm_bindgen]
pub fn option_null_string(a: Option<String>) -> Option<String> {
    a
}

#[wasm_bindgen]
pub fn option_null_vec_u8(a: Option<Vec<u8>>) -> Option<usize> {
    a.map(|v| v.len())
}

#[wasm_bindgen]
pub fn option_null_vec_jsvalue(a: Option<Vec<JsValue>>) -> Option<usize> {
    a.map(|v| v.len())
}

#[wasm_bindgen]
pub fn option_null_enum(a: Option<NullColor>) -> Option<NullColor> {
    a
}

#[wasm_bindgen]
pub fn option_null_string_enum(a: Option<NullName>) -> String {
    match a {
        None => "none",
        Some(NullName::Alpha) => "alpha",
        Some(NullName::Beta) => "beta",
        Some(_) => "invalid",
    }
    .into()
}

#[wasm_bindgen]
pub fn option_null_struct(a: Option<NullThing>) -> Option<u32> {
    a.map(|t| t.0)
}

fn describe_js(a: Option<&JsValue>) -> String {
    match a {
        None => "none".into(),
        Some(v) if v.is_null() => "null".into(),
        Some(_) => "some".into(),
    }
}

#[wasm_bindgen]
pub fn option_null_jsvalue(a: Option<JsValue>) -> String {
    describe_js(a.as_ref())
}

#[wasm_bindgen]
pub fn option_null_js_type(a: Option<NullJsType>) -> String {
    describe_js(a.as_deref())
}

#[wasm_bindgen_test]
fn exports() {
    option_null_js_exports();
}

#[wasm_bindgen_test]
fn closure_args() {
    let calls = Rc::new(Cell::new(0));
    let f = {
        let calls = AssertUnwindSafe(calls.clone());
        Closure::new(move |n: Option<u32>, s: Option<String>| {
            calls.set(calls.get() + 1);
            format!("{n:?} {s:?}")
        })
    };
    option_null_closure_driver(&f);
    assert_eq!(calls.get(), 4);
}

#[wasm_bindgen_test]
fn import_returns() {
    assert_eq!(js_value_u32(), Some(7));
    assert_eq!(js_null_u32(), None);
    assert_eq!(js_null_f64(), None);
    assert_eq!(js_null_i64(), None);
    assert_eq!(js_null_bool(), None);
    assert_eq!(js_null_char(), None);
    assert_eq!(js_string_string().as_deref(), Some("hi"));
    assert_eq!(js_null_string(), None);
    assert_eq!(js_null_vec(), None);
    assert_eq!(js_thing_struct().map(|t| t.0), Some(5));
    assert!(js_null_struct().is_none());
    assert_eq!(js_null_enum(), None);
    assert_eq!(js_null_string_enum(), None);
    assert_eq!(js_value_jsvalue().unwrap().as_f64(), Some(7.0));
    assert!(js_null_jsvalue().is_none());
    assert!(js_null_js_type().is_none());
}
