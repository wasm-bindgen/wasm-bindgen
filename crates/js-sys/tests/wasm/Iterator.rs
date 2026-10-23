use std::cell::Cell;

use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/Iterator.js")]
extern "C" {
    fn get_iterable() -> Object;

    fn get_not_iterable() -> Object;

    fn get_symbol_iterator_throws() -> Object;

    fn get_symbol_iterator_not_function() -> Object;

    fn get_symbol_iterator_returns_not_object() -> Object;

    fn get_symbol_iterator_returns_object_without_next() -> Object;

    fn get_iterator_like() -> Iterator<JsString>;

    fn get_naturals() -> Iterator<Number>;

    fn get_throwing_iterator() -> Iterator<JsString>;
}

#[wasm_bindgen_test]
fn try_iter_handles_iteration_protocol() {
    assert_eq!(
        try_iter(&get_iterable())
            .unwrap()
            .unwrap()
            .map(|x| x.unwrap().as_string().unwrap())
            .collect::<Vec<_>>(),
        vec!["one", "two", "three"]
    );

    assert!(try_iter(&get_not_iterable()).unwrap().is_none());
    assert!(try_iter(&get_symbol_iterator_throws()).is_err());
    assert!(try_iter(&get_symbol_iterator_not_function())
        .unwrap()
        .is_none());
    assert!(try_iter(&get_symbol_iterator_returns_not_object())
        .unwrap()
        .is_none());
    assert!(try_iter(&get_symbol_iterator_returns_object_without_next())
        .unwrap()
        .is_none());
}

// Typed Iterator tests using Set's typed iterator
#[wasm_bindgen_test]
fn typed_iterator_next() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("one"));
    set.add(&JsString::from("two"));

    let iter: Iterator<JsString> = set.values();
    let result = iter.next().unwrap();
    assert!(!result.done());
    let value: JsString = result.value();
    assert!(value == "one" || value == "two");
}

#[wasm_bindgen_test]
fn typed_iterator_result_done() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("test"));

    let iter: Iterator<JsString> = set.values();
    let first = iter.next().unwrap();
    assert!(!first.done());
    assert_eq!(first.value(), "test");

    let second = iter.next().unwrap();
    assert!(second.done());
}

#[wasm_bindgen_test]
fn typed_iterator_collect() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("a"));
    arr.push(&JsString::from("b"));
    arr.push(&JsString::from("c"));

    let iter: Iterator<JsString> = arr.values();
    let mut values = Vec::new();
    loop {
        let result = iter.next().unwrap();
        if result.done() {
            break;
        }
        values.push(result.value());
    }
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], "a");
    assert_eq!(values[1], "b");
    assert_eq!(values[2], "c");
}

fn has_iterator_helper(name: &str) -> bool {
    Reflect::get_str(&global(), &"Iterator".into())
        .unwrap()
        .map(|ctor| Reflect::has_str(ctor.unchecked_ref::<Object>(), &name.into()).unwrap())
        .unwrap_or(false)
}

fn strings(values: &[&str]) -> Array<JsString> {
    Array::of(
        &values
            .iter()
            .map(|s| JsString::from(*s))
            .collect::<Vec<_>>(),
    )
}

fn to_vec(iter: Iterator<JsString>) -> Vec<String> {
    iter.to_array().unwrap().iter().map(String::from).collect()
}

#[wasm_bindgen_test]
fn from() {
    if !has_iterator_helper("from") {
        return;
    }
    let iter = Iterator::from_iterable(&strings(&["a", "b"]));
    assert_eq!(to_vec(iter), ["a", "b"]);

    let iter = Iterator::from_iterable(&get_iterator_like());
    assert_eq!(to_vec(iter), ["one", "two"]);

    // The helpers dispatch through `Iterator.prototype`, so a bare `{ next }`
    // object works without wrapping.
    assert_eq!(to_vec(get_iterator_like().take(1)), ["one"]);
}

