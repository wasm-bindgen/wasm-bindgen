//! Under `js_sys_unstable_apis`, an element type is any type with an ABI,
//! crossing as itself at each instantiation.

use js_sys::*;
use wasm_bindgen::convert::{FromWasmAbi, RefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsStringLike;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
    fn describe(v: &JsValue) -> String;
    #[wasm_bindgen(js_name = asyncStrings)]
    fn async_strings() -> AsyncIterator<String>;
    fn strings() -> Array<String>;
    fn later() -> Promise<()>;
    fn optionals() -> Vec<Option<String>>;
    #[wasm_bindgen(js_name = optionalNumbers)]
    fn optional_numbers() -> Vec<Option<f64>>;
    #[wasm_bindgen(js_name = optionals)]
    fn optional_handles() -> Vec<Option<JsString>>;
    #[wasm_bindgen(experimental_generic_mono, js_name = optionals)]
    fn optionals_of<T>() -> Vec<Option<T>>
    where
        Vec<Option<T>>: FromWasmAbi;
    #[wasm_bindgen(js_name = callWith)]
    fn call_unit(f: &Function<fn(String)>, a: &str);
    #[wasm_bindgen(js_name = throwingIterator)]
    fn throwing_iterator() -> Iterator<String>;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn describe_like(values: &Array<impl JsStringLike>) -> String;

    #[wasm_bindgen(experimental_generic_mono)]
    fn show<T: RefIntoWasmAbi>(value: Option<&T>) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_owned(r#type: impl JsStringLike) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_view<T: TypedArray>(view: &T) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = bufferKind)]
    fn buffer_kind(bytes: impl BufferSourceLike) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = bufferKind)]
    fn maybe_buffer_kind(bytes: Option<impl BufferSourceLike>) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = keepBytes)]
    fn keep_view(bytes: impl BufferSourceLike);
    #[wasm_bindgen(experimental_generic_mono, js_name = keepBytes)]
    fn keep_copy(#[wasm_bindgen(slice = copy)] bytes: impl BufferSourceLike);
    #[wasm_bindgen(experimental_generic_mono, js_name = keepBytes)]
    fn keep_maybe_copy(#[wasm_bindgen(slice = copy)] bytes: Option<impl BufferSourceLike>);
    #[wasm_bindgen(js_name = keptBytes)]
    fn kept_bytes() -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = callThunk)]
    fn call_thunk<T: FromWasmAbi>(f: &Function<fn() -> T>) -> T;

    #[wasm_bindgen(js_name = callWith)]
    fn call_with(f: &Function<fn(String, f64) -> String>, a: &str, b: f64) -> String;
}

#[wasm_bindgen_test]
fn array_of_strings() {
    let a: Array<String> = ["a", "b"].into_iter().map(String::from).collect();
    a.push(&"c".into());
    a.push(&String::from("d"));
    a.push(&"e".to_string());
    assert_eq!(a.get(0).as_deref(), Some("a"));
    assert_eq!(a.get(9), None);
    assert_eq!(a.at(-1).as_deref(), Some("e"));
    assert_eq!(a.iter().collect::<Vec<String>>(), ["a", "b", "c", "d", "e"]);
    assert_eq!(a.to_vec(), ["a", "b", "c", "d", "e"]);
    assert_eq!(
        describe(&a),
        r#"Array:["string a","string b","string c","string d","string e"]"#
    );

    assert!(a.includes(&"c".into(), 0));
    assert!(!a.includes(&"z".into(), 0));
    assert_eq!(a.index_of(&"d".into(), 0), 3);
    assert_eq!(a.last_index_of(&"b".to_string(), -1), 1);

    a.set(1, &"B".into());
    a.unshift(&"z".into());
    assert_eq!(a.shift().as_deref(), Some("z"));
    assert_eq!(a.pop().as_deref(), Some("e"));
    assert_eq!(a.slice(1, 3).to_vec(), ["B", "c"]);
    assert_eq!(
        a.concat(&strings()).into_iter().collect::<Vec<_>>(),
        ["a", "B", "c", "d", "x", "y"]
    );

    // `Array::from` is `Array.from`, so the `From` impls go through `into`.
    let mut b: Array<String> = vec!["p".to_string()].into();
    b.extend(["q", "r"].map(String::from));
    b.extend(&["s".to_string()]);
    assert_eq!(b.to_vec(), ["p", "q", "r", "s"]);
    let c: Array<String> = (&["s".to_string()][..]).into();
    assert_eq!(c.to_vec(), ["s"]);
    assert_eq!(
        Array::of(&["t".to_string(), "u".to_string()]).to_vec(),
        ["t", "u"]
    );
    assert_eq!(
        Array::<String>::of2(&"v".into(), &"w".into()).to_vec(),
        ["v", "w"]
    );
    assert_eq!(Array::from(&b).unwrap().to_vec(), ["p", "q", "r", "s"]);
    assert_eq!(strings().iter().collect::<Vec<_>>(), ["x", "y"]);
}

