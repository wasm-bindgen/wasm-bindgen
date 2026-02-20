use js_sys::*;
use std::iter::FromIterator;
use wasm_bindgen::{convert::FromWasmAbi, prelude::*, JsGeneric};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

macro_rules! js_array {
    ($t:ty; $($e:expr),*) => ({
        let __x: Array<$t> = Array::new_typed();
        $(__x.push(&<$t>::from($e));)*
        __x
    })
}

macro_rules! array {
    ($t:ty; $($e:expr),*) => ({
        vec![$(<$t>::from($e)),*]
    })
}

fn to_rust<T: JsGeneric + FromWasmAbi>(arr: &Array<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(arr.length() as usize);
    arr.for_each(&mut |x, _, _| result.push(x));
    result
}

#[wasm_bindgen_test]
fn from_iter() {
    assert_eq!(
        to_rust(
            &vec![JsValue::from("a"), JsValue::from("b"), JsValue::from("c"),]
                .into_iter()
                .collect()
        ),
        vec!["a", "b", "c"],
    );

    assert_eq!(
        to_rust(
            &[JsValue::from("a"), JsValue::from("b"), JsValue::from("c")]
                .iter()
                .collect()
        ),
        vec!["a", "b", "c"],
    );

    let array = js_array![Number; 1u32, 2u32, 3u32];

    assert_eq!(
        to_rust(
            &vec![array.clone(),]
                .into_iter()
                .map(JsValue::from)
                .collect()
        ),
        vec![JsValue::from(array.clone())],
    );

    assert_eq!(
        to_rust(&[array.clone()].iter().map(JsValue::from).collect()),
        vec![JsValue::from(array)],
    );

    assert_eq!(
        to_rust(&vec![5, 10, 20,].into_iter().map(JsValue::from).collect()),
        vec![5, 10, 20],
    );

    assert_eq!(
        to_rust(&Array::from_iter(&[
            JsValue::from("a"),
            JsValue::from("b"),
            JsValue::from("c"),
        ])),
        vec!["a", "b", "c"],
    );

    let v = vec!["a", "b", "c"];

    assert_eq!(
        to_rust(&Array::from_iter(v.into_iter().map(JsValue::from))),
        vec!["a", "b", "c"],
    );
}

#[wasm_bindgen_test]
fn extend() {
    let mut array = array![JsString; "a", "b"];
    array.extend(vec![JsString::from("c"), JsString::from("d")]);
    assert_eq!(array, array![JsString; "a", "b", "c", "d"]);
}

#[wasm_bindgen_test]
fn to_vec() {
    let array = vec![JsValue::from("a"), JsValue::from("b"), JsValue::from("c")]
        .into_iter()
        .collect::<js_sys::Array>();

    assert_eq!(
        array.to_vec(),
        vec![JsValue::from("a"), JsValue::from("b"), JsValue::from("c")]
    );
}

#[wasm_bindgen_test]
fn iter() {
    let array = vec![JsValue::from("a"), JsValue::from("b"), JsValue::from("c")]
        .into_iter()
        .collect::<js_sys::Array>();

    assert_eq!(
        array.iter().collect::<Vec<JsValue>>(),
        vec![JsValue::from("a"), JsValue::from("b"), JsValue::from("c")]
    );

    let mut iter = array.iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(JsValue::from("a")));

    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some(JsValue::from("c")));

    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some(JsValue::from("b")));

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);

    let mut iter = array.iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(JsValue::from("a")));

    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(JsValue::from("b")));

    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(JsValue::from("c")));

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);

    let mut iter = array.iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(JsValue::from("c")));

    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some(JsValue::from("b")));

    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some(JsValue::from("a")));

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[wasm_bindgen_test]
fn new_with_length() {
    let array: Array<JsValue> = Array::new_with_length(5);
    assert_eq!(array.length(), 5);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(4), JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(4), None);
    #[cfg(not(js_sys_unstable_apis))]
    array.set(4, JsValue::from("a"));
    #[cfg(js_sys_unstable_apis)]
    array.set(4, &JsValue::from("a"));
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(4), "a");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(4), Some("a".into()));
    assert_eq!(array.length(), 5);
}

#[wasm_bindgen_test]
fn get() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(array.get(0), "a");
        assert_eq!(array.get(3), "n");
        assert_eq!(array.get(4), JsValue::undefined());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(array.get(0), Some("a".into()));
        assert_eq!(array.get(3), Some("n".into()));
        assert_eq!(array.get(4), None);
    }
}

#[wasm_bindgen_test]
fn set() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(0), "a");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(0), Some("a".into()));
    array.set_ref(0, &JsValue::from("b"));
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(0), "b");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(0), Some("b".into()));

    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(4), JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(4), None);
    assert_eq!(array.length(), 4);
    array.set_ref(4, &JsValue::from("d"));
    assert_eq!(array.length(), 5);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(4), "d");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(4), Some("d".into()));

    assert_eq!(array.get_checked(10), None);
    assert_eq!(array.length(), 5);
    array.set_ref(10, &JsValue::from("z"));
    assert_eq!(array.length(), 11);
    assert_eq!(array.get_checked(10), Some("z".into()));
    assert_eq!(array.get_checked(9), None);
}

#[wasm_bindgen_test]
fn delete() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(0), "a");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(0), Some("a".into()));
    array.delete(0);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array.get(0), JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array.get(0), None);
}

#[wasm_bindgen_test]
fn filter() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert!(
        array
            .filter(&mut |x: JsValue, _, _| { x.as_f64().is_some() })
            .length()
            == 0
    );

    let array = js_array![Number; 1, 2, 3, 4];
    assert_eq!(
        array.filter(&mut |x, _, _| x.as_f64().is_some()).length(),
        4
    );

    let array = js_array![JsValue; "a", 1, "b", 2];
    assert_eq!(
        array.filter(&mut |x, _, _| x.as_f64().is_some()).length(),
        2
    );
}

#[wasm_bindgen_test]
fn flat() {
    let array = js_array![
        JsValue;
        js_array![JsValue; "a", "b", "c"],
        "d",
        js_array![JsValue; "e", js_array![JsValue; "f", "g"]]
    ];

    assert_eq!(
        to_rust(&array.flat(1).slice(0, 5)),
        vec!["a", "b", "c", "d", "e"]
    );

    assert_eq!(array.flat(1).length(), 6);

    assert_eq!(
        to_rust(&array.flat(2)),
        vec!["a", "b", "c", "d", "e", "f", "g"]
    );
}

#[wasm_bindgen_test]
fn flat_map() {
    let array = js_array![JsValue; 1, 2, 3, 1];

    assert_eq!(
        to_rust(&array.flat_map::<JsValue>(
            &mut |val, _, _| match val.as_f64().map(|v| v as i32) {
                Some(1) => vec![JsString::from("x").into(), JsString::from("x").into()],
                Some(2) => vec![],
                Some(3) => vec![JsString::from("z").into()],
                _ => panic!("Unexpected conversion"),
            }
        )),
        vec!["x", "x", "z", "x", "x"]
    );
}

