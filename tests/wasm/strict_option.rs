//! JS → Rust `Option<T>` with and without `wasm_bindgen_unstable_strict_option`.
//! Under the cfg only `undefined` is `None` and `null` goes to `T`'s own
//! conversion; without it `null` is `None` too. The same module runs both
//! ways, with `strict_option_enabled` telling the JS side which to expect.

use std::cell::Cell;
use std::ptr::NonNull;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/strict_option.js")]
extern "C" {
    pub type StrictJsType;

    fn strict_option_js_exports();
    fn strict_option_js_debug() -> bool;
    fn strict_option_closure_driver(f: &Closure<dyn FnMut(Option<u32>, Option<String>) -> String>);

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_u32() -> Option<u32>;
    #[wasm_bindgen(js_name = strict_option_js_value)]
    fn js_value_u32() -> Option<u32>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_u32() -> Result<Option<u32>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_f64() -> Option<f64>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_f64() -> Result<Option<f64>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_bool() -> Option<bool>;
    #[wasm_bindgen(js_name = strict_option_js_true)]
    fn js_true_bool() -> Option<bool>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_bool() -> Result<Option<bool>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_char() -> Option<char>;
    #[wasm_bindgen(js_name = strict_option_js_string)]
    fn js_string_char() -> Option<char>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_char() -> Result<Option<char>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_string() -> Option<String>;
    #[wasm_bindgen(js_name = strict_option_js_string)]
    fn js_string_string() -> Option<String>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_string() -> Result<Option<String>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_vec() -> Option<Vec<u8>>;
    #[wasm_bindgen(js_name = strict_option_js_bytes)]
    fn js_bytes_vec() -> Option<Vec<u8>>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_vec() -> Result<Option<Vec<u8>>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_struct() -> Option<StrictThing>;
    #[wasm_bindgen(js_name = strict_option_js_thing)]
    fn js_thing_struct() -> Option<StrictThing>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_struct() -> Result<Option<StrictThing>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_enum() -> Option<StrictColor>;
    #[wasm_bindgen(js_name = strict_option_js_value)]
    fn js_value_enum() -> Option<StrictColor>;
    #[wasm_bindgen(js_name = strict_option_js_null, catch)]
    fn js_null_enum() -> Result<Option<StrictColor>, JsValue>;

    #[wasm_bindgen(js_name = strict_option_js_undefined)]
    fn js_undefined_jsvalue() -> Option<JsValue>;
    #[wasm_bindgen(js_name = strict_option_js_null)]
    fn js_null_jsvalue() -> Option<JsValue>;
    #[wasm_bindgen(js_name = strict_option_js_value)]
    fn js_value_jsvalue() -> Option<JsValue>;
    #[wasm_bindgen(js_name = strict_option_js_null)]
    fn js_null_js_type() -> Option<StrictJsType>;
}

const STRICT: bool = cfg!(wasm_bindgen_unstable_strict_option);

