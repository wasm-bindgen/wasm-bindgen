use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

fn set2vec(s: &Set) -> Vec<JsValue> {
    let mut result = Vec::new();
    s.for_each(&mut |x, _, _| result.push(x));
    result
}

#[wasm_bindgen_test]
fn add() {
    let set = Set::new(&JsValue::undefined());
    set.add(&100.into());
    assert_eq!(set.size(), 1);
    assert_eq!(set2vec(&set)[0], 100);
}

#[wasm_bindgen_test]
fn clear() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());
    assert_eq!(set.size(), 3);
    set.clear();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn delete() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.delete(&3.into()));
    assert!(!set.delete(&3.into()));
    assert!(!set.delete(&4.into()));
}

#[wasm_bindgen_test]
fn for_each() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let v = set2vec(&set);
    assert_eq!(v.len(), 3);
    assert!(v.iter().any(|v| *v == 1));
    assert!(v.iter().any(|v| *v == 2));
    assert!(v.iter().any(|v| *v == 3));
}

#[wasm_bindgen_test]
fn has() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.has(&1.into()));
    assert!(!set.has(&1.3.into()));
}

#[wasm_bindgen_test]
fn new() {
    assert_eq!(Set::new(&JsValue::undefined()).size(), 0);
}

#[wasm_bindgen_test]
fn size() {
    let set = Set::new(&JsValue::undefined());
    assert_eq!(set.size(), 0);
    set.add(&1.into());
    assert_eq!(set.size(), 1);
    set.add(&2.into());
    assert_eq!(set.size(), 2);
    set.add(&3.into());
    assert_eq!(set.size(), 3);
}

#[wasm_bindgen_test]
fn set_inheritance() {
    let set = Set::new(&JsValue::undefined());
    assert!(set.is_instance_of::<Set>());
    assert!(set.is_instance_of::<Object>());
    let _: &Object = set.as_ref();
}

#[wasm_bindgen_test]
fn keys() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let list = set
        .keys()
        .into_iter()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    assert_eq!(list.len(), 3);
    assert!(list.iter().any(|l| *l == 1));
    assert!(list.iter().any(|l| *l == 2));
    assert!(list.iter().any(|l| *l == 3));
}

#[wasm_bindgen_test]
fn values() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let list = set
        .values()
        .into_iter()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    assert_eq!(list.len(), 3);
    assert!(list.iter().any(|l| *l == 1));
    assert!(list.iter().any(|l| *l == 2));
    assert!(list.iter().any(|l| *l == 3));
}

// Typed Set tests
#[wasm_bindgen_test]
fn new_typed() {
    let set: Set<JsString> = Set::new_typed();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn new_empty() {
    let set: Set<JsString> = Set::new_empty();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn new_from_items() {
    let set: Set<JsString> = Set::new_from_items(&[
        JsString::from("a"),
        JsString::from("b"),
        JsString::from("c"),
    ]);
    assert_eq!(set.size(), 3);
    assert!(set.has(&JsString::from("a")));
    assert!(set.has(&JsString::from("b")));
    assert!(set.has(&JsString::from("c")));
}

#[wasm_bindgen_test]
fn typed_add_has() {
    let set: Set<JsString> = Set::new_typed();
    let val = JsString::from("test");
    assert!(!set.has(&val));
    set.add(&val);
    assert!(set.has(&val));
    assert_eq!(set.size(), 1);
}

#[wasm_bindgen_test]
fn typed_delete() {
    let set: Set<JsString> = Set::new_typed();
    let val = JsString::from("delete_me");
    set.add(&val);
    assert!(set.has(&val));
    assert!(set.delete(&val));
    assert!(!set.has(&val));
    assert!(!set.delete(&val));
}

#[wasm_bindgen_test]
fn typed_clear() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("a"));
    set.add(&JsString::from("b"));
    set.add(&JsString::from("c"));
    assert_eq!(set.size(), 3);
    set.clear();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn typed_values_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("x"));
    set.add(&JsString::from("y"));
    set.add(&JsString::from("z"));

    let values: Vec<JsString> = set.values().into_iter().map(|v| v.unwrap()).collect();
    assert_eq!(values.len(), 3);
    assert!(values.iter().any(|v| *v == "x"));
    assert!(values.iter().any(|v| *v == "y"));
    assert!(values.iter().any(|v| *v == "z"));
}

#[wasm_bindgen_test]
fn typed_keys_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("key1"));
    set.add(&JsString::from("key2"));

    let keys: Vec<JsString> = set.keys().into_iter().map(|k| k.unwrap()).collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.iter().any(|k| *k == "key1"));
    assert!(keys.iter().any(|k| *k == "key2"));
}

#[wasm_bindgen_test]
fn typed_entries_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("a"));
    set.add(&JsString::from("b"));

    let entries = set.entries_typed();
    let mut count = 0;
    for result in entries.into_iter() {
        // entries_typed returns Iterator<ArrayTuple<T, T>> - the tuple is an Array
        let arr: Array = result.unwrap().unchecked_into();
        // For Set, entries are [value, value] pairs
        assert_eq!(arr.length(), 2);
        count += 1;
    }
    assert_eq!(count, 2);
}

#[wasm_bindgen_test]
fn new_from_iterable() {
    let arr: Array<JsString> = Array::of(&[
        JsString::from("x"),
        JsString::from("y"),
        JsString::from("z"),
    ]);
    let set: Set<JsString> = Set::new_from_iterable(arr).unwrap();
    assert_eq!(set.size(), 3);
    assert!(set.has(&JsString::from("x")));
    assert!(set.has(&JsString::from("y")));
    assert!(set.has(&JsString::from("z")));
}