#[wasm_bindgen_test]
fn index_of() {
    let chars = js_array![JsString; "a", "c", "x", "n"];
    assert_eq!(chars.index_of(&"x".into(), 0), 2);
    assert_eq!(chars.index_of(&"z".into(), 0), -1);
    assert_eq!(chars.index_of(&"x".into(), -3), 2);
    assert_eq!(chars.index_of(&"z".into(), -2), -1);
}

#[wasm_bindgen_test]
fn is_array() {
    let arr: Array<JsValue> = Array::new();
    assert!(Array::is_array(&arr.into()));
    assert!(Array::is_array(&js_array![Number; 1].into()));
    assert!(!Array::is_array(&JsValue::null()));
    assert!(!Array::is_array(&JsValue::undefined()));
    assert!(!Array::is_array(&10.into()));
    assert!(!Array::is_array(&"x".into()));
    assert!(!Array::is_array(&true.into()));
    assert!(!Array::is_array(&false.into()));
}

#[wasm_bindgen_test]
fn sort() {
    let array = js_array![Number; 3, 1, 6, 2];
    let sorted = array.sort();
    assert_eq!(to_rust(&sorted), array![Number; 1, 2, 3, 6]);
}

#[wasm_bindgen_test]
#[allow(clippy::cmp_owned)]
fn some() {
    let array = js_array![JsValue; "z", 1, "y", 2];
    assert!(array.some(&mut |e| e == JsValue::from(2)));
    assert!(array.some(&mut |e| e == JsValue::from("y")));
    assert!(!array.some(&mut |e| e == JsValue::from("nope")));
}

#[wasm_bindgen_test]
fn last_index_of() {
    let characters = js_array![JsString; "a", "x", "c", "x", "n"];
    assert_eq!(characters.last_index_of(&"x".into(), 5), 3);
    assert_eq!(characters.last_index_of(&"z".into(), 5), -1);
    assert_eq!(characters.last_index_of(&"x".into(), 2), 1);
    assert_eq!(characters.last_index_of(&"x".into(), 0), -1);
}

#[wasm_bindgen_test]
fn join() {
    let characters = js_array![JsString; "a", "c", "x", "n"];
    assert_eq!(String::from(characters.join(", ")), "a, c, x, n");
    assert_eq!(String::from(characters.join("/")), "a/c/x/n");
}

#[wasm_bindgen_test]
fn slice() {
    let characters = js_array![JsValue; "a", "c", "x", "n", 1, "8"];
    let subset = characters.slice(1, 3);

    assert_eq!(to_rust(&subset), array![JsValue; "c", "x"]);
}

#[wasm_bindgen_test]
fn splice() {
    let characters = js_array![JsValue; "a", "c", "x", "n", 1, "8"];
    let removed = characters.splice(1, 3, &"b".into());

    assert_eq!(to_rust(&removed), array![JsValue; "c", "x", "n"]);
    assert_eq!(to_rust(&characters), array![JsValue; "a", "b", 1, "8"]);
}

#[wasm_bindgen_test]
fn fill() {
    let characters = js_array![JsValue; "a", "c", "x", "n", 1, "8"];
    let subset = characters.fill(&0.into(), 0, 3);

    assert_eq!(to_rust(&subset), array![JsValue; 0, 0, 0, "n", 1, "8"]);
}

#[wasm_bindgen_test]
fn copy_within() {
    let characters = js_array![Number; 8, 5, 4, 3, 1, 2];
    characters.copy_within(1, 4, 5);

    assert_eq!(to_rust(&characters)[1], Number::from(1));

    // if negatives were used
    characters.copy_within(-1, -3, -2);
    assert_eq!(to_rust(&characters)[5], Number::from(3));
}

#[wasm_bindgen_test]
fn of() {
    let a = JsValue::from("a");
    let b = JsValue::from("b");
    let c = JsValue::from("c");
    #[cfg(not(js_sys_unstable_apis))]
    let arr = {
        #[allow(deprecated)]
        Array::of3(&a, &b, &c)
    };
    #[cfg(js_sys_unstable_apis)]
    let arr = Array::of(&[a.clone(), b.clone(), c.clone()]);
    let vec = arr.to_vec();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], a);
    assert_eq!(vec[1], b);
    assert_eq!(vec[2], c);
    let items = vec![JsString::from("a"), JsString::from("b")];
    let arr = Array::<JsString>::of(&items);
    let vec = arr.to_vec();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "a");
    assert_eq!(vec[1], "b");
}

#[wasm_bindgen_test]
fn pop() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let item = characters.pop();
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(item, JsValue::from(2));
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(item, Some(JsValue::from(2)));
    assert_eq!(characters.length(), 5);
}

#[wasm_bindgen_test]
fn push() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let length = characters.push(&"a".into());
    assert_eq!(length, 7);
    assert_eq!(to_rust(&characters)[6], "a");
}

#[wasm_bindgen_test]
fn reverse() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let reversed = characters.reverse();
    assert_eq!(to_rust(&reversed), array![JsValue; 2, 1, 3, 4, 5, 8]);
}

#[wasm_bindgen_test]
fn shift() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let shifted_item = characters.shift();

    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(shifted_item, 8);
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(shifted_item, Some(JsValue::from(8)));
    assert_eq!(characters.length(), 5);
}

#[wasm_bindgen_test]
fn unshift() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let length = characters.unshift(&"abba".into());

    assert_eq!(length, 7);
    assert_eq!(to_rust(&characters)[0], "abba");
}

#[allow(deprecated)]
#[wasm_bindgen_test]
fn to_string() {
    let characters = js_array![Number; 8, 5, 4, 3, 1, 2];
    assert_eq!(String::from(characters.to_string()), "8,5,4,3,1,2");
}

#[wasm_bindgen_test]
fn includes() {
    let characters = js_array![Number; 8, 5, 4, 3, 1, 2];
    assert!(characters.includes(&2.into(), 0));
    assert!(!characters.includes(&9.into(), 0));
    assert!(!characters.includes(&3.into(), 4));
}

#[wasm_bindgen_test]
fn concat() {
    let arr1 = js_array![Number; 1, 2, 3];
    let arr2 = js_array![Number; 4, 5, 6];

    let new_array = arr1.concat(&arr2);
    assert_eq!(to_rust(&new_array), array![Number; 1, 2, 3, 4, 5, 6]);
}

#[wasm_bindgen_test]
fn length() {
    let characters = js_array![Number; 8, 5, 4, 3, 1, 2];
    assert_eq!(characters.length(), 6);
    assert_eq!(Array::<JsValue>::new().length(), 0);
}

#[wasm_bindgen_test]
fn every() {
    let even = js_array![Number; 2, 4, 6, 8];
    assert!(even.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
    let odd = js_array![Number; 1, 3, 5, 7];
    assert!(!odd.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
    let mixed = js_array![Number; 2, 3, 4, 5];
    assert!(!mixed.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
}

#[wasm_bindgen_test]
fn find() {
    let even = js_array![JsValue; 2, 4, 6, 8];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        even.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        2
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        even.find(ImmediateClosure::new_mut(
            &mut |x: JsValue, _: u32, _: Array<JsValue>| x.as_f64().unwrap() % 2.0 == 0.0
        )),
        Some(JsValue::from(2))
    );
    let odd = js_array![JsValue; 1, 3, 5, 7];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        odd.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        JsValue::undefined(),
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        odd.find(ImmediateClosure::new_mut(
            &mut |x: JsValue, _: u32, _: Array<JsValue>| x.as_f64().unwrap() % 2.0 == 0.0
        )),
        None
    );
    let mixed = js_array![JsValue; 3, 5, 7, 10];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        mixed.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        10
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        mixed.find(ImmediateClosure::new_mut(
            &mut |x: JsValue, _: u32, _: Array<JsValue>| x.as_f64().unwrap() % 2.0 == 0.0
        )),
        Some(JsValue::from(10))
    );
}

