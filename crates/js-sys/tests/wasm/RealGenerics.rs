//! Under `js_sys_unstable_apis`, an element type is any type with an ABI,
//! crossing as itself at each instantiation.

use js_sys::*;
use wasm_bindgen::convert::{FromWasmAbi, ReturnWasmAbi, VectorRefIntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsStringLike, MonoGeneric};
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

// `JsGeneric` carries the element ABI bounds, so generic code bounded by it
// alone can use the element APIs.
fn round_trip<T: JsGeneric + Clone + PartialEq + core::fmt::Debug>(x: T, y: T) {
    let a: Array<T> = Array::new_typed();
    a.push(&x);
    a.set(1, &y);
    assert_eq!(a.get(0), Some(x.clone()));
    assert!(a.includes(&y, 0));
    assert_eq!(a.to_vec(), [x.clone(), y.clone()]);

    let m: Map<T, T> = Map::new_typed();
    m.set(&x, &y);
    assert_eq!(m.get(&x), Some(y.clone()));
    assert!(m.delete(&x));

    let s: Set<T> = Set::new_typed();
    s.add(&x).add(&x);
    assert_eq!(s.size(), 1);
    assert!(s.has(&x));

    assert_eq!(Array::of(&[x.clone(), y]).length(), 2);
}

#[wasm_bindgen_test]
fn js_generic_bound_covers_element_apis() {
    round_trip(JsString::from("a"), JsString::from("b"));
    round_trip(JsValue::from(1), JsValue::from("b"));
}

// `MonoGeneric` covers Rust and JS element types alike, with nullable reads
// stating their `Option` bound.
fn mono_round_trip<T>(x: T, y: T)
where
    T: MonoGeneric + Clone + PartialEq + core::fmt::Debug,
    Option<T>: FromWasmAbi,
{
    let a: Array<T> = Array::new_typed();
    a.push(&x);
    a.push(&y);
    assert_eq!(a.get(1), Some(y.clone()));
    assert_eq!(a.index_of(&y, 0), 1);

    let m: Map<T, T> = Map::new_typed();
    m.set(&x, &y);
    assert_eq!(m.get(&x), Some(y.clone()));

    let s: Set<T> = Set::new_typed();
    s.add(&x).add(&y).add(&x);
    assert_eq!(s.size(), 2);
    assert!(s.delete(&y));
}

