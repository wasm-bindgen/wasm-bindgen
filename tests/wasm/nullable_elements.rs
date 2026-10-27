//! `Option<T>` and `Nullable<T>` as the element of a shape, rather than a
//! bare argument or return: a borrowed slice, an owned vector, a `Function`
//! signature, a `Promise` and a js-sys container's type arguments. Each is
//! covered as a plain import and under `experimental_generic_mono`, which is
//! where exact nullability carriers are wanted everywhere.

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use js_sys::{Array, Function, JsString, Map, Number, Promise, Undefined};
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, VectorRefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/nullable_elements.js")]
extern "C" {
    #[wasm_bindgen(js_name = describe)]
    fn opt_f64s(v: &[Option<f64>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn nullable_f64s(v: &[Nullable<f64>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_nullable_u8s(v: &[Option<Nullable<u8>>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_js_strings(v: &[Option<JsString>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn nullable_js_strings(v: &[Nullable<JsString>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_strings(v: &[Option<String>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn nullable_strings(v: &[Nullable<String>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn nullable_strs(v: &[Nullable<&str>]) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn opt_nullable_strings_slice(v: Option<&[Nullable<String>]>) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn mono_slice<T: VectorRefIntoWasmAbi>(v: &[T]) -> String;

    #[wasm_bindgen(js_name = describe)]
    fn vec_nullable_strings(v: Vec<Nullable<String>>) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn vec_opt_strings(v: Vec<Option<String>>) -> String;
    #[wasm_bindgen(js_name = echo)]
    fn echo_vec_nullable_strings(v: Vec<Nullable<String>>) -> Vec<Nullable<String>>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_vec_nullable_js_strings(v: Vec<Nullable<JsString>>) -> Vec<Nullable<JsString>>;
    #[wasm_bindgen(js_name = echo)]
    fn echo_vec_opt_f64s(v: Vec<Option<f64>>) -> Vec<Option<f64>>;
    #[wasm_bindgen(experimental_generic_mono, js_name = echo)]
    fn mono_vec<T>(v: Vec<T>) -> Vec<T>
    where
        Vec<T>: IntoWasmAbi + FromWasmAbi;

    #[wasm_bindgen(js_name = echo)]
    fn echo_opt_ref_nullable_string(v: Option<&Nullable<String>>) -> JsValue;
    #[wasm_bindgen(js_name = echo)]
    fn echo_ref_f64(v: &f64) -> f64;
    #[wasm_bindgen(js_name = echo)]
    fn echo_ref_opt_f64(v: &Option<f64>) -> JsValue;

    #[wasm_bindgen(js_name = callNullable)]
    fn call_nullable(f: &Function<fn(Nullable<JsString>) -> Undefined>);
    #[wasm_bindgen(js_name = echo)]
    fn echo_fn(
        f: &Function<fn(Nullable<JsString>) -> Option<Number>>,
    ) -> Function<fn(Nullable<JsString>) -> Option<Number>>;
    #[wasm_bindgen(experimental_generic_mono, js_name = callNullable)]
    fn mono_call_nullable<T: IntoWasmAbi>(
        f: &Function<fn(Nullable<JsString>) -> Undefined>,
        tag: T,
    );

    #[wasm_bindgen(js_name = echo)]
    async fn await_nullable(p: &Promise<Nullable<JsString>>) -> Nullable<JsString>;
    #[wasm_bindgen(experimental_generic_mono, js_name = echo)]
    async fn mono_await_nullable<T: IntoWasmAbi>(
        p: &Promise<Nullable<JsString>>,
        tag: T,
    ) -> Nullable<String>;

    #[wasm_bindgen(js_name = nullableArray)]
    fn nullable_array() -> Array<Nullable<JsString>>;
    #[wasm_bindgen(js_name = nullableMap)]
    fn nullable_map() -> Map<JsString, Nullable<JsString>>;
    #[wasm_bindgen(js_name = describe)]
    fn describe_array(v: &Array<Nullable<JsString>>) -> String;

    fn test_exports();
}

#[wasm_bindgen]
pub fn export_echo_vec(v: Vec<Nullable<String>>) -> Vec<Nullable<String>> {
    v
}

#[wasm_bindgen]
pub fn export_echo_opt_vec(v: Vec<Option<String>>) -> Vec<Option<String>> {
    v
}

#[wasm_bindgen]
pub fn export_echo_opt_js_vec(v: Vec<Option<JsString>>) -> Vec<Option<JsString>> {
    v
}

#[wasm_bindgen]
pub fn export_null_count(v: Vec<Nullable<JsString>>) -> u32 {
    v.iter().filter(|e| e.is_null()).count() as u32
}

#[wasm_bindgen_test]
fn slices() {
    assert_eq!(
        opt_f64s(&[Some(1.5), None, Some(-2.0)]),
        "Array:[1.5,\"undefined\",-2]"
    );
    assert_eq!(
        nullable_f64s(&[Nullable::Value(1.5), Nullable::Null]),
        "Array:[1.5,null]"
    );
    assert_eq!(
        opt_nullable_u8s(&[Some(Nullable::Value(7)), Some(Nullable::Null), None]),
        "Array:[7,null,\"undefined\"]"
    );
    assert_eq!(
        opt_js_strings(&[Some(JsString::from("a")), None]),
        "Array:[\"a\",\"undefined\"]"
    );
    assert_eq!(
        nullable_js_strings(&[Nullable::Value(JsString::from("a")), Nullable::Null]),
        "Array:[\"a\",null]"
    );
    assert_eq!(
        opt_strings(&[Some("a".to_string()), None]),
        "Array:[\"a\",\"undefined\"]"
    );
    assert_eq!(
        nullable_strings(&[Nullable::Value("a".to_string()), Nullable::Null]),
        "Array:[\"a\",null]"
    );
    assert_eq!(
        nullable_strs(&[Nullable::Value("a"), Nullable::Null]),
        "Array:[\"a\",null]"
    );
    assert_eq!(nullable_strings(&[]), "Array:[]");
    assert_eq!(opt_nullable_strings_slice(None), "undefined");
    assert_eq!(
        opt_nullable_strings_slice(Some(&[Nullable::Null])),
        "Array:[null]"
    );
}

#[wasm_bindgen_test]
fn mono_slices() {
    assert_eq!(mono_slice(&[Some(1.5), None]), "Array:[1.5,\"undefined\"]");
    assert_eq!(
        mono_slice(&[Nullable::Value(JsString::from("a")), Nullable::Null]),
        "Array:[\"a\",null]"
    );
    assert_eq!(
        mono_slice(&[Nullable::Value("a".to_string()), Nullable::Null]),
        "Array:[\"a\",null]"
    );
}

#[wasm_bindgen_test]
fn vecs() {
    let v = vec![Nullable::Value("a".to_string()), Nullable::Null];
    assert_eq!(vec_nullable_strings(v.clone()), "Array:[\"a\",null]");
    assert_eq!(echo_vec_nullable_strings(v.clone()), v);
    assert_eq!(
        vec_opt_strings(vec![Some("a".to_string()), None]),
        "Array:[\"a\",\"undefined\"]"
    );
    let js = vec![Nullable::Value(JsString::from("a")), Nullable::Null];
    assert_eq!(echo_vec_nullable_js_strings(js.clone()), js);
    assert_eq!(
        echo_vec_opt_f64s(vec![Some(1.5), None]),
        vec![Some(1.5), None]
    );
    assert_eq!(mono_vec(v.clone()), v);
    test_exports();
}

// A borrowed scalar crosses by copy, so `&T` and `Option<&T>` have `T`'s
// ABIs, and `Option<&Nullable<T>>` is `Option<Nullable<&T>>`'s.
#[wasm_bindgen_test]
fn borrowed() {
    assert!(echo_opt_ref_nullable_string(None).is_undefined());
    assert!(echo_opt_ref_nullable_string(Some(&Nullable::Null)).is_null());
    assert_eq!(
        echo_opt_ref_nullable_string(Some(&Nullable::Value("a".to_string()))),
        "a"
    );
    assert_eq!(echo_ref_f64(&1.5), 1.5);
    assert_eq!(echo_ref_opt_f64(&Some(1.5)), 1.5);
    assert!(echo_ref_opt_f64(&None).is_undefined());
}

#[wasm_bindgen_test]
fn functions() {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let cb = {
        let seen = AssertUnwindSafe(seen.clone());
        Closure::<dyn FnMut(Nullable<JsString>)>::new(move |v: Nullable<JsString>| {
            seen.borrow_mut().push(v.map(String::from));
        })
    };
    call_nullable(Function::closure_ref(&cb));
    mono_call_nullable(Function::closure_ref(&cb), 1u32);
    assert_eq!(
        *seen.borrow(),
        vec![
            Nullable::Null,
            Nullable::Value("a".to_string()),
            Nullable::Null,
            Nullable::Value("a".to_string()),
        ]
    );

    let len =
        Closure::<dyn FnMut(Nullable<JsString>) -> Option<Number>>::new(|v: Nullable<JsString>| {
            v.value().map(|s| Number::from(s.length()))
        });
    let f = echo_fn(Function::closure_ref(&len));
    assert_eq!(f.len(), 1);
}

// A `Nullable<T>` is a Rust enum, so it cannot be the erased `&JsValue` that
// `Promise::resolve(&T)` marshals; the promise is built from its JS value.
#[wasm_bindgen_test]
async fn promises() {
    let p: Promise<Nullable<JsString>> =
        Promise::resolve(&JsValue::from(Nullable::<JsString>::Null)).unchecked_into();
    assert_eq!(await_nullable(&p).await, Nullable::Null);
    let p: Promise<Nullable<JsString>> =
        Promise::resolve(&JsValue::from(Nullable::Value(JsString::from("a")))).unchecked_into();
    assert_eq!(
        await_nullable(&p).await,
        Nullable::Value(JsString::from("a"))
    );
    assert_eq!(
        mono_await_nullable(&p, 1u32).await,
        Nullable::Value("a".to_string())
    );
}

#[wasm_bindgen_test]
fn containers() {
    let arr = nullable_array();
    assert_eq!(arr.length(), 2);
    assert_eq!(describe_array(&arr), "Array:[\"a\",null]");
    let empty: Array<Nullable<JsString>> = Array::new_typed();
    assert_eq!(empty.length(), 0);
    let map = nullable_map();
    assert_eq!(map.size(), 2);
}
