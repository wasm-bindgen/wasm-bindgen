//! Under `js_sys_unstable_apis`, an element type is any type with an ABI,
//! crossing as itself at each instantiation.

use js_sys::*;
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
    fn optionals_of<T>() -> Vec<Option<T>>;
    #[wasm_bindgen(js_name = callWith)]
    fn call_unit(f: &Function<fn(String)>, a: &str);
    #[wasm_bindgen(js_name = throwingIterator)]
    fn throwing_iterator() -> Iterator<String>;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn describe_like(values: &Array<impl JsStringLike>) -> String;

    #[wasm_bindgen(experimental_generic_mono)]
    fn show<T>(value: Option<&T>) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_owned(r#type: impl JsStringLike) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = show)]
    fn show_view<T: TypedArray>(view: &T) -> String;

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
