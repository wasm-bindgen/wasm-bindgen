use js_sys::*;
use wasm_bindgen::convert::Upcast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

fn canon<T: JsCanon>(value: T) -> T::Canon {
    value.to_js_canon()
}

fn string_promise(s: &str) -> Promise<String> {
    Promise::resolve(&JsString::from(s)).unchecked_into()
}

fn string_array(items: &[&str]) -> Array<String> {
    let arr: Array<JsString> = Array::new_typed();
    for item in items {
        arr.push(&JsString::from(*item));
    }
    arr.unchecked_into()
}

#[wasm_bindgen_test]
fn scalars() {
    let s: JsString = String::from("owned").to_js_canon();
    assert_eq!(s, "owned");
    let s: JsString = "borrowed".to_js_canon();
    assert_eq!(s, "borrowed");
    let s: JsString = 'c'.to_js_canon();
    assert_eq!(s, "c");

    let n: Number = 1.5f64.to_js_canon();
    assert_eq!(n.value_of(), 1.5);
    let n: Number = 7u32.to_js_canon();
    assert_eq!(n.value_of(), 7.0);
    let n: Number = (-3i8).to_js_canon();
    assert_eq!(n.value_of(), -3.0);

    let b: Boolean = true.to_js_canon();
    assert!(b.value_of());

    let u: Undefined = ().to_js_canon();
    assert!(JsValue::from(u).is_undefined());
}

#[wasm_bindgen_test]
fn identity() {
    let s = JsString::from("same");
    let c: JsString = s.clone().to_js_canon();
    assert_eq!(c, s);

    let o = Object::new();
    let c: Object = o.clone().to_js_canon();
    assert_eq!(c, o);

    let v = JsValue::from(3);
    let c: JsValue = v.clone().to_js_canon();
    assert_eq!(c, v);

    let n = Number::from(4);
    let c: Number = canon(n.clone());
    assert_eq!(c, n);

    let f: Function<fn(JsValue) -> JsValue> = Object::new().constructor().unchecked_into();
    let c: Function<fn(JsValue) -> JsValue> = f.clone().to_js_canon();
    assert_eq!(JsValue::from(c), JsValue::from(f));
}

#[wasm_bindgen_test]
async fn promise_relabels_to_js_string() {
    let promise = string_promise("hello");
    let promise: Promise<JsString> = promise.to_js_canon();
    let value: JsString = JsFuture::from(promise).await.unwrap().unchecked_into();
    assert_eq!(value, "hello");
}

#[wasm_bindgen_test]
async fn promise_infers_target() {
    async fn takes_js(promise: Promise<JsString>) -> JsString {
        JsFuture::from(promise).await.unwrap().unchecked_into()
    }
    assert_eq!(
        takes_js(string_promise("inferred").to_js_canon()).await,
        "inferred"
    );

    let promise: Promise<f64> = Promise::resolve(&Number::from(2)).unchecked_into();
    let promise = canon(promise);
    let value: Number = JsFuture::from(promise).await.unwrap().unchecked_into();
    assert_eq!(value.value_of(), 2.0);
}

#[wasm_bindgen_test]
fn array_relabels_elements() {
    let arr = string_array(&["a", "b"]);
    let arr: Array<JsString> = arr.to_js_canon();
    assert_eq!(arr.join(","), "a,b");

    let bools: Array<bool> = Array::<Boolean>::new_typed().unchecked_into();
    let bools: Array<Boolean> = bools.to_js_canon();
    assert_eq!(bools.length(), 0);
}

#[wasm_bindgen_test]
fn nested_containers() {
    let arr = string_array(&["x"]);
    let promise: Promise<Array<Option<String>>> = Promise::resolve(&arr).unchecked_into();
    let promise: Promise<Array<Option<JsString>>> = promise.to_js_canon();
    let _: &Object = promise.as_ref();

    let typed: Map<JsString, Number> = Map::new_typed();
    typed.set(&JsString::from("k"), &Number::from(1));
    let map: Map<String, f64> = typed.unchecked_into();
    let map: Map<JsString, Number> = map.to_js_canon();
    assert!(map.has(&JsString::from("k")));

    let set: Set<Promise<String>> = Set::<Promise<JsString>>::new_typed().unchecked_into();
    let set: Set<Promise<JsString>> = canon(set);
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn option_and_nullable_map_payload() {
    let some: Option<JsString> = Some(String::from("v")).to_js_canon();
    assert_eq!(some.unwrap(), "v");
    let none: Option<JsString> = None::<String>.to_js_canon();
    assert!(none.is_none());

    let value: Nullable<Number> = Nullable::Value(2.5f64).to_js_canon();
    assert_eq!(value.into_option().unwrap().value_of(), 2.5);
    let null: Nullable<Number> = Nullable::<f64>::Null.to_js_canon();
    assert!(null.into_option().is_none());

    let nested: Option<Nullable<Promise<JsString>>> =
        Some(Nullable::Value(string_promise("n"))).to_js_canon();
    assert!(nested.unwrap().into_option().is_some());
}

#[wasm_bindgen_test]
fn js_option_and_js_nullable_relabel() {
    let opt: JsOption<String> = JsOption::wrap(JsString::from("o")).unchecked_into();
    let opt: JsOption<JsString> = opt.to_js_canon();
    assert_eq!(opt.into_option().unwrap(), "o");

    let empty: JsOption<Promise<String>> = JsOption::<Promise<JsString>>::new().unchecked_into();
    let empty: JsOption<Promise<JsString>> = empty.to_js_canon();
    assert!(empty.is_empty());

    let nullable: JsNullable<f64> = JsNullable::wrap(Number::from(9)).unchecked_into();
    let nullable: JsNullable<Number> = nullable.to_js_canon();
    assert_eq!(nullable.into_option().unwrap().value_of(), 9.0);
}

#[wasm_bindgen_test]
async fn string_upcasts_to_js_value() {
    let promise: Promise<JsValue> = string_promise("direct").upcast_into();
    let value = JsFuture::from(promise).await.unwrap();
    assert_eq!(value.as_string().unwrap(), "direct");

    let arr: Array<JsValue> = string_array(&["s"]).upcast_into();
    assert_eq!(arr.join(","), "s");

    let promise: Promise<&str> = string_promise("ref").unchecked_into();
    let promise: Promise<JsValue> = promise.upcast_into();
    let value = JsFuture::from(promise).await.unwrap();
    assert_eq!(value.as_string().unwrap(), "ref");
}