#[wasm_bindgen_test]
fn array_of_scalars() {
    let flags: Array<bool> = [true, false].into_iter().collect();
    assert_eq!(flags.get(0), Some(true));
    flags.push(&true);
    assert_eq!(flags.iter().collect::<Vec<_>>(), [true, false, true]);
    assert_eq!(
        describe(&flags),
        r#"Array:["boolean true","boolean false","boolean true"]"#
    );

    let numbers: Array<f64> = vec![1.5, 2.0].into();
    numbers.push(&3.0);
    assert_eq!(numbers.to_vec(), [1.5, 2.0, 3.0]);
    assert_eq!(numbers.into_iter().sum::<f64>(), 6.5);
    let numbers = Array::of(&[4u32, 5]);
    assert_eq!(describe(&numbers), r#"Array:["number 4","number 5"]"#);
    assert_eq!(numbers.get(1), Some(5));
}

#[wasm_bindgen_test]
fn array_of_handles() {
    let s = JsString::from("a");
    let a = Array::of2(&s, &s);
    a.push(&s);
    let all: Vec<JsString> = a.iter().collect();
    assert_eq!(all.len(), 3);
    assert!(a.includes(&s, 0));

    // Borrows deref-coerce to the element type, as with the erased API.
    let erased: Array<JsValue> = Array::of2(&s, &JsValue::from(1));
    erased.push(&s);
    assert_eq!(erased.length(), 3);
    assert_eq!(erased.get(2), Some(JsValue::from("a")));

    let values = [JsValue::from(1), JsValue::from("b")];
    let collected: Array = values.iter().collect();
    assert_eq!(collected.length(), 2);
    assert_eq!(collected.get(1), Some(JsValue::from("b")));
}

#[wasm_bindgen_test]
fn js_string_like_arrays_reach_js_as_strings() {
    let owned: Array<String> = ["a", "b"].into_iter().map(String::from).collect();
    let handles: Array<JsString> = ["a", "b"].into_iter().map(JsString::from).collect();
    let expected = r#"Array:["string a","string b"]"#;
    assert_eq!(describe_like(&owned), expected);
    assert_eq!(describe_like(&handles), expected);
}

#[wasm_bindgen_test]
async fn promise_of_rust_types() {
    let v: String = JsFuture::from(Promise::resolve("x")).await.unwrap();
    assert_eq!(v, "x");
    let v: String = Promise::resolve("y".to_string()).await.unwrap();
    assert_eq!(v, "y");
    let b: bool = JsFuture::from(Promise::resolve(true)).await.unwrap();
    assert!(b);
    let n: f64 = Promise::resolve(2.5f64).await.unwrap();
    assert_eq!(n, 2.5);

    // A promise resolves to its resolution, not to itself.
    let inner: Promise<String> = Promise::resolve("z");
    let outer: Promise<String> = Promise::resolve(&inner);
    assert_eq!(outer.await.unwrap(), "z");
}

#[wasm_bindgen_test]
fn map_of_rust_types() {
    let m: Map<String, f64> = Map::new_typed();
    m.set(&"k".into(), &1.0);
    m.set(&"j".into(), &2.0);
    assert_eq!(m.get("k"), Some(1.0));
    assert_eq!(m.get(&"j".to_string()), Some(2.0));
    assert_eq!(m.get("missing"), None);
    assert!(m.has("k"));
    assert_eq!(m.size(), 2);

    let keys: Vec<String> = m.keys().into_iter().collect();
    assert_eq!(keys, ["k", "j"]);
    let values: Vec<f64> = m.values().into_iter().collect();
    assert_eq!(values, [1.0, 2.0]);
    let entries: Vec<(String, f64)> = m.iter().collect();
    assert_eq!(entries, [("k".to_string(), 1.0), ("j".to_string(), 2.0)]);
    let mut total = 0.0;
    for (_, v) in &m {
        total += v;
    }
    assert_eq!(total, 3.0);

    let copy: Map<String, f64> = m.iter().collect();
    assert_eq!(copy.get("j"), Some(2.0));
    let mut extended = copy.clone();
    extended.extend([("z".to_string(), 9.0)]);
    assert_eq!(m.size(), 2);
    assert_eq!(copy.get("z"), Some(9.0));

    let mut seen = Vec::new();
    m.for_each(&mut |value, key| seen.push((key, value)));
    assert_eq!(seen, [("k".to_string(), 1.0), ("j".to_string(), 2.0)]);

    assert!(m.delete("k"));
    assert!(!m.has("k"));

    // A present key reads as `Some` even when its value is `null`.
    let m: Map<String, Option<String>> = Map::new_typed();
    m.set(&"a".into(), &Some("x".into()));
    m.set(&"b".into(), &None);
    assert_eq!(m.get("a"), Some(Some("x".to_string())));
    assert_eq!(m.get("b"), Some(None));
    assert_eq!(m.get("c"), None);

    let m: Map<JsValue, f64> = Map::new_typed();
    let key = JsValue::from("k");
    m.set(&key, &1.0);
    assert_eq!(m.get(&key), Some(1.0));
}

#[wasm_bindgen_test]
fn set_of_rust_types() {
    let s: Set<String> = Set::new_from_items(&["a".to_string()]);
    s.add(&"b".into()).add(&"a".into());
    assert_eq!(s.size(), 2);
    assert!(s.has("b"));
    let values: Vec<String> = s.values().into_iter().collect();
    assert_eq!(values, ["a", "b"]);
    assert_eq!(s.iter().collect::<Vec<_>>(), ["a", "b"]);
    let mut joined = String::new();
    for v in &s {
        joined += &v;
    }
    assert_eq!(joined, "ab");
    let pairs: Vec<(String, String)> = Entries::from(s.entries()).collect();
    assert_eq!(pairs[1], ("b".to_string(), "b".to_string()));

    let mut seen = Vec::new();
    s.for_each(&mut |value: String| seen.push(value));
    assert_eq!(seen, ["a", "b"]);

    assert!(s.delete("a"));
    assert!(!s.has(&"a".to_string()));

    let numbers: Set<u32> = Set::new_typed();
    numbers.add(&1).add(&1).add(&2);
    assert_eq!(numbers.size(), 2);

    let mut numbers: Set<u32> = [3, 1, 3].into_iter().collect();
    numbers.extend(&[4, 1]);
    assert_eq!(numbers.iter().collect::<Vec<_>>(), [3, 1, 4]);
}

#[wasm_bindgen_test]
async fn iterators_of_rust_types() {
    let values: Array<String> = strings();
    let it: Iterator<String> = values.values();
    let collected: Vec<String> = it.into_iter().collect();
    assert_eq!(collected, ["x", "y"]);
    let mut seen = Vec::new();
    for v in &values.values() {
        seen.push(v);
    }
    assert_eq!(seen, ["x", "y"]);

    let fallible: Result<Vec<String>, JsValue> = values.values().try_iter().collect();
    assert_eq!(fallible.unwrap(), ["x", "y"]);
    let thrown = throwing_iterator()
        .try_iter()
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(thrown.unwrap_err().as_string().as_deref(), Some("boom"));

    let indexed: Vec<(u32, String)> = Entries::from(values.entries()).collect();
    assert_eq!(indexed, [(0, "x".to_string()), (1, "y".to_string())]);
    let pair: (u32, String) = values.entries().into_iter().next().unwrap().into();
    assert_eq!(pair, (0, "x".to_string()));

    let keys: Array<String> = Object::keys(&values.unchecked_ref::<Object>());
    assert_eq!(keys.to_vec(), ["0", "1"]);

    let it = async_strings();
    let mut collected = Vec::new();
    loop {
        let next = JsFuture::from(it.next().unwrap()).await.unwrap();
        if next.done() {
            break;
        }
        collected.push(next.value());
    }
    assert_eq!(collected, ["a", "b"]);

    #[cfg(feature = "futures-core-03-stream")]
    {
        use futures_lite::stream::StreamExt;
        let mut stream = js_sys::futures::stream::JsStream::from(async_strings());
        assert_eq!(stream.next().await, Some(Ok("a".to_string())));
        assert_eq!(stream.next().await, Some(Ok("b".to_string())));
        assert_eq!(stream.next().await, None);
    }
}

#[wasm_bindgen_test]
fn functions_of_rust_types() {
    let f: Function<fn(String, f64) -> String> = Function::from_fn(|s, n| format!("{s}{n}"));
    assert_eq!(call_with(&f, "a", 1.0), "a1");

    let closure = Closure::<dyn FnMut(String, f64) -> String>::new(|s, n| format!("{n}{s}"));
    assert_eq!(call_with(Function::closure_ref(&closure), "b", 2.0), "2b");
}

#[wasm_bindgen_test]
fn function_calls_with_rust_types() {
    let f: Function<fn(String, f64) -> String> = Function::from_fn(|s, n| format!("{s}{n}"));
    assert_eq!(
        f.call(&JsValue::NULL, (&"x".to_string(), &2.0)).unwrap(),
        "x2"
    );
    assert_eq!(
        f.call2(&JsValue::NULL, &"y".to_string(), &3.0).unwrap(),
        "y3"
    );

    let g: Function<fn() -> bool> = Function::from_fn(|| true);
    assert!(g.call(&JsValue::NULL, ()).unwrap());
}

#[wasm_bindgen_test]
async fn typed_promise_new() {
    let p: Promise<String> = Promise::new(&mut |resolve, _reject| {
        resolve
            .call(&JsValue::NULL, (&"done".to_string(),))
            .unwrap();
    });
    assert_eq!(p.await.unwrap(), "done");

    let p = Promise::<f64>::new(&mut |resolve, _reject| {
        resolve.call1(&JsValue::NULL, &1.5).unwrap();
    });
    assert_eq!(p.await.unwrap(), 1.5);
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

#[wasm_bindgen_test]
fn rust_string_returns() {
    let e = Error::new("boom");
    let message: String = e.message();
    assert_eq!(message, "boom");
    assert_eq!(e.name(), "Error");
    assert_eq!(e.to_string(), "Error: boom");

    let d = Date::new_with_value(0.0);
    assert_eq!(d.to_iso_string(), "1970-01-01T00:00:00.000Z");
    assert_eq!(
        Date::new_with_value("1970-01-01T00:00:00.000Z").get_time(),
        0.0
    );
    assert_eq!(
        Date::new_with_value(86_400_000u32).to_iso_string(),
        "1970-01-02T00:00:00.000Z"
    );

    let f: Function<fn() -> bool> = Function::from_fn(|| true);
    let _name: String = f.name();
}

#[wasm_bindgen_test]
fn resolver_upcasts_to_untyped_function() {
    let _p = Promise::<JsValue>::new(&mut |resolve, _| {
        let untyped: &Function = resolve.upcast();
        untyped.call1(&JsValue::NULL, &JsValue::NULL).unwrap();
    });
}

#[wasm_bindgen_test]
async fn unit_promises_and_callbacks() {
    let () = later().await.unwrap();

    let seen = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let s = seen.clone();
    let c = Closure::<dyn FnMut(String)>::new(move |v: String| *s.borrow_mut() = v);
    let f: Function<fn(String)> = c.into();
    call_unit(&f, "x");
    assert_eq!(*seen.borrow(), "x");
}

// A `Function` of a generic callback signature crosses with no bounds on its
// argument types.
#[allow(dead_code)]
fn generic_function_abi<E, R>(f: &Function<fn(E, String) -> R>) {
    fn crosses<T: wasm_bindgen::convert::IntoWasmAbi + wasm_bindgen::describe::WasmDescribe>(_: T) {
    }
    crosses(f);
}

#[wasm_bindgen_test]
fn records_and_borrowed_arrays() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), "x".to_string());
    let record: Object<String> = map.iter().collect();
    let mut record2: Object<String> = [("b", "y".to_string())].into_iter().collect();
    record2.extend(&map);
    let mut entries: Vec<(String, String)> = record2.iter().collect();
    entries.sort();
    assert_eq!(
        entries,
        [("a".into(), "x".into()), ("b".into(), "y".into())]
    );
    let back: std::collections::HashMap<String, String> = (&record).into_iter().collect();
    assert_eq!(back, map);

    let a: Array<String> = strings();
    let mut joined = String::new();
    for s in &a {
        joined += &s;
    }
    assert_eq!(joined, "xy");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map<String, f64>, extends = Object, js_name = Map)]
    type ScoreMap;
    #[wasm_bindgen(constructor, js_class = Map)]
    fn new() -> ScoreMap;

    #[wasm_bindgen(extends = Map<String, T>, extends = Object, js_name = Map)]
    type Scores<T = JsValue>;
    #[wasm_bindgen(constructor, js_class = Map)]
    fn new_scores<T>() -> Scores<T>;

    #[wasm_bindgen(extends = Map<T, T>, extends = Object, js_name = Map)]
    type Pairs<T>;
}