#[wasm_bindgen_test]
fn find_last() {
    let even = js_array![JsValue; 2, 4, 6, 8];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        even.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        8
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        even.find_last(&mut |x: JsValue, _| x.as_f64().unwrap() % 2.0 == 0.0),
        Some(JsValue::from(8))
    );
    let odd = js_array![JsValue; 1, 3, 5, 7];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        odd.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        JsValue::undefined(),
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        odd.find_last(&mut |x: JsValue, _| x.as_f64().unwrap() % 2.0 == 0.0),
        None,
    );
    let mixed = js_array![JsValue; 3, 5, 7, 10];
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        mixed.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 != 0.0),
        7
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        mixed.find_last(&mut |x: JsValue, _| x.as_f64().unwrap() % 2.0 != 0.0),
        Some(JsValue::from(7))
    );
}

#[wasm_bindgen_test]
fn map() {
    let numbers = js_array![Number; 1, 4, 9];
    let sqrt: Array<Number> = numbers.map(&mut |x, _, _| x.as_f64().unwrap().sqrt().into());
    assert_eq!(to_rust(&sqrt), array![Number; 1, 2, 3]);
}

#[wasm_bindgen_test]
fn reduce() {
    #[cfg(not(js_sys_unstable_apis))]
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
        &"".into(),
    );
    #[cfg(js_sys_unstable_apis)]
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce(
        ImmediateClosure::new_mut(
            &mut |ac: JsValue, cr: JsString, _: u32, _: Array<JsString>| {
                JsValue::from(format!(
                    "{}{}",
                    &ac.as_string().unwrap(),
                    &cr.as_string().unwrap()
                ))
            },
        ),
        &"".into(),
    );
    assert_eq!(arr, "01234");
}

#[wasm_bindgen_test]
fn reduce_right() {
    #[cfg(not(js_sys_unstable_apis))]
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce_right(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
        &"".into(),
    );
    #[cfg(js_sys_unstable_apis)]
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce_right(
        ImmediateClosure::new_mut(
            &mut |ac: JsValue, cr: JsString, _: u32, _: Array<JsString>| {
                JsValue::from(format!(
                    "{}{}",
                    &ac.as_string().unwrap(),
                    &cr.as_string().unwrap()
                ))
            },
        ),
        &"".into(),
    );
    assert_eq!(arr, "43210");
}

#[wasm_bindgen_test]
fn find_index() {
    let even = js_array![Number; 2, 4, 6, 8];
    assert_eq!(
        even.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        0
    );
    let odd = js_array![Number; 1, 3, 5, 7];
    assert_eq!(
        odd.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        -1
    );
    let mixed = js_array![Number; 3, 5, 7, 10];
    assert_eq!(
        mixed.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        3
    );
}

#[wasm_bindgen_test]
fn find_last_index() {
    let even = js_array![Number; 2, 4, 6, 8];
    assert_eq!(
        even.find_last_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        3
    );
    let odd = js_array![Number; 1, 3, 5, 7];
    assert_eq!(
        odd.find_last_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        -1
    );
    let mixed = js_array![Number; 3, 5, 7, 10];
    assert_eq!(
        mixed.find_last_index(&mut |e, _, _| e.as_f64().unwrap() % 2. != 0.),
        2
    );
}

#[wasm_bindgen_test]
fn to_locale_string() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let output = js_array![JsValue; 1, "a", Date::new(&"21 Dec 1997 14:12:00 UTC".into())]
            .to_locale_string(&"en".into(), &JsValue::undefined());
        assert!(!String::from(output).is_empty());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let output = js_array![JsValue; 1, "a", Date::new(&"21 Dec 1997 14:12:00 UTC".into())]
            .to_locale_string(&[JsString::from("en")], &Default::default());
        assert!(!String::from(output).is_empty());
    }
}

#[wasm_bindgen_test]
fn for_each() {
    fn sum_indices_of_evens(array: &Array<Number>) -> u32 {
        let mut res = 0;
        array.for_each(&mut |elem: Number, i, _| match elem.as_f64() {
            Some(val) if val % 2. == 0. => res += i,
            _ => {}
        });
        res
    }

    assert_eq!(
        sum_indices_of_evens(&js_array![Number; 2, 4, 6, 8]),
        1 + 2 + 3
    );
    assert_eq!(sum_indices_of_evens(&js_array![Number; 1, 3, 5, 7]), 0);
    assert_eq!(sum_indices_of_evens(&js_array![Number; 3, 5, 7, 10]), 3);
}

#[wasm_bindgen_test]
fn set_length() {
    let array = js_array![JsValue; 1, 2, 3, 4, 5];
    array.set_length(3);
    assert_eq!(
        array.iter().collect::<Vec<_>>(),
        [1.0, 2.0, 3.0].map(JsValue::from_f64)
    );

    array.set_length(7);
    assert_eq!(
        array.iter().collect::<Vec<_>>(),
        [1.0, 2.0, 3.0]
            .iter()
            .copied()
            .map(JsValue::from_f64)
            .chain([JsValue::UNDEFINED; 4])
            .collect::<Vec<_>>()
    );

    let mut calls = 0;
    array.for_each(&mut |_, _, _| calls += 1);
    // The later elements don't get filled with `undefined`, they get filled with
    // empty slots, which get skipped by `for_each`.
    assert_eq!(calls, 3);
}

#[wasm_bindgen_test]
fn array_inheritance() {
    let array: Array<JsValue> = Array::new();
    assert!(array.is_instance_of::<Array>());
    assert!(array.is_instance_of::<Object>());
    let _: &Object = array.as_ref();
}

#[wasm_bindgen(module = "tests/wasm/Array.js")]
extern "C" {
    fn populate_array(arr: JsValue, start: JsValue, len: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = createAsyncIterable)]
    fn create_async_iterable_str(values: &Array<JsString>) -> AsyncIterator<JsString>;

    #[wasm_bindgen(js_name = createAsyncIterable)]
    fn create_async_iterable_num(values: &Array<Number>) -> AsyncIterator<Number>;
}

fn test_array_view_mut_raw<ElemT: std::cmp::PartialEq + std::fmt::Debug, ArrT>(
    sut: unsafe fn(*mut ElemT, usize) -> ArrT,
    u8ToElem: fn(u8) -> ElemT,
    arrToJsValue: fn(ArrT) -> JsValue,
) {
    let start: u8 = 10;
    let len: usize = 32;
    let end: u8 = start + len as u8;
    let mut buffer: Vec<ElemT> = Vec::with_capacity(len);
    unsafe {
        let array: ArrT = sut(buffer.as_mut_ptr(), len);
        populate_array(
            arrToJsValue(array),
            JsValue::from(start),
            JsValue::from(len as u32),
        );
        buffer.set_len(len);
    }
    let expected: Vec<ElemT> = (start..end).map(u8ToElem).collect();
    assert_eq!(buffer, expected)
}

