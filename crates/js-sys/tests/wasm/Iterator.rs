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