#[wasm_bindgen_test]
fn mono_generic_bound_covers_element_apis() {
    mono_round_trip(String::from("a"), String::from("b"));
    mono_round_trip(1u32, 2u32);
    mono_round_trip(JsString::from("a"), JsString::from("b"));
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

#[wasm_bindgen_test]
fn functions_of_rust_types() {
    let f: Function<fn(String, u32) -> String> =
        Function::new_with_args_typed("s, n", "return s.repeat(n)");
    assert_eq!(f.call2(&JsValue::NULL, &"ab".into(), &2).unwrap(), "abab");
    assert_eq!(f.call(&JsValue::NULL, (&"c".into(), &3)).unwrap(), "ccc");

    let bound: Function<fn(u32) -> String> = f.bind1(&JsValue::NULL, &"x".into());
    assert_eq!(bound.call1(&JsValue::NULL, &2).unwrap(), "xx");
    let bound: Function<fn() -> String> = f.bindn(&JsValue::NULL, (&"y".into(), &1));
    assert_eq!(bound.call0(&JsValue::NULL).unwrap(), "y");

    let args: Array<JsValue> = Array::of2(&"z".into(), &2.into());
    assert_eq!(f.apply(&JsValue::NULL, &args).unwrap(), "zz");
    assert_eq!(Reflect::apply(&f, &JsValue::NULL, &args).unwrap(), "zz");

    let len: Function<fn(JsString) -> f64> = Function::new_with_args_typed("s", "return s.length");
    assert_eq!(len.call1(&JsValue::NULL, &"abc".into()).unwrap(), 3.0);

    let mut calls = Vec::new();
    let mut record = |s: String, n: u32| -> bool {
        calls.push(s.repeat(n as usize));
        n > 1
    };
    let closure = ScopedClosure::borrow_mut(&mut record);
    let func: &Function<fn(String, u32) -> bool> = Function::closure_ref(&closure);
    assert!(func.call(&JsValue::NULL, (&"q".into(), &2)).unwrap());
    assert!(!func.call2(&JsValue::NULL, &"r".into(), &1).unwrap());
    drop(closure);
    assert_eq!(calls, ["qq", "r"]);

    let owned: Function<fn(f64) -> f64> =
        Function::from_closure(Closure::<dyn FnMut(f64) -> f64>::new(|x| x * 2.0));
    assert_eq!(owned.call1(&JsValue::NULL, &1.5).unwrap(), 3.0);
}

#[wasm_bindgen_test]
fn property_descriptors_of_rust_types() {
    let desc = PropertyDescriptor::new_value(&"v".to_string());
    assert_eq!(desc.get_value().as_deref(), Some("v"));
    desc.set_value(&"w".into());
    assert_eq!(desc.get_value().as_deref(), Some("w"));

    let desc: PropertyDescriptor<u32> = PropertyDescriptor::new();
    desc.set_get(Function::new_no_args_typed("return 7"));
    let obj: Object<u32> = Object::new_typed();
    Object::define_property_str(&obj, &"seven".into(), &desc).unwrap();
    assert_eq!(Reflect::get(&obj, "seven").unwrap(), Some(7));
    let getter = desc.get_get().unwrap();
    assert_eq!(getter.call0(&JsValue::NULL).unwrap(), 7);
}

#[wasm_bindgen_test]
async fn promise_new_of_rust_types() {
    let p: Promise<String> = Promise::new(&mut |resolve, _reject| {
        resolve.call1(&JsValue::NULL, &"done".into()).unwrap();
    });
    assert_eq!(p.await.unwrap(), "done");
}

fn call_generic<T: JsGeneric + PartialEq + core::fmt::Debug>(x: T) {
    let identity: Function<fn(T) -> T> = Function::new_with_args_typed("x", "return x");
    assert_eq!(identity.call1(&JsValue::NULL, &x).unwrap(), x);
    assert_eq!(identity.call(&JsValue::NULL, (&x,)).unwrap(), x);
    let bound: Function<fn() -> T> = identity.bind1(&JsValue::NULL, &x);
    assert_eq!(bound.call0(&JsValue::NULL).unwrap(), x);
}

fn call_mono<T: MonoGeneric + PartialEq + core::fmt::Debug>(x: T) {
    let identity: Function<fn(T) -> T> = Function::new_with_args_typed("x", "return x");
    assert_eq!(identity.call1(&JsValue::NULL, &x).unwrap(), x);
    assert_eq!(identity.call(&JsValue::NULL, (&x,)).unwrap(), x);
    let bound: Function<fn() -> T> = identity.bindn(&JsValue::NULL, (&x,));
    assert_eq!(bound.call0(&JsValue::NULL).unwrap(), x);
}

#[wasm_bindgen_test]
fn generic_bounds_cover_function_apis() {
    call_generic(JsString::from("a"));
    call_generic(JsValue::from(1));
    call_mono(String::from("a"));
    call_mono(2u32);
    call_mono(JsString::from("b"));
}

#[wasm_bindgen_test]
fn tuples_of_rust_types() {
    let t: ArrayTuple<(String, u32)> = ArrayTuple::new2(&"a".into(), &1);
    assert_eq!(t.get0(), "a");
    assert_eq!(t.get1(), 1);
    t.set0(&"b".into());
    t.set1(&2);
    assert_eq!(t.first(), "b");
    assert_eq!(t.last(), 2);
    assert_eq!(describe(&t), r#"Array:["string b","number 2"]"#);
    assert_eq!(t.into_tuple(), ("b".to_string(), 2));

    let t: ArrayTuple<(f64, String, JsString)> = (1.5, "x".to_string(), JsString::from("y")).into();
    assert_eq!(t.get0(), 1.5);
    assert_eq!(t.get1(), "x");
    assert_eq!(t.get2(), "y");
    let t: ArrayTuple<(bool, u32)> = Default::default();
    assert_eq!(t.into_tuple(), (false, 0));
}

#[wasm_bindgen_test]
fn entries_of_rust_types() {
    let entries = Array::of2(
        &ArrayTuple::new2(&"k".to_string(), &1.0),
        &ArrayTuple::new2(&"j".to_string(), &2.0),
    );
    let m: Map<String, f64> = Map::new_from_entries(&entries);
    assert_eq!(m.get(&"j".into()), Some(2.0));
    let pairs: Vec<(String, f64)> = m
        .entries()
        .into_iter()
        .map(|entry| entry.unwrap().into_tuple())
        .collect();
    assert_eq!(pairs, [("k".to_string(), 1.0), ("j".to_string(), 2.0)]);

    let a: Array<String> = strings();
    let pairs: Vec<(Number, String)> = a
        .entries()
        .into_iter()
        .map(|entry| entry.unwrap().into_tuple())
        .collect();
    assert_eq!(pairs[1].0, 1);
    assert_eq!(pairs[1].1, "y");

    let s: Set<u32> = Set::new_from_items(&[3, 4]);
    let pairs: Vec<(u32, u32)> = s
        .entries()
        .into_iter()
        .map(|entry| entry.unwrap().into_tuple())
        .collect();
    assert_eq!(pairs, [(3, 3), (4, 4)]);

    let o: Object<u32> =
        Object::from_entries(&Array::of1(&ArrayTuple::new2(&"n".to_string(), &5u32))).unwrap();
    let pairs: Vec<(JsString, u32)> = Object::entries(&o)
        .unwrap()
        .iter()
        .map(|entry| entry.into_tuple())
        .collect();
    assert_eq!(pairs, [(JsString::from("n"), 5)]);
}

#[wasm_bindgen_test]
async fn promise_tuples_of_rust_types() {
    let (s, n) = Promise::all_tuple((Promise::resolve("a"), Promise::resolve(2u32)))
        .await
        .unwrap()
        .into_tuple();
    assert_eq!(s, "a");
    assert_eq!(n, 2);

    let (s,) = Promise::all_settled_tuple((Promise::resolve(true),))
        .await
        .unwrap()
        .into_tuple();
    assert_eq!(s.get_value(), Some(true));
}

fn tuple_generic<T: JsGeneric + Clone + PartialEq + core::fmt::Debug>(x: T, y: T) {
    let t = ArrayTuple::new2(&x, &y);
    t.set0(&y);
    assert_eq!(t.into_tuple(), (y.clone(), y));
}

fn tuple_mono<T: MonoGeneric + Clone + PartialEq + core::fmt::Debug>(x: T, y: T) {
    let t: ArrayTuple<(T, T)> = (x.clone(), y.clone()).into();
    assert_eq!(t.get0(), x);
    t.set1(&x);
    assert_eq!(t.into_tuple(), (x.clone(), x));
}

#[wasm_bindgen_test]
fn generic_bounds_cover_tuple_apis() {
    tuple_generic(JsString::from("a"), JsString::from("b"));
    tuple_mono(String::from("a"), String::from("b"));
    tuple_mono(1u32, 2u32);
    tuple_mono(JsString::from("a"), JsString::from("b"));
}

fn string_array(values: &[&str]) -> Array<String> {
    values.iter().map(|s| s.to_string()).collect()
}

#[wasm_bindgen_test]
fn array_callbacks_of_rust_types() {
    let a = string_array(&["a", "bb", "ccc"]);
    assert!(a.every(&mut |s, _, _| !s.is_empty()));
    assert!(a.some(&mut |s| s == "bb"));
    assert_eq!(a.filter(&mut |s, _, _| s.len() > 1).to_vec(), ["bb", "ccc"]);
    assert_eq!(a.find(&mut |s, _, _| s.len() > 1).as_deref(), Some("bb"));
    assert_eq!(a.find(&mut |s, _, _| s.len() > 9), None);
    assert_eq!(a.find_index(&mut |s, _, _| s == "ccc"), 2);
    assert_eq!(a.find_last(&mut |s, _| s.len() < 3).as_deref(), Some("bb"));
    assert_eq!(a.find_last_index(&mut |s, _, _| s == "a"), 0);

    let lengths: Array<u32> = a.map(&mut |s, _, _| s.len() as u32);
    assert_eq!(lengths.to_vec(), [1, 2, 3]);
    assert_eq!(
        describe(&lengths),
        r#"Array:["number 1","number 2","number 3"]"#
    );
    assert_eq!(lengths.reduce(&mut |acc, n, _, _| acc + n, &10), 16);
    assert_eq!(
        a.reduce_right(&mut |acc: String, s, _, _| acc + &s, &String::new()),
        "cccbba"
    );
    let chars: Array<String> = a.flat_map(&mut |s, _, _| s.chars().map(String::from).collect());
    assert_eq!(chars.length(), 6);

    let mut seen = Vec::new();
    a.for_each(&mut |s, i, _| seen.push((i, s)));
    assert_eq!(seen[2], (2, "ccc".to_string()));

    let numbers: Array<f64> = vec![3.0, 1.0, 2.0].into();
    assert_eq!(
        numbers.to_sorted_by(&mut |x, y| (x - y) as i32).to_vec(),
        [1.0, 2.0, 3.0]
    );
    numbers.sort_by(&mut |x, y| (y - x) as i32);
    assert_eq!(numbers.to_vec(), [3.0, 2.0, 1.0]);

    let handles: Array<JsString> = ["x", "yy"].into_iter().map(JsString::from).collect();
    let lengths: Array<u32> = handles.map(&mut |s, _, _| s.length());
    assert_eq!(lengths.to_vec(), [1, 2]);
}

#[wasm_bindgen_test]
fn array_fallible_callbacks_of_rust_types() {
    let a = string_array(&["a", "bb", "ccc"]);
    assert!(a.try_every(&mut |s, _| Ok(!s.is_empty())).unwrap());
    assert!(a.try_some(&mut |s| Ok(s == "a")).unwrap());
    assert_eq!(
        a.try_filter(&mut |s, _| Ok(s.len() > 2)).unwrap().to_vec(),
        ["ccc"]
    );
    assert_eq!(
        a.try_find(&mut |s, _| Ok(s.len() == 2)).unwrap().as_deref(),
        Some("bb")
    );
    assert_eq!(a.try_find_index(&mut |s, _| Ok(s == "bb")).unwrap(), 1);
    assert_eq!(
        a.try_find_last(&mut |s, _| Ok(s.len() < 3))
            .unwrap()
            .as_deref(),
        Some("bb")
    );
    assert_eq!(a.try_find_last_index(&mut |s, _| Ok(s == "a")).unwrap(), 0);
    let lengths: Array<u32> = a.try_map(&mut |s, _| Ok(s.len() as u32)).unwrap();
    assert_eq!(lengths.to_vec(), [1, 2, 3]);
    assert_eq!(
        lengths
            .try_reduce(&mut |acc, n, _| Ok(acc + n), &0)
            .unwrap(),
        6
    );
    assert_eq!(
        a.try_reduce_right(&mut |acc, s, _| Ok(acc + s.as_str()), &String::new())
            .unwrap(),
        "cccbba"
    );
    let chars: Array<String> = a
        .try_flat_map(&mut |s, _| s.chars().map(String::from).collect())
        .unwrap();
    assert_eq!(chars.length(), 6);
    let mut total = 0;
    a.try_for_each(&mut |s, _| {
        total += s.len();
        Ok(())
    })
    .unwrap();
    assert_eq!(total, 6);
    assert_eq!(
        a.try_to_sorted_by(&mut |x, y| Ok(y.len() as i32 - x.len() as i32))
            .unwrap()
            .to_vec(),
        ["ccc", "bb", "a"]
    );
    a.try_sort_by(&mut |x, y| Ok(x.len() as i32 - y.len() as i32))
        .unwrap();
    assert_eq!(a.to_vec(), ["a", "bb", "ccc"]);

    assert!(a.try_every(&mut |_, _| Err(JsError::new("no"))).is_err());
    assert!(a
        .try_map::<String>(&mut |_, _| Err(JsError::new("no")))
        .is_err());
}

#[wasm_bindgen_test]
async fn array_from_maps_of_rust_types() {
    let a = string_array(&["a", "bb"]);
    let lengths: Array<u32> =
        Array::from_iterable_map(&a, &mut |s: String, i| Ok(s.len() as u32 + i)).unwrap();
    assert_eq!(lengths.to_vec(), [1, 3]);

    let map = Closure::new(|s: String, i: u32| Ok(format!("{s}{i}")));
    let p: Promise<Array<String>> = Array::from_async_map(&async_strings(), &map).unwrap();
    assert_eq!(p.await.unwrap().to_vec(), ["a0", "b1"]);
}

#[wasm_bindgen_test]
fn array_element_methods_of_rust_types() {
    let a = string_array(&["a", "b"]);
    assert_eq!(a.get_checked(1).as_deref(), Some("b"));
    assert_eq!(a.get_checked(5), None);
    a.set_ref(2, &"c".into());
    assert_eq!(a.push_many(&["d".to_string(), "e".to_string()]), 5);
    assert_eq!(a.unshift_many(&["z".to_string()]), 6);
    assert_eq!(a.shift_checked().as_deref(), Some("z"));
    assert_eq!(a.pop_checked().as_deref(), Some("e"));
    assert_eq!(a.with(0, &"A".into()).to_vec(), ["A", "b", "c", "d"]);
    assert_eq!(
        a.to_spliced(1, 2, &["x".to_string()]).to_vec(),
        ["a", "x", "d"]
    );
    assert_eq!(a.splice(0, 1, &"q".into()).to_vec(), ["a"]);
    assert_eq!(
        a.splice_many(1, 1, &["r".to_string(), "s".to_string()])
            .to_vec(),
        ["b"]
    );
    assert_eq!(a.to_vec(), ["q", "r", "s", "c", "d"]);
    assert_eq!(
        a.fill(&"f".into(), 1, 3).to_vec(),
        ["q", "f", "f", "c", "d"]
    );

    let numbers: Array<u32> = Array::new_typed();
    numbers.push_many(&[1, 2, 3]);
    assert_eq!(
        describe(&numbers),
        r#"Array:["number 1","number 2","number 3"]"#
    );
    assert_eq!(numbers.pop_checked(), Some(3));
}

fn callbacks_generic<T: JsGeneric + Clone + PartialEq + core::fmt::Debug>(x: T, y: T) {
    let a: Array<T> = Array::of2(&x, &y);
    assert_eq!(a.filter(&mut |v, _, _| v == y).to_vec(), [y.clone()]);
    assert_eq!(a.find(&mut |v, _, _| v == x), Some(x.clone()));
    let copied: Array<T> = a.map(&mut |v, _, _| v);
    assert_eq!(copied.to_vec(), [x.clone(), y.clone()]);
    assert_eq!(a.reduce(&mut |_, v, _, _| v, &x), y);
    let copied: Array<T> = a.try_map(&mut |v, _| Ok(v)).unwrap();
    assert_eq!(copied.to_vec(), [x.clone(), y.clone()]);
    assert_eq!(a.try_reduce(&mut |_, v, _| Ok(v), &x).unwrap(), y);
    let copied: Array<T> = Array::from_iterable_map(&a, &mut |v: T, _| Ok(v)).unwrap();
    assert_eq!(copied.to_vec(), [x.clone(), y.clone()]);
    a.splice_many(0, 0, &[y.clone()]);
    assert_eq!(a.pop_checked(), Some(y));
}

fn callbacks_mono<T>(x: T, y: T)
where
    T: MonoGeneric + VectorRefIntoWasmAbi + Clone + PartialEq + core::fmt::Debug,
    Option<T>: FromWasmAbi,
{
    let a: Array<T> = Array::of2(&x, &y);
    assert_eq!(
        a.filter(&mut |v, _, _| v == y).iter().collect::<Vec<_>>(),
        [y.clone()]
    );
    assert_eq!(a.find(&mut |v, _, _| v == x), Some(x.clone()));
    let copied: Array<T> = a.map(&mut |v, _, _| v);
    assert_eq!(copied.iter().collect::<Vec<_>>(), [x.clone(), y.clone()]);
    assert_eq!(a.reduce(&mut |_, v, _, _| v, &x), y);
    a.splice_many(0, 0, &[y.clone()]);
    assert_eq!(a.pop_checked(), Some(y));
}

#[wasm_bindgen_test]
fn generic_bounds_cover_array_callbacks() {
    callbacks_generic(JsString::from("a"), JsString::from("b"));
    callbacks_generic(JsValue::from(1), JsValue::from("b"));
    callbacks_mono(String::from("a"), String::from("b"));
    callbacks_mono(1u32, 2u32);
    callbacks_mono(JsString::from("a"), JsString::from("b"));
}

#[wasm_bindgen_test]
async fn promise_callbacks_of_rust_types() {
    let doubled = Closure::new(|n: u32| Ok(n * 2));
    let p: Promise<u32> = Promise::resolve(21u32).then(&doubled);
    assert_eq!(p.await.unwrap(), 42);

    let len = Closure::new(|s: String| Ok(s.len() as f64));
    let p: Promise<f64> = Promise::resolve("abc").then_map(&len);
    assert_eq!(p.await.unwrap(), 3.0);

    let on_ok = Closure::new(|s: String| Ok(s + "!"));
    let on_err = Closure::new(|_: JsValue| Ok("err".to_string()));
    let p: Promise<String> = Promise::resolve("ok").then_with_reject(&on_ok, &on_err);
    assert_eq!(p.await.unwrap(), "ok!");
    let rejected: Promise<String> = Promise::reject(&"bad".into());
    let p: Promise<String> = rejected.then_with_reject(&on_ok, &on_err);
    assert_eq!(p.await.unwrap(), "err");

    let recover = Closure::new(|reason: JsValue| Ok(reason.as_string().unwrap().len() as u32));
    let rejected: Promise<JsValue> = Promise::reject(&"bad".into());
    let p: Promise<u32> = rejected.catch(&recover);
    assert_eq!(p.await.unwrap(), 3);
    let rejected: Promise<String> = Promise::reject(&"four".into());
    let p: Promise<u32> = rejected.catch_map(&recover);
    assert_eq!(p.await.unwrap(), 4);

    let fail = Closure::new(|_: String| Err::<u32, _>(JsError::new("no")));
    assert!(Promise::resolve("x").then(&fail).await.is_err());

    let (state,) = Promise::all_settled_tuple((Promise::resolve(true),))
        .await
        .unwrap()
        .into_tuple();
    assert_eq!(state.get_value(), Some(true));
    assert_eq!(Result::from(state), Ok(true));
}

#[wasm_bindgen(module = "tests/wasm/RealGenerics.js")]
extern "C" {
    #[wasm_bindgen(js_name = echoGenerator)]
    fn echo_generator() -> Generator<String>;
    #[wasm_bindgen(js_name = echoAsyncGenerator)]
    fn echo_async_generator() -> AsyncGenerator<u32>;
}

#[wasm_bindgen_test]
async fn generators_of_rust_types() {
    let g = echo_generator();
    assert_eq!(g.next(&"ignored".into()).unwrap().value(), "start");
    assert_eq!(g.next(&"a".into()).unwrap().value(), "a");
    assert_eq!(g.next_iterator(&"b".into()).unwrap().value(), "b");
    let done = g.return_(&"end".into()).unwrap();
    assert!(done.done());
    assert_eq!(done.value(), "end");
    assert!(g.try_return(&"again".into()).unwrap().done());

    let g = echo_async_generator();
    assert_eq!(g.next(&0).unwrap().await.unwrap().value(), 0);
    assert_eq!(g.next(&7).unwrap().await.unwrap().value(), 7);
    let done = g.return_(&9).unwrap().await.unwrap();
    assert!(done.done());
    assert_eq!(done.value(), 9);
}

#[wasm_bindgen_test]
fn keyed_collections_of_rust_types() {
    let m: Map<String, u32> = Map::new_typed();
    m.set(&"a".into(), &1).set(&"b".into(), &2);
    assert_eq!(m.get_checked(&"b".into()), Some(2));
    assert_eq!(m.get_checked(&"z".into()), None);
    let mut total = 0;
    m.try_for_each(&mut |v, k: String| {
        total += v + k.len() as u32;
        Ok(())
    })
    .unwrap();
    assert_eq!(total, 5);

    let s: Set<f64> = Set::new_from_items(&[1.5, 2.5]);
    let mut sum = 0.0;
    s.try_for_each(&mut |v| {
        sum += v;
        Ok(())
    })
    .unwrap();
    assert_eq!(sum, 4.0);
    assert!(s.try_for_each(&mut |_| Err(JsError::new("no"))).is_err());

    let obj: Object<String> = Object::new_typed();
    assert!(Reflect::set_str(&obj, &"k".into(), &"v".into()).unwrap());
    assert_eq!(
        Reflect::get_str(&obj, &"k".into()).unwrap().as_deref(),
        Some("v")
    );
    assert_eq!(Reflect::get_str(&obj, &"missing".into()).unwrap(), None);

    let wm: WeakMap<Object, String> = WeakMap::new();
    let key = Object::new();
    wm.set(&key, &"v".into());
    assert_eq!(wm.get(&key).as_deref(), Some("v"));
    assert_eq!(wm.get_checked(&Object::new()), None);
    let wm: WeakMap<Object, u32> = WeakMap::new_typed();
    wm.set(&key, &3);
    assert_eq!(wm.get_checked(&key), Some(3));
    assert!(wm.has(&key));
}

async fn promise_generic<T>(x: T)
where
    T: JsGeneric + Promising<Resolution = T> + Clone + PartialEq + core::fmt::Debug,
{
    let identity = Closure::new(|v: T| Ok(v));
    let p: Promise<T> = Promise::new(&mut |resolve, _| {
        resolve.call1(&JsValue::NULL, &x).unwrap();
    });
    assert_eq!(p.then(&identity).await.unwrap(), x);
    let wm: WeakMap<Object, T> = WeakMap::new_typed();
    let key = Object::new();
    wm.set(&key, &x);
    assert_eq!(wm.get_checked(&key), Some(x));
}

// A `MonoGeneric` callback return states its `Result` ABI bound, as a mono
// callback's return crosses at its own ABI. `JsGeneric` implies it.
async fn promise_mono<T>(x: T)
where
    T: MonoGeneric + Promising<Resolution = T> + Clone + PartialEq + core::fmt::Debug,
    Option<T>: FromWasmAbi,
    Result<T, JsError>: ReturnWasmAbi,
{
    let identity = Closure::new(|v: T| Ok(v));
    let p: Promise<T> = Promise::resolve(x.clone()).then(&identity);
    assert_eq!(p.await.unwrap(), x);
    let m: Map<T, T> = Map::new_typed();
    m.set(&x, &x);
    assert_eq!(m.get_checked(&x), Some(x.clone()));
    let wm: WeakMap<Object, T> = WeakMap::new_typed();
    let key = Object::new();
    wm.set(&key, &x);
    assert_eq!(wm.get_checked(&key), Some(x));
}

#[wasm_bindgen_test]
async fn generic_bounds_cover_promise_and_keyed_apis() {
    promise_generic(JsString::from("a")).await;
    promise_mono(String::from("a")).await;
    promise_mono(1u32).await;
    promise_mono(JsString::from("b")).await;
}