#[wasm_bindgen_test]
fn Int8Array_view_mut_raw() {
    fn u8Toi8_unsafe(x: u8) -> i8 {
        x as i8
    }
    test_array_view_mut_raw(
        js_sys::Int8Array::view_mut_raw,
        u8Toi8_unsafe,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Int16Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Int16Array::view_mut_raw, i16::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Int32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Int32Array::view_mut_raw, i32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn BigInt64Array_view_mut_raw() {
    test_array_view_mut_raw(
        js_sys::BigInt64Array::view_mut_raw,
        i64::from,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Uint8Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint8Array::view_mut_raw, u8::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Uint8ClampedArray_view_mut_raw() {
    test_array_view_mut_raw(
        js_sys::Uint8ClampedArray::view_mut_raw,
        u8::from,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Uint16Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint16Array::view_mut_raw, u16::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Uint32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint32Array::view_mut_raw, u32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn BigUint64Array_view_mut_raw() {
    test_array_view_mut_raw(
        js_sys::BigUint64Array::view_mut_raw,
        u64::from,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Float32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Float32Array::view_mut_raw, f32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Float64Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Float64Array::view_mut_raw, f64::from, JsValue::from);
}

#[wasm_bindgen_test]
async fn from_async() {
    // Check if Array.fromAsync exists (not available in all browsers)
    let array_constructor = Array::<JsValue>::new().constructor();
    if Reflect::get_str(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_none()
    {
        return;
    }

    let source = js_array![Number; 10, 20, 30];

    let async_iterable = create_async_iterable_num(&source);

    let promise = Array::from_async(&async_iterable).unwrap();

    let result = JsFuture::from(promise).await.unwrap();

    assert_eq!(result.length(), 3);
    assert_eq!(to_rust(&result), array![Number; 10, 20, 30]);
}

#[wasm_bindgen_test]
async fn from_async_map() {
    use wasm_bindgen::prelude::Closure;

    // Check if Array.fromAsync exists (not available in all browsers)
    let array_constructor = Array::<JsValue>::new().constructor();
    if Reflect::get_str(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_none()
    {
        return;
    }

    let source: Array<Number> = js_array![Number; 1, 2, 3, 4, 5].unchecked_into();

    let async_iterable = create_async_iterable_num(&source);

    let map_fn = Closure::new(|val: Number, _idx: u32| {
        let num = val.as_f64().unwrap();
        Ok(Number::from(num * 2.0))
    });

    let promise = Array::from_async_map(&async_iterable, &map_fn).unwrap();
    let result = JsFuture::from(promise).await.unwrap();

    assert_eq!(result.length(), 5);
    assert_eq!(to_rust(&result), array![Number; 2, 4, 6, 8, 10]);
}

#[wasm_bindgen_test]
async fn from_async_map_with_index() {
    use wasm_bindgen::prelude::Closure;

    // Check if Array.fromAsync exists (not available in all browsers)
    let array_constructor = Array::<JsValue>::new().constructor();
    if Reflect::get_str(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_none()
    {
        return;
    }

    let source = js_array![JsString; "a", "b", "c"];

    let async_iterable = create_async_iterable_str(&source);

    let map_fn = Closure::new(|val: JsString, idx: u32| {
        let s = val.as_string().unwrap();
        Ok(Promise::resolve(&JsString::from(format!("{}{}", s, idx))))
    });

    let promise = Array::from_async_map(&async_iterable, &map_fn).unwrap();
    let result = JsFuture::from(promise).await.unwrap();

    assert_eq!(result.length(), 3);
    assert_eq!(to_rust(&result), array![JsString; "a0", "b1", "c2"]);
}

#[wasm_bindgen_test]
fn covariance() {
    use wasm_bindgen::prelude::Upcast;

    // Helper function that accepts Array<JsValue>
    fn accepts_jsvalue_array(arr: Array<JsValue>) -> u32 {
        arr.length()
    }

    // Create an Array<Number>
    let number_array = js_array![Number; 1u32, 2u32, 3u32];

    // Test that we can pass Array<Number> where Array<JsValue> is expected
    // This works because Number is covariant to JsValue
    let length = accepts_jsvalue_array(number_array.upcast_into());
    assert_eq!(length, 3);

    // Also test with Array<JsString>
    let string_array = js_array![JsString; "a", "b", "c"];
    let length = accepts_jsvalue_array(string_array.upcast_into());
    assert_eq!(length, 3);
}

// Helper macros for handling Option returns in js_sys_unstable_apis
#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_get {
    ($arr:expr, $idx:expr) => {
        $arr.get($idx)
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_get {
    ($arr:expr, $idx:expr) => {
        $arr.get($idx).unwrap()
    };
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_at {
    ($arr:expr, $idx:expr) => {
        $arr.at($idx)
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_at {
    ($arr:expr, $idx:expr) => {
        $arr.at($idx).unwrap()
    };
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_pop {
    ($arr:expr) => {
        $arr.pop()
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_pop {
    ($arr:expr) => {
        $arr.pop().unwrap()
    };
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_shift {
    ($arr:expr) => {
        $arr.shift()
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_shift {
    ($arr:expr) => {
        $arr.shift().unwrap()
    };
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_find {
    ($arr:expr, $pred:expr) => {
        $arr.find($pred)
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_find {
    ($arr:expr, $pred:expr) => {
        $arr.find(ImmediateClosure::new_mut($pred)).unwrap()
    };
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! unwrap_find_last {
    ($arr:expr, $pred:expr) => {
        $arr.find_last($pred)
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! unwrap_find_last {
    ($arr:expr, $pred:expr) => {
        $arr.find_last($pred).unwrap()
    };
}

#[wasm_bindgen(module = "tests/wasm/Array.js")]
extern "C" {
    #[derive(Clone)]
    pub type TestItem;

    #[wasm_bindgen(constructor)]
    fn new(id: u32, name: &JsString) -> TestItem;

    #[wasm_bindgen(method, getter)]
    fn id(this: &TestItem) -> u32;

    #[wasm_bindgen(method, getter)]
    fn name(this: &TestItem) -> JsString;

    #[wasm_bindgen(method)]
    fn with_prefix(this: &TestItem, prefix: &JsString) -> TestItem;
}

#[wasm_bindgen(module = "tests/wasm/Array.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createTestItemArray")]
    fn create_test_item_array() -> Array<TestItem>;

    #[wasm_bindgen(js_name = "processTestItemArray")]
    fn process_test_item_array(arr: &Array<TestItem>) -> u32;

    #[wasm_bindgen(js_name = "checkArrayType")]
    fn check_array_type(arr: &Array<TestItem>) -> bool;

    #[wasm_bindgen(js_name = "createThrowingIterable")]
    fn create_throwing_iterable() -> JsValue;
}

#[wasm_bindgen_test]
fn test_array_new_typed() {
    let arr: Array<TestItem> = Array::new_typed();
    assert_eq!(arr.length(), 0);
}

#[wasm_bindgen_test]
fn test_array_new_with_length_typed() {
    let arr: Array<TestItem> = Array::new_with_length_typed(5);
    assert_eq!(arr.length(), 5);
}

#[wasm_bindgen_test]
fn test_array_of() {
    let arr: Array<TestItem> = Array::of(&[
        TestItem::new(4, &JsString::from("d")),
        TestItem::new(5, &JsString::from("e")),
    ]);
    assert_eq!(arr.length(), 2);
    assert_eq!(unwrap_get!(arr, 1).id(), 5);
}

#[wasm_bindgen_test]
fn test_array_get_set() {
    let arr: Array<TestItem> = Array::new_typed();
    let item = TestItem::new(1, &JsString::from("first"));

    arr.set_ref(0, &item);
    assert_eq!(arr.length(), 1);

    let retrieved: TestItem = unwrap_get!(arr, 0);
    assert_eq!(retrieved.id(), 1);
    assert_eq!(retrieved.name(), "first");
}

#[wasm_bindgen_test]
fn test_array_get_checked() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let item: Option<TestItem> = arr.get_checked(0);
    assert!(item.is_some());
    assert_eq!(item.unwrap().id(), 1);

    let missing: Option<TestItem> = arr.get_checked(99);
    assert!(missing.is_none());

    let sparse: Array<TestItem> = Array::new_with_length_typed(5);
    sparse.set_ref(2, &TestItem::new(42, &JsString::from("middle")));
    assert!(sparse.get_checked(0).is_none());
    assert!(sparse.get_checked(2).is_some());
}

#[wasm_bindgen_test]
fn test_array_at() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    let item: TestItem = unwrap_at!(arr, 1);
    assert_eq!(item.id(), 2);
    let last: TestItem = unwrap_at!(arr, -1);
    assert_eq!(last.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_delete() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    arr.delete(0);

    assert_eq!(arr.length(), 2);
    #[cfg(not(js_sys_unstable_apis))]
    assert!(arr.get(0).is_undefined());
    #[cfg(js_sys_unstable_apis)]
    assert!(arr.get(0).is_none());
}

#[wasm_bindgen_test]
fn test_array_push_pop() {
    let arr: Array<TestItem> = Array::new_typed();

    let len = arr.push(&TestItem::new(1, &JsString::from("first")));
    assert_eq!(len, 1);

    let len = arr.push(&TestItem::new(2, &JsString::from("second")));
    assert_eq!(len, 2);

    let more = [
        TestItem::new(3, &JsString::from("third")),
        TestItem::new(4, &JsString::from("fourth")),
    ];
    let len = arr.push_many(&more);
    assert_eq!(len, 4);

    let popped: TestItem = unwrap_pop!(arr);
    assert_eq!(popped.id(), 4);
    assert_eq!(arr.length(), 3);

    let popped: TestItem = unwrap_pop!(arr);
    assert_eq!(popped.id(), 3);

    let popped: TestItem = unwrap_pop!(arr);
    assert_eq!(popped.id(), 2);
    assert_eq!(arr.length(), 1);

    let popped: TestItem = unwrap_pop!(arr);
    assert_eq!(popped.id(), 1);
    assert_eq!(arr.length(), 0);
    #[cfg(not(js_sys_unstable_apis))]
    assert!(arr.pop().is_undefined());
    #[cfg(js_sys_unstable_apis)]
    assert!(arr.pop().is_none());
    assert!(arr.pop_checked().is_none());
}

#[wasm_bindgen_test]
fn test_array_shift() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("first")));
    arr.push(&TestItem::new(2, &JsString::from("second")));

    let shifted: TestItem = unwrap_shift!(arr);
    assert_eq!(shifted.id(), 1);
    assert_eq!(arr.length(), 1);

    let remaining: TestItem = unwrap_get!(arr, 0);
    assert_eq!(remaining.id(), 2);
    arr.shift();
    assert!(arr.shift_checked().is_none());

    let items = [
        TestItem::new(1, &JsString::from("a")),
        TestItem::new(2, &JsString::from("b")),
    ];
    let len = arr.unshift_many(&items);
    assert_eq!(len, 2);
    assert_eq!(unwrap_get!(arr, 0).id(), 1);
    assert_eq!(unwrap_get!(arr, 1).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_concat() {
    let arr1: Array<TestItem> = Array::new_typed();
    arr1.push(&TestItem::new(1, &JsString::from("a")));

    let arr2: Array<TestItem> = Array::new_typed();
    arr2.push(&TestItem::new(2, &JsString::from("b")));

    let combined = arr1.concat(&arr2);
    assert_eq!(combined.length(), 2);

    let first: TestItem = unwrap_get!(combined, 0);
    let second: TestItem = unwrap_get!(combined, 1);
    assert_eq!(first.id(), 1);
    assert_eq!(second.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_reverse() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let reversed = arr.reverse();

    let first: TestItem = unwrap_get!(reversed, 0);
    let last: TestItem = unwrap_get!(reversed, 2);
    assert_eq!(first.id(), 3);
    assert_eq!(last.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_copy_within() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));
    let result = arr.copy_within(2, 0, 2);

    let at2: TestItem = unwrap_get!(result, 2);
    let at3: TestItem = unwrap_get!(result, 3);
    assert_eq!(at2.id(), 1);
    assert_eq!(at3.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_splice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let items = [
        TestItem::new(10, &JsString::from("x")),
        TestItem::new(11, &JsString::from("y")),
    ];
    let removed = arr.splice_many(0, 1, &items);
    assert_eq!(unwrap_get!(removed, 0).id(), 1);
    assert_eq!(unwrap_get!(arr, 0).id(), 10);
    assert_eq!(unwrap_get!(arr, 1).id(), 11);

    let spliced = arr.to_spliced(0, 2, &[]);
    assert_eq!(spliced.length(), 2);

    let items = [TestItem::new(99, &JsString::from("new"))];
    let spliced = arr.to_spliced(0, 1, &items);
    assert_eq!(unwrap_get!(spliced, 0).id(), 99);

    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    arr.splice_many(1, 1, &[]);
}

#[wasm_bindgen_test]
fn test_array_iter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let ids: Vec<u32> = arr
        .iter()
        .map(|item| {
            let t: TestItem = item;
            t.id()
        })
        .collect();

    assert_eq!(ids, vec![1, 2, 3]);
}

#[wasm_bindgen_test]
fn test_array_into_iter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("x")));
    arr.push(&TestItem::new(20, &JsString::from("y")));

    let mut sum = 0;
    for item in arr {
        let t: TestItem = item;
        sum += t.id();
    }
    assert_eq!(sum, 30);
}

#[wasm_bindgen_test]
fn test_array_to_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let vec: Vec<TestItem> = arr.to_vec();
    assert_eq!(vec.len(), 2);

    let first = vec[0].clone();
    let second = vec[1].clone();
    assert_eq!(first.id(), 1);
    assert_eq!(second.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_from_iter() {
    let items = vec![
        TestItem::new(1, &JsString::from("a")),
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];

    let arr: Array<TestItem> = items.iter().map(|i| i).collect();

    assert_eq!(arr.length(), 3);
    let first: TestItem = unwrap_get!(arr, 0);
    assert_eq!(first.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_extend() {
    let mut arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let more = vec![
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];
    arr.extend(more);

    assert_eq!(arr.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_find() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("apple")));
    arr.push(&TestItem::new(2, &JsString::from("banana")));
    arr.push(&TestItem::new(3, &JsString::from("cherry")));

    let found: TestItem = unwrap_find!(arr, &mut |val: TestItem, _, _| val.id() == 2);
    assert_eq!(found.name(), "banana");
}

#[wasm_bindgen_test]
fn test_array_find_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let idx = arr.find_index(&mut |val: TestItem, _, _| val.id() == 2);
    assert_eq!(idx, 1);

    let not_found = arr.find_index(&mut |val: TestItem, _, _| val.id() == 99);
    assert_eq!(not_found, -1);
}

#[wasm_bindgen_test]
fn test_array_filter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let evens: Array<TestItem> = arr.filter(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert_eq!(evens.length(), 2);
    assert_eq!(unwrap_get!(evens, 0).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_every() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(4, &JsString::from("b")));
    arr.push(&TestItem::new(6, &JsString::from("c")));

    let all_even = arr.every(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert!(all_even);

    arr.push(&TestItem::new(7, &JsString::from("d")));

    let still_all_even = arr.every(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert!(!still_all_even);
}

#[wasm_bindgen_test]
fn test_array_from_js() {
    let arr = create_test_item_array();

    assert_eq!(arr.length(), 3);

    let first: TestItem = unwrap_get!(arr, 0);
    assert_eq!(first.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_try_from_catches_error() {
    use wasm_bindgen::JsCast;

    let throwing_iterable = create_throwing_iterable();
    let result: Result<Array, JsValue> =
        Array::from_iterable(throwing_iterable.unchecked_ref::<js_sys::Iterator>());

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.is_instance_of::<js_sys::Error>());
    let error: &js_sys::Error = err.unchecked_ref();
    assert_eq!(error.message(), "iterator error");
}

#[wasm_bindgen_test]
fn test_array_to_js() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("x")));
    arr.push(&TestItem::new(20, &JsString::from("y")));

    let sum = process_test_item_array(&arr);
    assert_eq!(sum, 30);
}

#[wasm_bindgen_test]
fn test_array_type_preserved() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    assert!(check_array_type(&arr));
}

#[wasm_bindgen]
pub fn rust_create_test_item_array() -> Array<TestItem> {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(100, &JsString::from("rust1")));
    arr.push(&TestItem::new(200, &JsString::from("rust2")));
    arr
}

#[wasm_bindgen_test]
fn test_rust_export_Array() {
    let arr = rust_create_test_item_array();
    assert_eq!(arr.length(), 2);

    let first: TestItem = unwrap_get!(arr, 0);
    assert_eq!(first.id(), 100);
    assert_eq!(first.name(), "rust1");
}

#[wasm_bindgen_test]
fn test_array_slice_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Vec<TestItem> = arr.slice(1, 3).to_vec();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);

    assert_eq!(arr.length(), 4);
}

#[wasm_bindgen_test]
fn test_array_slice_box() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Box<[TestItem]> = arr.slice(1, 3).to_vec().into_boxed_slice();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);

    assert_eq!(arr.length(), 4);
}

#[wasm_bindgen_test]
fn test_array_to_reversed() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let reversed: Array<TestItem> = arr.to_reversed();
    assert_eq!(reversed.length(), 3);
    assert_eq!(unwrap_get!(reversed, 0).id(), 3);
    assert_eq!(unwrap_get!(reversed, 2).id(), 1);
    assert_eq!(unwrap_get!(arr, 0).id(), 1);
}

#[wasm_bindgen_test]
fn test_array_to_sorted() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let sorted: Array<TestItem> = arr.to_sorted();
    assert_eq!(sorted.length(), 3);
    assert_eq!(unwrap_get!(arr, 0).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_with() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let new_arr = arr.with(1, &TestItem::new(99, &JsString::from("replaced")));
    assert_eq!(unwrap_get!(new_arr, 1).id(), 99);
    assert_eq!(unwrap_get!(arr, 1).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_slice_from() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Array<TestItem> = arr.slice_from(2);
    assert_eq!(sliced.length(), 2);
    assert_eq!(unwrap_get!(sliced, 0).id(), 3);
    assert_eq!(unwrap_get!(sliced, 1).id(), 4);
}

#[wasm_bindgen_test]
fn test_array_slice_from_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let sliced: Vec<TestItem> = arr.slice_from(1).to_vec();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);
}

#[wasm_bindgen_test]
fn test_array_slice_from_boxed_slice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let sliced: Box<[TestItem]> = arr.slice_from(1).to_vec().into_boxed_slice();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
}

#[wasm_bindgen_test]
fn test_array_to_boxed_slice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let boxed: Box<[TestItem]> = arr.to_vec().into_boxed_slice();
    assert_eq!(boxed.len(), 2);
    assert_eq!(boxed[0].id(), 1);
    assert_eq!(boxed[1].id(), 2);
}

#[wasm_bindgen_test]
fn test_array_fill() {
    let arr: Array<TestItem> = Array::new_with_length_typed(3);
    let filler = TestItem::new(42, &JsString::from("fill"));
    arr.fill(&filler, 0, 3);
    assert_eq!(unwrap_get!(arr, 0).id(), 42);
    assert_eq!(unwrap_get!(arr, 1).id(), 42);
    assert_eq!(unwrap_get!(arr, 2).id(), 42);
}

#[wasm_bindgen_test]
fn test_array_includes() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);

    assert!(arr.includes(&item1, 0));
    assert!(arr.includes(&item2, 0));
}

#[wasm_bindgen_test]
fn test_array_index_of() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);
    arr.push(&item1);

    assert_eq!(arr.index_of(&item1, 0), 0);
    assert_eq!(arr.index_of(&item1, 1), 2);
    assert_eq!(arr.index_of(&item2, 0), 1);
}

#[wasm_bindgen_test]
fn test_array_last_index_of() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);
    arr.push(&item1);

    assert_eq!(arr.last_index_of(&item1, 3), 2);
    assert_eq!(arr.last_index_of(&item1, 1), 0);
}

#[wasm_bindgen_test]
fn test_array_find_last() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    #[cfg(not(js_sys_unstable_apis))]
    let found: TestItem = unwrap_find_last!(arr, &mut |item: TestItem, _, _| item.id() > 1);
    #[cfg(js_sys_unstable_apis)]
    let found: TestItem = unwrap_find_last!(arr, &mut |item: TestItem, _| item.id() > 1);
    assert_eq!(found.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_find_last_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let idx = arr.find_last_index(&mut |item: TestItem, _, _| item.id() > 1);
    assert_eq!(idx, 2);
}

#[wasm_bindgen_test]
fn test_array_join() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("a"));
    arr.push(&JsString::from("b"));
    arr.push(&JsString::from("c"));

    let joined = arr.join(",");
    assert_eq!(joined, "a,b,c");
}

#[wasm_bindgen_test]
fn test_array_set_length() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    assert_eq!(arr.length(), 3);

    arr.set_length(2);
    assert_eq!(arr.length(), 2);
}

#[wasm_bindgen_test]
fn test_array_unshift() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let new_len = arr.unshift(&TestItem::new(1, &JsString::from("a")));
    assert_eq!(new_len, 3);
    assert_eq!(unwrap_get!(arr, 0).id(), 1);
}