#[wasm_bindgen_test]
fn extends_with_type_arguments() {
    let m = ScoreMap::new();
    m.set(&"a".to_string(), &1.5);
    assert_eq!(m.get("a"), Some(1.5));
    let parent: &Map<String, f64> = m.as_ref();
    assert_eq!(parent.size(), 1);
    let parent: Map<String, f64> = m.into();
    assert_eq!(parent.get("a"), Some(1.5));

    let s: Scores<u32> = Scores::new_scores();
    s.set(&"b".to_string(), &2);
    assert_eq!(s.get("b"), Some(2));
    let parent: &Map<String, u32> = s.as_ref();
    assert_eq!(parent.size(), 1);
}

#[wasm_bindgen_test]
fn vectors_of_options() {
    let some = |s: &str| Some(s.to_string());
    assert_eq!(optionals(), [some("a"), None, None, some("b")]);
    assert_eq!(optionals_of::<String>(), [some("a"), None, None, some("b")]);
    assert_eq!(optional_numbers(), [Some(1.0), None, Some(2.5)]);
    let handles = optional_handles();
    assert_eq!(handles[0].as_ref().unwrap(), "a");
    assert!(handles[1].is_none() && handles[2].is_none());
    assert_eq!(optionals_of::<JsValue>()[3], Some("b".into()));
}

