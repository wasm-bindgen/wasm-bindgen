//! Runtime coverage for a bare shared reference to a generic type parameter
//! (`&T`) in a per-monomorphisation generic import
//! (`#[wasm_bindgen(experimental_generic_mono)]`).
//!
//! Each instantiation exercises a distinct referent shape:
//! - `&RefThing`: a JS-handle type, marshalled by handle index.
//! - `&JsValue`: marshalled as an externref.
//!
//! It also covers `Option<&T>`, `?Sized` referents, and owned arguments that
//! cross borrowed.

use core::borrow::Borrow;
use js_sys::{JsString, Object};
use wasm_bindgen::convert::{FromWasmAbi, RefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsStringLike;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_ref.js")]
extern "C" {
    type RefThing;

    #[wasm_bindgen(constructor)]
    fn new(val: f64) -> RefThing;

    // `&T` where `T` monomorphises to a JS-handle type: marshalled via the
    // handle's `IntoWasmAbi for &RefThing` impl.
    #[wasm_bindgen(experimental_generic_mono, js_name = readRefThing)]
    fn read_ref_thing<T>(x: &T) -> f64
    where
        T: RefIntoWasmAbi;

    // `&T` where `T` monomorphises to `JsValue`: marshalled as an externref.
    #[wasm_bindgen(experimental_generic_mono, js_name = echoRef)]
    fn echo_ref<T>(x: &T) -> JsValue
    where
        T: RefIntoWasmAbi;
}

#[wasm_bindgen_test]
fn generic_import_shared_ref_handle() {
    let thing = RefThing::new(42.0);
    assert_eq!(read_ref_thing(&thing), 42.0);
}

#[wasm_bindgen_test]
fn generic_import_shared_ref_jsvalue() {
    let value = JsValue::from("hello");
    assert_eq!(echo_ref(&value), JsValue::from("hello"));
}

#[wasm_bindgen(module = "tests/wasm/generic_import_ref.js")]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn show<T: RefIntoWasmAbi>(value: Option<&T>) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_owned(r#type: impl JsStringLike) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_unsized<Q>(value: &Q) -> String
    where
        Q: ?Sized + RefIntoWasmAbi;

    type Init<Cf = JsValue>;
    #[wasm_bindgen(constructor)]
    fn new() -> Init;
    #[wasm_bindgen(method, setter, experimental_generic_mono, js_name = cf)]
    fn set_cf<Cf: RefIntoWasmAbi>(this: &Init<Cf>, val: &Cf);
    #[wasm_bindgen(method, setter, experimental_generic_mono, js_name = cf)]
    fn set_maybe_cf<Cf: RefIntoWasmAbi>(this: &Init<Cf>, val: Option<&Cf>);

    type Req<B = JsValue, Cf = JsValue>;
    #[wasm_bindgen(method, getter, experimental_generic_mono)]
    fn input<B, Cf>(this: &Req<B, Cf>) -> String;
    #[wasm_bindgen(method, getter, experimental_generic_mono)]
    fn cf<B, Cf>(this: &Req<B, Cf>) -> Option<Cf>
    where
        Option<Cf>: FromWasmAbi;

    #[wasm_bindgen(catch, experimental_generic_mono, js_name = makeReq)]
    fn make_req<Cf>(input: impl JsStringLike, init: &Init<Cf>)
        -> Result<Req<JsValue, Cf>, JsValue>;
}

#[wasm_bindgen_test]
fn optional_references() {
    assert_eq!(show(Some(&1.5)), "number 1.5");
    assert_eq!(show(None::<&f64>), "undefined undefined");
    assert_eq!(show(Some(&7u32)), "number 7");
    assert_eq!(show(Some(&true)), "boolean true");
    assert_eq!(show(Some(&"x".to_string())), "string x");
    assert_eq!(show(None::<&String>), "undefined undefined");
    assert_eq!(show(Some(&JsValue::from("h"))), "string h");
    assert_eq!(show(Some(&JsString::from("j"))), "string j");
}

#[wasm_bindgen_test]
fn owned_arguments_cross_borrowed() {
    let r#type = String::from("owned");
    assert_eq!(show_owned(r#type), "string owned");
    assert_eq!(show_owned("borrowed"), "string borrowed");
    assert_eq!(show_owned(JsString::from("handle")), "string handle");
    assert_eq!(show_owned(Some("x").map(String::from).unwrap()), "string x");
}

fn show_key<K: Borrow<Q>, Q: ?Sized + RefIntoWasmAbi>(key: &Q) -> String {
    show_unsized(key)
}

#[wasm_bindgen_test]
fn unsized_references() {
    assert_eq!(show_unsized("k"), "string k");
    assert_eq!(show_key::<String, str>("k"), "string k");
}

struct InitBuilder<Cf> {
    inner: Init<Cf>,
}

impl<Cf: RefIntoWasmAbi> InitBuilder<Cf> {
    fn cf(self, val: &Cf) -> Self {
        self.inner.set_cf(val);
        self
    }
}

#[wasm_bindgen_test]
fn declared_bounds_only() {
    let init: Init<Object> = Init::new().unchecked_into();
    let cf = Object::new();
    js_sys::Reflect::set(&cf, &"colo".into(), &"LHR".into()).unwrap();
    let init = InitBuilder { inner: init }.cf(&cf).inner;
    let req = make_req("https://a", &init).unwrap();
    assert_eq!(req.input(), "https://a");
    assert!(req.cf().unwrap() == cf);
    init.set_maybe_cf(None);
    let req = make_req(String::from("https://b"), &init).unwrap();
    assert_eq!(req.input(), "https://b");
    assert!(req.cf().is_none());
}

#[wasm_bindgen_test]
fn native_class_params_by_reference() {
    let init: Init<String> = Init::new().unchecked_into();
    let init = InitBuilder { inner: init }.cf(&"LHR".to_string()).inner;
    assert_eq!(make_req("a", &init).unwrap().cf().as_deref(), Some("LHR"));
    init.set_maybe_cf(Some(&"SFO".to_string()));
    assert_eq!(make_req("a", &init).unwrap().cf().as_deref(), Some("SFO"));
    init.set_maybe_cf(None);
    assert_eq!(make_req("a", &init).unwrap().cf(), None);

    let init: Init<f64> = Init::new().unchecked_into();
    let init = InitBuilder { inner: init }.cf(&1.5).inner;
    assert_eq!(make_req("a", &init).unwrap().cf(), Some(1.5));
    init.set_maybe_cf(Some(&2.5));
    assert_eq!(make_req("a", &init).unwrap().cf(), Some(2.5));
    init.set_maybe_cf(None);
    assert_eq!(make_req("a", &init).unwrap().cf(), None);

    let init: Init<Option<String>> = Init::new().unchecked_into();
    init.set_cf(&Some("x".to_string()));
    let req: Req<JsValue, String> = make_req("a", &init).unwrap().unchecked_into();
    assert_eq!(req.cf().as_deref(), Some("x"));
    init.set_cf(&None);
    let req: Req<JsValue, String> = make_req("a", &init).unwrap().unchecked_into();
    assert_eq!(req.cf(), None);
}