#[wasm_bindgen_test]
fn test_array_flat() {
    let inner1: Array<JsString> = Array::new_typed();
    inner1.push(&JsString::from("a"));
    inner1.push(&JsString::from("b"));

    let inner2: Array<JsString> = Array::new_typed();
    inner2.push(&JsString::from("c"));

    let outer: Array = Array::new_typed();
    outer.push(&inner1.into());
    outer.push(&inner2.into());

    let flat: Array = outer.flat(1);
    assert_eq!(flat.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_spliced() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let spliced: Array<TestItem> = arr.to_spliced(1, 1, &[]);
    assert_eq!(spliced.length(), 2);
    assert_eq!(unwrap_get!(spliced, 0).id(), 1);
    assert_eq!(unwrap_get!(spliced, 1).id(), 3);
    assert_eq!(arr.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_string() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("x"));
    arr.push(&JsString::from("y"));

    let s = arr.to_string();
    assert_eq!(s, "x,y");
}

#[wasm_bindgen_test]
fn test_array_push_many() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let items = [
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];
    let new_len = arr.push_many(&items);
    assert_eq!(new_len, 3);
    assert_eq!(arr.length(), 3);
    assert_eq!(unwrap_get!(arr, 1).id(), 2);
    assert_eq!(unwrap_get!(arr, 2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_sort_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let sorted = arr.sort_by(&mut |a, b| (a.id() as i32) - (b.id() as i32));
    assert_eq!(unwrap_get!(sorted, 0).id(), 1);
    assert_eq!(unwrap_get!(sorted, 1).id(), 2);
    assert_eq!(unwrap_get!(sorted, 2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_sorted_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    let sorted: Array<TestItem> = arr.to_sorted_by(&mut |a, b| (b.id() as i32) - (a.id() as i32));
    assert_eq!(unwrap_get!(sorted, 0).id(), 3);
    assert_eq!(unwrap_get!(sorted, 1).id(), 2);
    assert_eq!(unwrap_get!(sorted, 2).id(), 1);
    assert_eq!(unwrap_get!(arr, 0).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_splice_many() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let insert = [
        TestItem::new(10, &JsString::from("x")),
        TestItem::new(11, &JsString::from("y")),
    ];
    let removed = arr.splice_many(1, 1, &insert);
    assert_eq!(removed.length(), 1);
    assert_eq!(unwrap_get!(removed, 0).id(), 2);
    assert_eq!(arr.length(), 4);
    assert_eq!(unwrap_get!(arr, 1).id(), 10);
    assert_eq!(unwrap_get!(arr, 2).id(), 11);
}

#[wasm_bindgen_test]
fn test_array_entries_typed() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("a")));
    arr.push(&TestItem::new(20, &JsString::from("b")));
    arr.push(&TestItem::new(30, &JsString::from("c")));

    let entries = arr.entries_typed();
    let mut count = 0u32;
    for result in entries.into_iter() {
        let tuple: js_sys::ArrayTuple<(Number, TestItem)> = result.unwrap();
        let arr_ref: &js_sys::Array = wasm_bindgen::JsCast::unchecked_ref(&tuple);
        assert_eq!(arr_ref.get_unchecked(0), count);
        count += 1;
    }
    assert_eq!(count, 3);
}

#[wasm_bindgen_test]
fn test_from_iterable_map() {
    let source: Array<Number> = Array::of(&[Number::from(1), Number::from(2), Number::from(3)]);

    let result: Array<Number> = Array::from_iterable_map(
        &source,
        ImmediateClosure::new_mut_aborting(&mut |val, _idx| Ok(Number::from(val.value_of() * 2.0))),
    )
    .unwrap();

    assert_eq!(result.length(), 3);
    assert_eq!(unwrap_get!(result, 0).value_of(), 2.0);
    assert_eq!(unwrap_get!(result, 1).value_of(), 4.0);
    assert_eq!(unwrap_get!(result, 2).value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_every() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(2, &JsString::from("a")));
    arr.push(&TestItem::new(4, &JsString::from("b")));
    arr.push(&TestItem::new(6, &JsString::from("c")));

    let result = arr.try_every(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() % 2 == 0)
    }));
    assert!(result.is_ok());
    assert!(result.unwrap());

    arr.push(&TestItem::new(7, &JsString::from("d")));
    let result = arr.try_every(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() % 2 == 0)
    }));
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[wasm_bindgen_test]
fn test_array_try_every_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_every(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsError::new("error at 2"))
        } else {
            Ok(true)
        }
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_filter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let result = arr.try_filter(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() % 2 == 0)
    }));
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert_eq!(filtered.length(), 2);
    assert_eq!(unwrap_get!(filtered, 0).id(), 2);
    assert_eq!(unwrap_get!(filtered, 1).id(), 4);
}