#[wasm_bindgen_test]
fn array_buffer_bytes() {
    let buf = ArrayBuffer::from(&b"abc"[..]);
    assert_eq!(buf.byte_length(), 3);
    assert_eq!(buf.to_vec(), b"abc");
    let bytes: Vec<u8> = buf.into();
    assert_eq!(bytes, b"abc");
    let buf: ArrayBuffer = vec![1, 2].into();
    assert_eq!(Vec::from(&buf), [1, 2]);
}

// A `JsGeneric` (here `TypedArray`) parameter passes by reference, or as an
// `Option<&T>`, with no `IntoWasmAbi` bound.
fn show_any_view<T: TypedArray>(view: &T) -> String {
    show_view(view)
}

fn show_any_handle<T: wasm_bindgen::JsGeneric>(value: Option<&T>) -> String {
    show(value)
}

#[wasm_bindgen_test]
fn handle_params_need_no_abi_bounds() {
    assert_eq!(
        show_any_view(&Uint8Array::new_from_slice(&[1, 2])),
        "object 1,2"
    );
    assert_eq!(show_any_handle(Some(&JsString::from("x"))), "string x");
    assert_eq!(show_any_handle(None::<&JsString>), "undefined undefined");
    assert_eq!(show_any_handle(None::<&JsValue>), "undefined undefined");
}

