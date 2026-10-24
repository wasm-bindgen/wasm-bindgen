//! `Nullable<T>` and `Option<T>` as type arguments of js-sys containers,
//! `Function` signatures and `Promise`s. The element accessors are per-mono
//! under `js_sys_unstable_apis`, so Rust-side carriers marshal directly.

use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn array_nullable_js_string() {
    let arr: Array<Nullable<JsString>> = Array::new();
    arr.push(&Nullable::Value(JsString::from("a")));
    arr.push(&Nullable::Null);
    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get(0), Some(Nullable::Value(JsString::from("a"))));
    assert_eq!(arr.get(1), Some(Nullable::Null));
    assert_eq!(arr.get(2), None);
    arr.set(1, &Nullable::Value(JsString::from("b")));
    assert_eq!(arr.pop(), Some(Nullable::Value(JsString::from("b"))));
    assert!(arr.includes(&Nullable::Value(JsString::from("a")), 0));
    assert!(!arr.includes(&Nullable::Null, 0));
}

#[wasm_bindgen_test]
fn array_nullable_string() {
    let arr: Array<Nullable<String>> = Array::new();
    arr.push(&Nullable::Value("a".to_string()));
    arr.push(&Nullable::Null);
    assert_eq!(arr.get(0), Some(Nullable::Value("a".to_string())));
    assert_eq!(arr.get(1), Some(Nullable::Null));
    assert_eq!(arr.get(2), None);
    assert_eq!(arr.index_of(&Nullable::Null, 0), 1);
}

#[wasm_bindgen_test]
fn array_nullable_f64() {
    let arr: Array<Nullable<f64>> = Array::new();
    arr.push(&Nullable::Value(1.5));
    arr.push(&Nullable::Null);
    assert_eq!(arr.get(0), Some(Nullable::Value(1.5)));
    assert_eq!(arr.get(1), Some(Nullable::Null));
    assert_eq!(arr.get(2), None);
}

#[wasm_bindgen_test]
fn map_nullable() {
    let map: Map<JsString, Nullable<JsString>> = Map::new();
    map.set(&JsString::from("a"), &Nullable::Value(JsString::from("x")));
    map.set(&JsString::from("b"), &Nullable::Null);
    assert_eq!(
        map.get(&JsString::from("a")),
        Some(Nullable::Value(JsString::from("x")))
    );
    assert_eq!(map.get(&JsString::from("b")), Some(Nullable::Null));
    assert_eq!(map.get(&JsString::from("c")), None);
    assert!(map.has(&JsString::from("b")));

    let map: Map<String, Nullable<String>> = Map::new();
    map.set(&"a".to_string(), &Nullable::Value("x".to_string()));
    map.set(&"b".to_string(), &Nullable::Null);
    assert_eq!(
        map.get(&"a".to_string()),
        Some(Nullable::Value("x".to_string()))
    );
    assert_eq!(map.get(&"b".to_string()), Some(Nullable::Null));
    assert_eq!(map.get(&"c".to_string()), None);
    assert!(map.delete(&"b".to_string()));
    assert_eq!(map.size(), 1);
}

#[wasm_bindgen_test]
fn set_nullable() {
    let set: Set<Nullable<JsString>> = Set::new();
    set.add(&Nullable::Null);
    set.add(&Nullable::Value(JsString::from("a")));
    assert_eq!(set.size(), 2);
    assert!(set.has(&Nullable::Null));
    assert!(set.delete(&Nullable::Value(JsString::from("a"))));
    assert!(!set.has(&Nullable::Value(JsString::from("a"))));
}

#[wasm_bindgen_test]
async fn promise_nullable() {
    let p = Promise::resolve(&Nullable::<JsString>::Null);
    assert_eq!(JsFuture::from(p).await.unwrap(), Nullable::Null);
    let p = Promise::resolve(&Nullable::Value(JsString::from("a")));
    assert_eq!(
        JsFuture::from(p).await.unwrap(),
        Nullable::Value(JsString::from("a"))
    );
}

#[wasm_bindgen_test]
fn function_nullable() {
    let closure: Closure<dyn FnMut(Nullable<JsString>) -> Option<Number>> =
        Closure::new(|s: Nullable<JsString>| s.value().map(|s| Number::from(s.length())));
    let f: Function<fn(Nullable<JsString>) -> Option<Number>> = Function::from_closure(closure);
    assert_eq!(f.call1(&JsValue::NULL, &Nullable::Null).unwrap(), None);
    assert_eq!(
        f.call1(&JsValue::NULL, &Nullable::Value(JsString::from("ab")))
            .unwrap(),
        Some(Number::from(2))
    );
    assert_eq!(
        f.call(&JsValue::NULL, (&Nullable::Value(JsString::from("abc")),))
            .unwrap(),
        Some(Number::from(3))
    );
}