#[wasm_bindgen_test]
fn test_array_try_filter_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_filter(ImmediateClosure::new_mut_assert_unwind_safe(
        &mut |val, _| {
            if val.id() == 2 {
                Err(JsError::new("filter error"))
            } else {
                Ok(true)
            }
        },
    ));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("apple")));
    arr.push(&TestItem::new(2, &JsString::from("banana")));
    arr.push(&TestItem::new(3, &JsString::from("cherry")));

    let result = arr.try_find(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() == 2)
    }));
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "banana");

    let result = arr.try_find(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() == 99)
    }));
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[wasm_bindgen_test]
fn test_array_try_find_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find(ImmediateClosure::new_mut(&mut |_val: TestItem, _| {
        Err(JsError::new("find error"))
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_index(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() == 2)
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = arr.try_find_index(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() == 99)
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[wasm_bindgen_test]
fn test_array_try_find_index_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find_index(ImmediateClosure::new_mut(&mut |_val: TestItem, _| {
        Err(JsError::new("find_index error"))
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_last() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_last(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() > 1)
    }));
    assert!(result.is_ok());
    let found = result.unwrap().unwrap();
    assert_eq!(found.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_try_find_last_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find_last(ImmediateClosure::new_mut(&mut |_val: TestItem, _| {
        Err(JsError::new("find_last error"))
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_last_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_last_index(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        Ok(val.id() > 1)
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[wasm_bindgen_test]
fn test_array_try_find_last_index_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find_last_index(ImmediateClosure::new_mut(&mut |_val: TestItem, _| {
        Err(JsError::new("find_last_index error"))
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_for_each() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let mut sum = 0;
    let result = arr.try_for_each(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        sum += val.id();
        Ok(())
    }));
    assert!(result.is_ok());
    assert_eq!(sum, 6);
}

#[wasm_bindgen_test]
fn test_array_try_for_each_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_for_each(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsError::new("for_each error"))
        } else {
            Ok(())
        }
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_map() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result: Result<Array<TestItem>, JsValue> =
        arr.try_map(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
            Ok(val.with_prefix(&JsString::from("pre_")))
        }));
    assert!(result.is_ok());
    let mapped = result.unwrap();
    assert_eq!(mapped.length(), 3);
    assert_eq!(unwrap_get!(mapped, 0).name(), "pre_a");
}

#[wasm_bindgen_test]
fn test_array_try_map_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result: Result<Array<TestItem>, JsValue> =
        arr.try_map(ImmediateClosure::new_mut(&mut |val: TestItem, _| {
            if val.id() == 2 {
                Err(JsError::new("map error"))
            } else {
                Ok(val)
            }
        }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_reduce() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));
    arr.push(&Number::from(3));

    let initial = Number::from(0);
    let result = arr.try_reduce(
        ImmediateClosure::new_mut(&mut |acc: Number, val: Number, _| {
            Ok(Number::from(acc.value_of() + val.value_of()))
        }),
        &initial,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_reduce_error() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));

    let initial = Number::from(0);
    let result = arr.try_reduce(
        ImmediateClosure::new_mut(&mut |_acc: Number, val: Number, _| {
            if val.value_of() == 2.0 {
                Err(JsError::new("reduce error"))
            } else {
                Ok(val)
            }
        }),
        &initial,
    );
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_reduce_right() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));
    arr.push(&Number::from(3));

    let initial = Number::from(0);
    let result = arr.try_reduce_right(
        ImmediateClosure::new_mut(&mut |acc: JsValue, val: Number, _| {
            let acc_num: Number = acc.unchecked_into();
            Ok(Number::from(acc_num.value_of() + val.value_of()))
        }),
        &initial,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_reduce_right_error() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));

    let initial = Number::from(0);
    let result = arr.try_reduce_right(
        ImmediateClosure::new_mut(&mut |_acc: JsValue, val: Number, _| {
            if val.value_of() == 1.0 {
                Err(JsError::new("reduce_right error"))
            } else {
                Ok(val)
            }
        }),
        &initial,
    );
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_some() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_some(ImmediateClosure::new_mut(&mut |val: TestItem| {
        Ok(val.id() == 2)
    }));
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = arr.try_some(ImmediateClosure::new_mut(&mut |val: TestItem| {
        Ok(val.id() == 99)
    }));
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[wasm_bindgen_test]
fn test_array_try_some_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_some(ImmediateClosure::new_mut(&mut |_val: TestItem| {
        Err(JsError::new("some error"))
    }));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_sort_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_sort_by(ImmediateClosure::new_mut(
        &mut |a: TestItem, b: TestItem| Ok((a.id() as i32) - (b.id() as i32)),
    ));
    assert!(result.is_ok());
    let sorted = result.unwrap();
    assert_eq!(unwrap_get!(sorted, 0).id(), 1);
    assert_eq!(unwrap_get!(sorted, 1).id(), 2);
    assert_eq!(unwrap_get!(sorted, 2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_try_sort_by_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_sort_by(ImmediateClosure::new_mut(
        &mut |_a: TestItem, _b: TestItem| Err(JsError::new("sort error")),
    ));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_to_sorted_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_to_sorted_by(ImmediateClosure::new_mut(
        &mut |a: TestItem, b: TestItem| Ok((b.id() as i32) - (a.id() as i32)),
    ));
    assert!(result.is_ok());
    let sorted = result.unwrap();
    assert_eq!(unwrap_get!(sorted, 0).id(), 3);
    assert_eq!(unwrap_get!(sorted, 1).id(), 2);
    assert_eq!(unwrap_get!(sorted, 2).id(), 1);
    assert_eq!(unwrap_get!(arr, 0).id(), 3); // Original array unchanged
}

#[wasm_bindgen_test]
fn test_array_try_to_sorted_by_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_to_sorted_by(ImmediateClosure::new_mut(
        &mut |_a: TestItem, _b: TestItem| Err(JsError::new("to_sorted error")),
    ));
    assert!(result.is_err());
}