#[wasm_bindgen_test]
fn buffer_sources() {
    let bytes = [1u8, 2, 3];
    assert_eq!(buffer_kind(&bytes[..]), "Uint8Array:1,2,3");
    assert_eq!(buffer_kind(bytes.to_vec()), "Uint8Array:1,2,3");
    let buf = ArrayBuffer::from(&bytes[..]);
    assert_eq!(buffer_kind(&buf), "ArrayBuffer:1,2,3");
    let view = Uint8Array::new_from_slice(&bytes);
    assert_eq!(buffer_kind(&view), "Uint8Array:1,2,3");
    assert_eq!(buffer_kind(view.subarray(1, 3)), "Uint8Array:2,3");
    assert_eq!(buffer_kind(DataView::new(&buf, 1, 1)), "DataView:2");
    assert_eq!(maybe_buffer_kind(Some(&buf)), "ArrayBuffer:1,2,3");
    assert_eq!(maybe_buffer_kind(None::<&[u8]>), "none");
}

#[wasm_bindgen_test]
fn function_once() {
    let s = String::from("moved");
    let out = call_thunk(&Function::once(move || s));
    assert_eq!(out, "moved");
}

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = maybeAsync)]
    fn maybe_async(fail: bool) -> Promise<String>;
    #[wasm_bindgen(catch, experimental_generic_mono, js_name = maybeAsync)]
    fn maybe_async_like(fail: impl JsBooleanOrStringLike) -> Promise<String>;

    type Thrower;
    #[wasm_bindgen(constructor)]
    fn new() -> Thrower;
    #[wasm_bindgen(catch, method, js_name = maybeAsync)]
    fn run(this: &Thrower, fail: bool) -> Promise<String>;
    #[wasm_bindgen(catch, static_method_of = Thrower, js_name = maybeAsync)]
    fn make(fail: bool) -> Promise<String>;
    #[wasm_bindgen(catch, method, experimental_generic_mono, js_name = echo)]
    fn echo<T: RefIntoWasmAbi>(this: &Thrower, value: &T, fail: bool) -> Promise<T>;

    #[wasm_bindgen(catch, js_name = maybeAsync)]
    fn maybe_async_err(fail: bool) -> Promise<String, Failed>;
    #[wasm_bindgen(catch, method, experimental_generic_mono, js_name = echo)]
    fn echo_err<T: RefIntoWasmAbi>(this: &Thrower, value: &T, fail: bool) -> Promise<T, Failed>;
}

