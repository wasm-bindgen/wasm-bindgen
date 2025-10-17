use js_sys::*;
use std::iter::FromIterator;
use wasm_bindgen::{convert::FromWasmAbi, prelude::*, ErasableGeneric};
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

fn to_rust<T: ErasableGeneric<Repr = JsValue> + FromWasmAbi>(arr: &Array<T>) -> Vec<T> {
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
    let array = Array::new_with_length(5);
    assert_eq!(array.length(), 5);
    assert_eq!(array.get(4), JsValue::undefined());
    array.set(4, JsValue::from("a"));
    assert_eq!(array.get(4), "a");
    assert_eq!(array.length(), 5);
}

#[wasm_bindgen_test]
fn get() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    assert_eq!(array.get(3), "n");
    assert_eq!(array.get(4), JsValue::undefined());
}

#[wasm_bindgen_test]
fn set() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    array.set(0, JsValue::from("b"));
    assert_eq!(array.get(0), "b");

    assert_eq!(array.get(4), JsValue::undefined());
    assert_eq!(array.length(), 4);
    array.set(4, JsValue::from("d"));
    assert_eq!(array.length(), 5);
    assert_eq!(array.get(4), "d");

    assert_eq!(array.get_checked(10), None);
    assert_eq!(array.length(), 5);
    array.set(10, JsValue::from("z"));
    assert_eq!(array.length(), 11);
    assert_eq!(array.get_checked(10), Some("z".into()));
    assert_eq!(array.get_checked(9), None);
}

#[wasm_bindgen_test]
fn delete() {
    let array = js_array![JsValue; "a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    array.delete(0);
    assert_eq!(array.get(0), JsValue::undefined());
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
    assert!(Array::is_array(&Array::new().into()));
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
    #[allow(deprecated)]
    let arr = Array::of3(&a, &b, &c);
    let vec = to_rust(&arr);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], a);
    assert_eq!(vec[1], b);
    assert_eq!(vec[2], c);
}

#[wasm_bindgen_test]
fn pop() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let item = characters.pop();
    assert_eq!(item, JsValue::from(2));
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
    let shiftedItem = characters.shift();

    assert_eq!(shiftedItem, 8);
    assert_eq!(characters.length(), 5);
}

#[wasm_bindgen_test]
fn unshift() {
    let characters = js_array![JsValue; 8, 5, 4, 3, 1, 2];
    let length = characters.unshift(&"abba".into());

    assert_eq!(length, 7);
    assert_eq!(to_rust(&characters)[0], "abba");
}

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
    assert_eq!(Array::new().length(), 0);
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
    assert_eq!(
        even.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        2
    );
    let odd = js_array![JsValue; 1, 3, 5, 7];
    assert_eq!(
        odd.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        JsValue::undefined(),
    );
    let mixed = js_array![JsValue; 3, 5, 7, 10];
    assert_eq!(
        mixed.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        10
    );
}

#[wasm_bindgen_test]
fn find_last() {
    let even = js_array![JsValue; 2, 4, 6, 8];
    assert_eq!(
        even.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        8
    );
    let odd = js_array![JsValue; 1, 3, 5, 7];
    assert_eq!(
        odd.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        JsValue::undefined(),
    );
    let mixed = js_array![JsValue; 3, 5, 7, 10];
    assert_eq!(
        mixed.find_last(&mut |x, _, _| x.as_f64().unwrap() % 2.0 != 0.0),
        7
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
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
        &"".into(),
    );
    assert_eq!(arr, "01234");
}

#[wasm_bindgen_test]
fn reduce_right() {
    let arr = js_array![JsString; "0", "1", "2", "3", "4"].reduce_right(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
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
    let output = js_array![JsValue; 1, "a", Date::new(&"21 Dec 1997 14:12:00 UTC".into())]
        .to_locale_string(&"en".into(), &JsValue::undefined());
    assert!(!String::from(output).is_empty());
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
    let array = Array::new();
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
    let array_constructor = Array::new().constructor();
    if Reflect::get(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_undefined()
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
    let array_constructor = Array::new().constructor();
    if Reflect::get(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_undefined()
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
    let array_constructor = Array::new().constructor();
    if Reflect::get(array_constructor.as_ref(), &"fromAsync".into())
        .unwrap()
        .is_undefined()
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
    let length = accepts_jsvalue_array(number_array.upcast());
    assert_eq!(length, 3);

    // Also test with Array<JsString>
    let string_array = js_array![JsString; "a", "b", "c"];
    let length = accepts_jsvalue_array(string_array.upcast());
    assert_eq!(length, 3);
}