// Exported functions with standard js-sys types
#[wasm_bindgen]
pub fn rust_create_number_array() -> Array<Number> {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1.0));
    arr.push(&Number::from(2.0));
    arr.push(&Number::from(3.0));
    arr
}

#[wasm_bindgen]
pub fn rust_create_string_array() -> Array<JsString> {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("a"));
    arr.push(&JsString::from("b"));
    arr.push(&JsString::from("c"));
    arr
}

#[wasm_bindgen]
pub fn rust_sum_number_array(arr: Array<Number>) -> f64 {
    let mut sum = 0.0;
    for i in 0..arr.length() {
        #[cfg(not(js_sys_unstable_apis))]
        let num: Number = arr.get(i);
        #[cfg(js_sys_unstable_apis)]
        let num: Number = arr.get(i).unwrap();
        sum += num.value_of();
    }
    sum
}

#[wasm_bindgen]
pub fn rust_concat_string_array(arr: Array<JsString>, separator: &str) -> String {
    let mut result = String::new();
    for i in 0..arr.length() {
        if i > 0 {
            result.push_str(separator);
        }
        #[cfg(not(js_sys_unstable_apis))]
        let s: JsString = arr.get(i);
        #[cfg(js_sys_unstable_apis)]
        let s: JsString = arr.get(i).unwrap();
        result.push_str(&String::from(s));
    }
    result
}