#[derive(Debug, PartialEq)]
struct Failed(String);

impl From<JsValue> for Failed {
    fn from(e: JsValue) -> Self {
        Failed(e.unchecked_into::<Error>().message())
    }
}

#[wasm_bindgen_test]
async fn promise_error_type() {
    async fn tail(fail: bool) -> Result<String, Failed> {
        maybe_async_err(fail).await
    }
    async fn chained() -> Result<f64, Failed> {
        let s = maybe_async_err(false).await?;
        Thrower::new().echo_err(&(s.len() as f64), false).await
    }
    assert_eq!(tail(false).await, Ok("ok".to_string()));
    assert_eq!(tail(true).await, Err(Failed("sync".to_string())));
    assert_eq!(chained().await, Ok(2.0));
    assert_eq!(
        Thrower::new().echo_err(&1.0, true).await,
        Err(Failed("sync".to_string()))
    );

    let p = maybe_async_err(false);
    let js_err: &Promise<String> = p.upcast();
    let js_err: Result<String, JsValue> = js_err.clone().await;
    assert_eq!(js_err.unwrap(), "ok");
    let number = Thrower::new().echo_err(&1.5, false);
    let erased: &Promise = number.upcast();
    assert_eq!(erased.clone().await.unwrap().as_f64(), Some(1.5));
    let back: Promise<String, Failed> = Promise::resolve("x".to_string()).upcast_into();
    assert_eq!(back.await, Ok("x".to_string()));
    let fut: JsFuture<String, Failed> = JsFuture::from(p.clone());
    assert_eq!(fut.await, Ok("ok".to_string()));
}

#[wasm_bindgen_test]
async fn catch_rejects_sync_throws() {
    async fn message<T: wasm_bindgen::convert::FromWasmAbi + 'static>(p: Promise<T>) -> String {
        p.await.err().unwrap().unchecked_into::<Error>().message()
    }
    assert_eq!(maybe_async(false).await.unwrap(), "ok");
    assert_eq!(message(maybe_async(true)).await, "sync");
    assert_eq!(maybe_async_like("").await.unwrap(), "ok");
    assert_eq!(message(maybe_async_like("fail")).await, "sync");
    let t = Thrower::new();
    assert_eq!(t.run(false).await.unwrap(), "ok");
    assert_eq!(message(t.run(true)).await, "sync");
    assert_eq!(message(Thrower::make(true)).await, "sync");
    assert_eq!(t.echo(&2.5, false).await.unwrap(), 2.5);
    assert_eq!(message(t.echo(&"x".to_string(), true)).await, "sync");
}