#[wasm_bindgen_test]
fn concat() {
    if !has_iterator_helper("concat") {
        return;
    }
    let iter = Iterator::concat(&[strings(&["a", "b"]), strings(&["c"])]).unwrap();
    assert_eq!(to_vec(iter), ["a", "b", "c"]);
    assert_eq!(
        Iterator::concat::<Array<JsString>>(&[])
            .unwrap()
            .to_array()
            .unwrap()
            .length(),
        0
    );

    // A bare `{ next }` object has no `Symbol.iterator`, so it is not
    // iterable for `concat`.
    assert!(Iterator::concat(&[get_iterator_like()])
        .unwrap_err()
        .is_instance_of::<TypeError>());
}

#[wasm_bindgen_test]
fn map_is_lazy() {
    if !has_iterator_helper("from") {
        return;
    }
    let calls = Cell::new(0);
    let mut f = |x: JsString, i: u32| -> JsString {
        calls.set(calls.get() + 1);
        JsString::from(format!("{}{i}", String::from(x.to_upper_case())))
    };
    let mapper = ScopedClosure::<dyn FnMut(JsString, u32) -> JsString>::borrow_mut(&mut f);
    let mapped = strings(&["a", "b", "c"]).values().map(&mapper);
    assert_eq!(calls.get(), 0);

    let first = mapped.next().unwrap();
    assert!(!first.done());
    assert_eq!(first.value(), "A0");
    assert_eq!(calls.get(), 1);

    assert_eq!(to_vec(mapped), ["B1", "C2"]);
    assert_eq!(calls.get(), 3);
}

#[wasm_bindgen_test]
fn filter() {
    if !has_iterator_helper("from") {
        return;
    }
    let predicate: Closure<dyn FnMut(JsString, u32) -> bool> =
        Closure::new(|x: JsString, i: u32| x != "b" && i != 3);
    let filtered = strings(&["a", "b", "c", "d", "e"])
        .values()
        .filter(&predicate);
    assert_eq!(to_vec(filtered), ["a", "c", "e"]);
}

#[wasm_bindgen_test]
fn take_and_drop() {
    if !has_iterator_helper("from") {
        return;
    }
    let taken = get_naturals().take(3).to_array().unwrap();
    assert_eq!(taken.length(), 3);
    assert_eq!(taken.get_checked(0).unwrap(), 0);
    assert_eq!(taken.get_checked(2).unwrap(), 2);

    let dropped = get_naturals().drop(5).take(2).to_array().unwrap();
    assert_eq!(dropped.length(), 2);
    assert_eq!(dropped.get_checked(0).unwrap(), 5);
    assert_eq!(dropped.get_checked(1).unwrap(), 6);

    assert_eq!(
        to_vec(strings(&["a", "b", "c"]).values().drop(1)),
        ["b", "c"]
    );
    assert_eq!(to_vec(strings(&["a", "b"]).values().take(0)), [""; 0]);
}

#[wasm_bindgen_test]
fn flat_map() {
    if !has_iterator_helper("from") {
        return;
    }
    let mapper: Closure<dyn FnMut(JsString, u32) -> Array<JsString>> =
        Closure::new(|x: JsString, i: u32| {
            let arr = Array::new_typed();
            for _ in 0..=i {
                arr.push(&x);
            }
            arr
        });
    let flattened = strings(&["a", "b", "c"]).values().flat_map(&mapper);
    assert_eq!(to_vec(flattened), ["a", "b", "b", "c", "c", "c"]);
}

#[wasm_bindgen_test]
fn chained_helpers() {
    if !has_iterator_helper("from") {
        return;
    }
    let mapper: Closure<dyn FnMut(Number, u32) -> Number> =
        Closure::new(|x: Number, _: u32| Number::from(x.value_of() * 2.0));
    let predicate: Closure<dyn FnMut(Number, u32) -> bool> =
        Closure::new(|x: Number, _: u32| x.value_of() % 3.0 == 0.0);
    let result = get_naturals()
        .map(&mapper)
        .filter(&predicate)
        .drop(1)
        .take(3)
        .to_array()
        .unwrap();
    assert_eq!(result.length(), 3);
    assert_eq!(result.get_checked(0).unwrap(), 6);
    assert_eq!(result.get_checked(1).unwrap(), 12);
    assert_eq!(result.get_checked(2).unwrap(), 18);
}

