//! `Option<T>` and `Nullable<T>` as the element of a shape, rather than a
//! bare argument or return: a borrowed slice, a `Function` signature, a
//! `Promise` and a js-sys container's type arguments. Each is
//! covered as a plain import and under `experimental_generic_mono`, which is
//! where exact nullability carriers are wanted everywhere.

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use js_sys::{Array, Function, JsString, Map, Number, Promise, Undefined};
use wasm_bindgen::convert::{IntoWasmAbi, RefIntoWasmAbi, VectorRefIntoWasmAbi};
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

    #[wasm_bindgen(js_name = echo)]
    fn echo_opt_ref_nullable_string(v: Option<&Nullable<String>>) -> JsValue;
    #[wasm_bindgen(js_name = echo)]
    fn echo_ref_f64(v: &f64) -> f64;
    #[wasm_bindgen(js_name = echo)]
    fn echo_ref_opt_f64(v: &Option<f64>) -> JsValue;
    #[wasm_bindgen(experimental_generic_mono, js_name = echo)]
    fn mono_ref<T: RefIntoWasmAbi>(v: &T) -> JsValue;

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
    assert_eq!(mono_ref(&1.5), 1.5);
    assert!(mono_ref(&Nullable::<f64>::Null).is_null());
    assert_eq!(mono_ref(&"a"), "a");
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

// Under `wasm_bindgen_unstable_generic_vec`, an owned vector crosses by its
// element's own ABI, so optional elements need no rows of their own.
#[cfg(wasm_bindgen_unstable_generic_vec)]
mod owned {
    use super::*;

    #[wasm_bindgen(module = "tests/wasm/nullable_elements.js")]
    extern "C" {
        // No JS class backs it, so its values are whatever JS passes.
        type Opaque;

        #[wasm_bindgen(js_name = describe)]
        fn opt_f64s(v: Vec<Option<f64>>) -> String;
        #[wasm_bindgen(js_name = describe)]
        fn nullable_strings(v: Vec<Nullable<String>>) -> String;
        #[wasm_bindgen(js_name = describe)]
        fn opt_nullable_u8s(v: Vec<Option<Nullable<u8>>>) -> String;
        #[wasm_bindgen(js_name = echo)]
        fn echo_opt_f64s(v: Vec<Option<f64>>) -> Vec<Option<f64>>;
        #[wasm_bindgen(js_name = echo)]
        fn echo_nullable_strings(v: Vec<Nullable<String>>) -> Vec<Nullable<String>>;
        #[wasm_bindgen(js_name = echo)]
        fn echo_opt_opaques(v: Vec<Option<Opaque>>) -> Vec<Option<Opaque>>;
        #[wasm_bindgen(js_name = echo)]
        fn echo_nullable_opaques(v: Vec<Nullable<Opaque>>) -> Vec<Nullable<Opaque>>;
        #[wasm_bindgen(js_name = echo)]
        fn echo_opt_nullable_opaques(
            v: Vec<Option<Nullable<Opaque>>>,
        ) -> Vec<Option<Nullable<Opaque>>>;
        #[wasm_bindgen(js_name = describe)]
        fn describe_opt_nullable_opaques(v: Vec<Option<Nullable<Opaque>>>) -> String;
        #[wasm_bindgen(js_name = echo)]
        fn echo_opt_byte_vecs(v: Vec<Option<Vec<u8>>>) -> Vec<Option<Vec<u8>>>;
    }

    fn opaque(v: &str) -> Opaque {
        JsValue::from(v).unchecked_into()
    }

    fn text(v: &Opaque) -> String {
        v.unchecked_ref::<JsValue>().as_string().unwrap()
    }

    #[wasm_bindgen_test]
    fn vectors() {
        assert_eq!(
            opt_f64s(vec![Some(1.5), None, Some(-2.0)]),
            "Array:[1.5,\"undefined\",-2]"
        );
        assert_eq!(
            nullable_strings(vec![Nullable::Value("a".to_string()), Nullable::Null]),
            "Array:[\"a\",null]"
        );
        assert_eq!(
            opt_nullable_u8s(vec![Some(Nullable::Value(7)), Some(Nullable::Null), None]),
            "Array:[7,null,\"undefined\"]"
        );
        assert_eq!(echo_opt_f64s(vec![Some(1.5), None]), vec![Some(1.5), None]);
        assert_eq!(
            echo_nullable_strings(vec![Nullable::Null, Nullable::Value("a".to_string())]),
            vec![Nullable::Null, Nullable::Value("a".to_string())]
        );
        assert_eq!(
            echo_opt_byte_vecs(vec![Some(vec![1, 2]), None, Some(vec![])]),
            vec![Some(vec![1, 2]), None, Some(vec![])]
        );
    }

    #[wasm_bindgen_test]
    fn opaque_vectors() {
        let v = echo_opt_opaques(vec![Some(opaque("a")), None]);
        assert_eq!(v.len(), 2);
        assert_eq!(text(v[0].as_ref().unwrap()), "a");
        assert!(v[1].is_none());

        let v = echo_nullable_opaques(vec![Nullable::Null, Nullable::Value(opaque("b"))]);
        assert!(matches!(v[0], Nullable::Null));
        assert!(matches!(&v[1], Nullable::Value(o) if text(o) == "b"));

        let v = echo_opt_nullable_opaques(vec![
            Some(Nullable::Value(opaque("c"))),
            Some(Nullable::Null),
            None,
        ]);
        assert!(matches!(&v[0], Some(Nullable::Value(o)) if text(o) == "c"));
        assert!(matches!(v[1], Some(Nullable::Null)));
        assert!(v[2].is_none());
        assert_eq!(
            describe_opt_nullable_opaques(vec![Some(Nullable::Null), None]),
            "Array:[null,\"undefined\"]"
        );
    }
}