#[wasm_bindgen_test]
fn buffer_source_stored_copy() {
    let mut bytes = vec![1u8, 2, 3];
    keep_view(&bytes[..]);
    bytes[0] = 9;
    assert_eq!(kept_bytes(), "Uint8Array:9,2,3");
    keep_copy(&bytes[..]);
    bytes[0] = 7;
    assert_eq!(kept_bytes(), "Uint8Array:9,2,3");
    keep_maybe_copy(Some(&bytes[..]));
    bytes[0] = 5;
    assert_eq!(kept_bytes(), "Uint8Array:7,2,3");
    keep_copy(bytes.clone());
    assert_eq!(kept_bytes(), "Uint8Array:5,2,3");
    let buf = ArrayBuffer::from(&bytes[..]);
    keep_copy(&buf);
    assert_eq!(kept_bytes(), "ArrayBuffer:5,2,3");
    keep_copy(Uint8Array::new_from_slice(&bytes).subarray(1, 3));
    assert_eq!(kept_bytes(), "Uint8Array:2,3");
    keep_maybe_copy(None::<&[u8]>);
    assert_eq!(kept_bytes(), "none");
}

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
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
    Reflect::set(&cf, "colo", &"LHR".into()).unwrap();
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

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colo {
    Lhr = "LHR",
    Sfo = "SFO",
}

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
    #[wasm_bindgen(js_name = nullableMap)]
    fn nullable_map() -> Map<String, JsNullable<String>>;
    #[wasm_bindgen(js_name = nullableNumbers)]
    fn nullable_numbers() -> Array<JsNullable<f64>>;
    #[wasm_bindgen(js_name = identity)]
    fn nullable_bool(v: &JsValue) -> JsNullable<bool>;
    #[wasm_bindgen(js_name = identity)]
    fn nullable_colo(v: &JsValue) -> JsNullable<Colo>;
    #[wasm_bindgen(js_name = identity)]
    fn nullable_maybe(v: &JsValue) -> JsNullable<Option<String>>;
    #[wasm_bindgen(js_name = identity)]
    fn nullable_object(v: &JsValue) -> JsNullable<Object>;
    #[wasm_bindgen(js_name = identity)]
    fn later_nullable(v: &JsValue) -> Promise<JsNullable<String>>;
    #[wasm_bindgen(experimental_generic_mono, js_name = identity)]
    fn echo_nullable<T>(v: &JsNullable<T>) -> JsNullable<T>;

    #[wasm_bindgen(js_name = identity)]
    fn maybe_value(v: &JsValue) -> Option<JsValue>;
    #[wasm_bindgen(experimental_generic_mono, js_name = identity)]
    fn maybe_of<T>(v: &JsValue) -> Option<T>
    where
        Option<T>: FromWasmAbi;
    #[wasm_bindgen(js_name = identity)]
    fn later_maybe(v: &JsValue) -> Promise<Option<JsValue>>;
    #[wasm_bindgen(js_name = callWithEach)]
    fn call_with_each(f: &Function<fn(Option<JsValue>) -> bool>, values: &Array) -> Array<bool>;
}

#[wasm_bindgen_test]
fn nullable_reads_any_abi_type() {
    let map = nullable_map();
    assert_eq!(map.get("a").and_then(Option::from), Some("x".to_string()));
    let b = map.get("b").unwrap();
    assert!(b.is_null());
    assert_eq!(b.into_option(), None);
    assert!(map.get("c").unwrap().is_null());
    assert!(map.get("d").is_none());

    let numbers: Vec<Option<f64>> = nullable_numbers().iter().map(Option::from).collect();
    assert_eq!(numbers, [Some(1.0), None, None, Some(2.5)]);

    assert_eq!(nullable_bool(&JsValue::FALSE).into_option(), Some(false));
    assert_eq!(nullable_bool(&JsValue::NULL).into_option(), None);
    assert_eq!(nullable_colo(&"SFO".into()).into_option(), Some(Colo::Sfo));
    assert_eq!(nullable_colo(&JsValue::UNDEFINED).into_option(), None);
    assert_eq!(
        nullable_maybe(&"x".into()).into_option(),
        Some(Some("x".to_string()))
    );
    assert_eq!(nullable_maybe(&JsValue::NULL).into_option(), None);
    let obj = Object::new();
    assert!(nullable_object(&obj).into_option().unwrap() == obj);
    assert!(nullable_object(&JsValue::NULL).is_null());
}

#[wasm_bindgen_test]
fn nullable_writes_any_abi_type() {
    let mut map: Map<String, JsNullable<String>> = [
        ("a".to_string(), JsNullable::wrap("x".to_string())),
        ("b".to_string(), JsNullable::new()),
    ]
    .into_iter()
    .collect();
    map.extend([("c".to_string(), None.into())]);
    assert_eq!(map.get("a").unwrap().into_option().as_deref(), Some("x"));
    assert!(map.get("b").unwrap().is_null());
    assert!(map.get("c").unwrap().is_null());
    assert!(JsValue::is_null(&map.get("b").unwrap()));

    let numbers: Array<JsNullable<f64>> = [Some(1.5), None]
        .into_iter()
        .map(JsNullable::from)
        .collect();
    assert_eq!(numbers.length(), 2);
    assert_eq!(numbers.get(0).unwrap().unwrap(), 1.5);
    assert_eq!(numbers.get(1).and_then(Option::from), None::<f64>);
    assert_eq!(JsNullable::<f64>::new().unwrap_or_default(), 0.0);

    assert_eq!(
        echo_nullable(&JsNullable::wrap(true)).into_option(),
        Some(true)
    );
    assert_eq!(
        echo_nullable(&JsNullable::wrap(Colo::Lhr)).into_option(),
        Some(Colo::Lhr)
    );
    assert_eq!(
        echo_nullable(&JsNullable::wrap(Some("y".to_string()))).into_option(),
        Some(Some("y".to_string()))
    );
    assert_eq!(
        echo_nullable(&JsNullable::<Option<String>>::from_option(None)).into_option(),
        None
    );
    assert_eq!(
        echo_nullable(&JsNullable::wrap(JsString::from("z"))).unwrap(),
        "z"
    );
}