#[wasm_bindgen]
pub fn strict_option_enabled() -> bool {
    STRICT
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StrictColor {
    Red = 0,
    Green = 1,
    Blue = 7,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StrictName {
    Alpha = "alpha",
    Beta = "beta",
}

#[wasm_bindgen]
pub struct StrictThing(pub u32);

#[wasm_bindgen]
impl StrictThing {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u32) -> StrictThing {
        StrictThing(n)
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct StrictFields {
    pub n: Option<u32>,
    #[wasm_bindgen(getter_with_clone)]
    pub s: Option<String>,
}

#[wasm_bindgen]
impl StrictFields {
    #[wasm_bindgen(constructor)]
    pub fn new() -> StrictFields {
        StrictFields::default()
    }
}

#[wasm_bindgen]
pub fn strict_option_i32(a: Option<i32>) -> Option<i32> {
    a
}

#[wasm_bindgen]
pub fn strict_option_u8(a: Option<u8>) -> Option<u8> {
    a
}

#[wasm_bindgen]
pub fn strict_option_usize(a: Option<usize>) -> Option<usize> {
    a
}

#[wasm_bindgen]
pub fn strict_option_f32(a: Option<f32>) -> Option<f32> {
    a
}

#[wasm_bindgen]
pub fn strict_option_f64(a: Option<f64>) -> Option<f64> {
    a
}

#[wasm_bindgen]
pub fn strict_option_i64(a: Option<i64>) -> Option<i64> {
    a
}

#[wasm_bindgen]
pub fn strict_option_u128(a: Option<u128>) -> Option<u128> {
    a
}

#[wasm_bindgen]
pub fn strict_option_nonnull(a: Option<NonNull<u32>>) -> bool {
    a.is_some()
}

#[wasm_bindgen]
pub fn strict_option_bool(a: Option<bool>) -> Option<bool> {
    a
}

#[wasm_bindgen]
pub fn strict_option_char(a: Option<char>) -> Option<char> {
    a
}

#[wasm_bindgen]
pub fn strict_option_string(a: Option<String>) -> Option<String> {
    a
}

#[wasm_bindgen]
pub fn strict_option_vec_u8(a: Option<Vec<u8>>) -> Option<usize> {
    a.map(|v| v.len())
}

#[wasm_bindgen]
pub fn strict_option_vec_bool(a: Option<Vec<bool>>) -> Option<usize> {
    a.map(|v| v.len())
}

#[wasm_bindgen]
pub fn strict_option_vec_jsvalue(a: Option<Vec<JsValue>>) -> Option<usize> {
    a.map(|v| v.len())
}

#[wasm_bindgen]
pub fn strict_option_enum(a: Option<StrictColor>) -> Option<StrictColor> {
    a
}

#[wasm_bindgen]
pub fn strict_option_string_enum(a: Option<StrictName>) -> String {
    match a {
        None => "none",
        Some(StrictName::Alpha) => "alpha",
        Some(StrictName::Beta) => "beta",
        Some(_) => "invalid",
    }
    .into()
}

#[wasm_bindgen]
pub fn strict_option_struct(a: Option<StrictThing>) -> Option<u32> {
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
pub fn strict_option_jsvalue(a: Option<JsValue>) -> String {
    describe_js(a.as_ref())
}

#[wasm_bindgen]
pub fn strict_option_js_type(a: Option<StrictJsType>) -> String {
    describe_js(a.as_deref())
}

#[wasm_bindgen_test]
fn exports() {
    strict_option_js_exports();
}

#[wasm_bindgen_test]
fn closure_args() {
    let calls = Rc::new(Cell::new(0));
    let f = {
        let calls = calls.clone();
        Closure::new(move |n: Option<u32>, s: Option<String>| {
            calls.set(calls.get() + 1);
            format!("{n:?} {s:?}")
        })
    };
    strict_option_closure_driver(&f);
    assert!(calls.get() >= 2);
}

/// What `null` does for a release-mode numeric `null`: the throw is the
/// debug-assertion one, otherwise JS coerces it like the `T` conversion does.
fn assert_null_number<T: PartialEq + std::fmt::Debug>(r: Result<Option<T>, JsValue>, coerced: T) {
    match (STRICT, strict_option_js_debug()) {
        (false, _) => assert_eq!(r.unwrap(), None),
        (true, true) => assert!(r.is_err()),
        (true, false) => assert_eq!(r.unwrap(), Some(coerced)),
    }
}

fn assert_null_throws<T: PartialEq + std::fmt::Debug>(r: Result<Option<T>, JsValue>) {
    if STRICT {
        assert!(r.is_err());
    } else {
        assert_eq!(r.unwrap(), None);
    }
}

#[wasm_bindgen_test]
fn import_returns() {
    assert_eq!(js_undefined_u32(), None);
    assert_eq!(js_value_u32(), Some(7));
    assert_null_number(js_null_u32(), 0);

    assert_eq!(js_undefined_f64(), None);
    assert_null_number(js_null_f64(), 0.0);

    assert_eq!(js_undefined_bool(), None);
    assert_eq!(js_true_bool(), Some(true));
    assert_null_number(js_null_bool(), false);

    assert_eq!(js_undefined_enum(), None);
    assert_eq!(js_value_enum(), Some(StrictColor::Blue));
    assert_null_number(js_null_enum(), StrictColor::Red);

    assert_eq!(js_undefined_char(), None);
    assert_eq!(js_string_char(), Some('h'));
    assert_null_throws(js_null_char());

    assert_eq!(js_undefined_string(), None);
    assert_eq!(js_string_string().as_deref(), Some("hi"));
    assert_null_throws(js_null_string());

    assert_eq!(js_undefined_vec(), None);
    assert_eq!(js_bytes_vec(), Some(vec![1, 2, 3]));
    assert_null_throws(js_null_vec());

    assert!(js_undefined_struct().is_none());
    assert_eq!(js_thing_struct().map(|t| t.0), Some(5));
    assert_null_throws(js_null_struct().map(|t| t.map(|t| t.0)));
}

#[wasm_bindgen_test]
fn js_types() {
    assert!(js_undefined_jsvalue().is_none());
    assert_eq!(js_value_jsvalue().unwrap().as_f64(), Some(7.0));
    if cfg!(wasm_bindgen_unstable_strict_option) {
        assert!(js_null_jsvalue().unwrap().is_null());
        assert!(js_null_js_type().unwrap().is_null());
    } else {
        assert!(js_null_jsvalue().is_none());
        assert!(js_null_js_type().is_none());
    }
}