#[wasm_bindgen_test]
fn reduce() {
    if !has_iterator_helper("from") {
        return;
    }
    let joined = strings(&["a", "b", "c"])
        .values()
        .reduce(
            &mut |acc: JsString, x: JsString, i: u32| {
                JsString::from(format!("{}{i}{}", String::from(acc), String::from(x)))
            },
            &JsString::from(">"),
        )
        .unwrap();
    assert_eq!(joined, ">0a1b2c");
}

#[wasm_bindgen_test]
fn to_array() {
    if !has_iterator_helper("from") {
        return;
    }
    let arr = strings(&["x", "y"]).values().to_array().unwrap();
    assert!(arr.is_instance_of::<Array>());
    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get_checked(0).unwrap(), "x");
    assert_eq!(arr.get_checked(1).unwrap(), "y");
}

#[wasm_bindgen_test]
fn for_each() {
    if !has_iterator_helper("from") {
        return;
    }
    let mut seen = Vec::new();
    strings(&["a", "b"])
        .values()
        .for_each(&mut |x: JsString, i: u32| seen.push((String::from(x), i)))
        .unwrap();
    assert_eq!(seen, [("a".to_string(), 0), ("b".to_string(), 1)]);
}

#[wasm_bindgen_test]
fn some_every_find() {
    if !has_iterator_helper("from") {
        return;
    }
    let iter = || strings(&["a", "bb", "ccc"]).values();

    assert!(iter()
        .some(&mut |x: JsString, _: u32| x.length() == 2)
        .unwrap());
    assert!(!iter()
        .some(&mut |x: JsString, _: u32| x.length() == 4)
        .unwrap());

    assert!(iter()
        .every(&mut |x: JsString, i: u32| x.length() == i + 1)
        .unwrap());
    assert!(!iter()
        .every(&mut |x: JsString, _: u32| x.length() == 1)
        .unwrap());

    assert_eq!(
        iter()
            .find(&mut |x: JsString, _: u32| x.length() > 1)
            .unwrap(),
        Some(JsString::from("bb"))
    );
    assert_eq!(iter().find(&mut |_: JsString, i: u32| i > 5).unwrap(), None);

    let mut visited = 0;
    assert!(get_naturals()
        .some(&mut |x: Number, _: u32| {
            visited += 1;
            x == 2
        })
        .unwrap());
    assert_eq!(visited, 3);
}

#[wasm_bindgen_test]
fn errors_propagate() {
    if !has_iterator_helper("from") {
        return;
    }
    assert!(get_throwing_iterator().to_array().is_err());
    assert!(get_throwing_iterator()
        .for_each(&mut |_: JsString, _: u32| {})
        .is_err());
    assert!(get_throwing_iterator()
        .reduce(
            &mut |acc: JsString, _: JsString, _: u32| acc,
            &JsString::from("")
        )
        .is_err());
    assert!(get_throwing_iterator()
        .every(&mut |_: JsString, _: u32| true)
        .is_err());
    assert!(get_throwing_iterator()
        .find(&mut |_: JsString, _: u32| false)
        .is_err());

    let mapper: Closure<dyn FnMut(JsString, u32) -> JsString> =
        Closure::new(|x: JsString, _: u32| x);
    let mapped = get_throwing_iterator().map(&mapper);
    assert_eq!(mapped.next().unwrap().value(), "one");
    assert!(mapped.next().is_err());

    assert!(get_throwing_iterator().take(1).to_array().is_ok());
    assert!(get_throwing_iterator().drop(1).to_array().is_err());
}
