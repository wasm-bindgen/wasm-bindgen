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

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn describe_like(values: &Array<impl JsStringLike>) -> String;
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
    assert_eq!(m.get(&"k".into()), Some(1.0));
    assert_eq!(m.get(&"j".into()), Some(2.0));
    assert_eq!(m.get(&"missing".into()), None);
    assert!(m.has(&"k".into()));
    assert_eq!(m.size(), 2);

    let keys: Vec<String> = m.keys().into_iter().map(Result::unwrap).collect();
    assert_eq!(keys, ["k", "j"]);
    let values: Vec<f64> = m.values().into_iter().map(Result::unwrap).collect();
    assert_eq!(values, [1.0, 2.0]);

    let mut seen = Vec::new();
    m.for_each(&mut |value, key| seen.push((key, value)));
    assert_eq!(seen, [("k".to_string(), 1.0), ("j".to_string(), 2.0)]);

    assert!(m.delete(&"k".into()));
    assert!(!m.has(&"k".into()));
}

#[wasm_bindgen_test]
fn set_of_rust_types() {
    let s: Set<String> = Set::new_from_items(&["a".to_string()]);
    s.add(&"b".into()).add(&"a".into());
    assert_eq!(s.size(), 2);
    assert!(s.has(&"b".into()));
    let values: Vec<String> = s.values().into_iter().map(Result::unwrap).collect();
    assert_eq!(values, ["a", "b"]);

    let mut seen = Vec::new();
    s.for_each(&mut |value: String| seen.push(value));
    assert_eq!(seen, ["a", "b"]);

    assert!(s.delete(&"a".into()));
    assert!(!s.has(&"a".into()));

    let numbers: Set<u32> = Set::new_typed();
    numbers.add(&1).add(&1).add(&2);
    assert_eq!(numbers.size(), 2);
}

#[wasm_bindgen_test]
async fn iterators_of_rust_types() {
    let values: Array<String> = strings();
    let it: Iterator<String> = values.values();
    let collected: Vec<String> = it.into_iter().map(Result::unwrap).collect();
    assert_eq!(collected, ["x", "y"]);

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
}