#[wasm_bindgen_test]
fn nullable_debug_and_eq() {
    let x = JsNullable::wrap("x".to_string());
    assert!(format!("{x:?}").ends_with("?(\"x\")"));
    assert!(format!("{:?}", JsNullable::<f64>::new()).ends_with("?(null)"));
    assert_eq!(x, JsNullable::wrap("x".to_string()));
    assert_ne!(x, JsNullable::wrap("y".to_string()));
    assert_eq!(JsNullable::wrap(1.5), JsNullable::wrap(1.5));
    assert_eq!(JsNullable::<bool>::new(), JsNullable::default());
}

#[wasm_bindgen_test]
async fn nullable_promise() {
    let p = later_nullable(&Promise::resolve(&JsValue::from("x")));
    assert_eq!(p.await.unwrap().into_option().as_deref(), Some("x"));
    let p = later_nullable(&Promise::resolve(&JsValue::NULL));
    assert_eq!(p.await.unwrap().into_option(), None);
}

#[wasm_bindgen_test]
async fn option_jsvalue_reads_fold_null() {
    assert_eq!(maybe_value(&JsValue::NULL), None);
    assert_eq!(maybe_value(&JsValue::UNDEFINED), None);
    assert_eq!(maybe_value(&1.into()), Some(JsValue::from(1)));
    assert_eq!(maybe_of::<JsValue>(&JsValue::NULL), None);
    assert_eq!(maybe_of::<JsValue>(&"v".into()), Some(JsValue::from("v")));
    assert_eq!(maybe_of::<String>(&JsValue::NULL), None);
    let init: Init<JsValue> = Init::new().unchecked_into();
    init.set_cf(&JsValue::NULL);
    assert_eq!(make_req("a", &init).unwrap().cf(), None);

    assert_eq!(
        later_maybe(&Promise::resolve(&JsValue::NULL))
            .await
            .unwrap(),
        None
    );
    assert_eq!(
        later_maybe(&Promise::resolve(&JsValue::from(0)))
            .await
            .unwrap(),
        Some(JsValue::from(0))
    );

    let values: Array = [JsValue::NULL, JsValue::UNDEFINED, JsValue::FALSE]
        .into_iter()
        .collect();
    let empties = call_with_each(
        &Function::from_fn(|v: Option<JsValue>| v.is_none()),
        &values,
    );
    assert_eq!(empties.to_vec(), [true, true, false]);
}

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono, js_name = bufferKinds)]
    fn buffer_kinds(bits: &[impl BufferSourceLike]) -> String;
    #[wasm_bindgen(experimental_generic_mono, js_name = keepBytes)]
    fn keep_bits(bits: &[impl BufferSourceLike]);
    #[wasm_bindgen(js_name = keptKinds)]
    fn kept_kinds() -> String;
}

#[wasm_bindgen_test]
fn buffer_source_slices() {
    let mut bytes = vec![1u8, 2, 3];
    assert_eq!(
        buffer_kinds(&[&bytes[..], &bytes[1..]]),
        "Uint8Array:1,2,3|Uint8Array:2,3"
    );
    assert_eq!(buffer_kinds(&[bytes.clone()]), "Uint8Array:1,2,3");
    let buf = ArrayBuffer::from(&bytes[..]);
    assert_eq!(
        buffer_kinds(&[&buf, &buf]),
        "ArrayBuffer:1,2,3|ArrayBuffer:1,2,3"
    );
    assert_eq!(
        buffer_kinds(&[Uint8Array::new_from_slice(&bytes)]),
        "Uint8Array:1,2,3"
    );
    assert_eq!(buffer_kinds(&[] as &[&[u8]]), "");
    keep_bits(&[&bytes[..]]);
    bytes[0] = 9;
    assert_eq!(kept_kinds(), "Uint8Array:1,2,3");
}