#[wasm_bindgen_test]
fn rust_export_number_array() {
    let arr = rust_create_number_array();
    assert_eq!(arr.length(), 3);
    assert_eq!(unwrap_get!(arr, 0).value_of(), 1.0);
    assert_eq!(unwrap_get!(arr, 1).value_of(), 2.0);
    assert_eq!(unwrap_get!(arr, 2).value_of(), 3.0);
}

#[wasm_bindgen_test]
fn rust_export_string_array() {
    let arr = rust_create_string_array();
    assert_eq!(arr.length(), 3);
    assert_eq!(unwrap_get!(arr, 0), JsString::from("a"));
    assert_eq!(unwrap_get!(arr, 1), JsString::from("b"));
    assert_eq!(unwrap_get!(arr, 2), JsString::from("c"));
}

#[wasm_bindgen_test]
fn rust_export_process_number_array() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(10.0));
    arr.push(&Number::from(20.0));
    arr.push(&Number::from(30.0));

    let sum = rust_sum_number_array(arr);
    assert_eq!(sum, 60.0);
}

#[wasm_bindgen_test]
fn rust_export_process_string_array() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("hello"));
    arr.push(&JsString::from("world"));

    let result = rust_concat_string_array(arr, " ");
    assert_eq!(result, "hello world");
}

#[wasm_bindgen_test]
fn rust_export_round_trip_arrays() {
    let num_arr = rust_create_number_array();
    let doubled: Array<Number> = Array::new_typed();
    for i in 0..num_arr.length() {
        let num = unwrap_get!(num_arr, i);
        doubled.push(&Number::from(num.value_of() * 2.0));
    }
    let sum = rust_sum_number_array(doubled);
    assert_eq!(sum, 12.0); // (1+2+3) * 2 = 12
}
