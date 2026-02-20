use js_sys::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn clear() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"foo".into(), &"bar".into());
    map.set(&"bar".into(), &"baz".into());
    assert_eq!(map.size(), 2);
    map.clear();
    assert_eq!(map.size(), 0);
    map.clear();
    assert_eq!(map.size(), 0);
}

#[wasm_bindgen_test]
fn delete() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"foo".into(), &"bar".into());
    assert_eq!(map.size(), 1);
    assert!(map.delete(&"foo".into()));
    assert!(!map.delete(&"bar".into()));
    assert_eq!(map.size(), 0);
}

#[wasm_bindgen_test]
fn for_each() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&1.into(), &true.into());
    map.set(&2.into(), &false.into());
    map.set(&3.into(), &"awoo".into());
    map.set(&4.into(), &100.into());
    map.set(&5.into(), &Array::<JsValue>::new().into());
    map.set(&6.into(), &Object::new().into());

    let mut res = Vec::new();
    map.for_each(&mut |value, key| {
        if value.as_bool().is_some() {
            res.push((key, value));
        }
    });

    assert_eq!(map.size(), 6);
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].0, 1);
    assert_eq!(res[0].1, true);
    assert_eq!(res[1].0, 2);
    assert_eq!(res[1].1, false);
}

#[wasm_bindgen_test]
fn get() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"foo".into(), &"bar".into());
    map.set(&1.into(), &2.into());
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(map.get(&"foo".into()), "bar");
        assert_eq!(map.get(&1.into()), 2);
        assert!(map.get(&2.into()).is_undefined());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(map.get(&"foo".into()), Some(JsValue::from("bar")));
        assert_eq!(map.get(&1.into()), Some(JsValue::from(2)));
        assert_eq!(map.get(&2.into()), None);
    }
}

#[wasm_bindgen_test]
fn has() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"foo".into(), &"bar".into());
    assert!(map.has(&"foo".into()));
    assert!(!map.has(&"bar".into()));
}

#[wasm_bindgen_test]
fn new() {
    assert_eq!(Map::<JsValue, JsValue>::new().size(), 0);
}

#[wasm_bindgen_test]
fn set() {
    let map: Map<JsValue, JsValue> = Map::new();
    let new = map.set(&"foo".into(), &"bar".into());
    assert!(map.has(&"foo".into()));
    assert!(new.has(&"foo".into()));
}

#[wasm_bindgen_test]
fn size() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"foo".into(), &"bar".into());
    map.set(&"bar".into(), &"baz".into());
    assert_eq!(map.size(), 2);
}

#[wasm_bindgen_test]
fn map_inheritance() {
    let map: Map<JsValue, JsValue> = Map::new();
    assert!(map.is_instance_of::<Map>());
    assert!(map.is_instance_of::<Object>());
    let _: &Object = map.as_ref();
}

#[wasm_bindgen_test]
fn new_typed() {
    let map: Map<JsString, JsString> = Map::new_typed();
    assert_eq!(map.size(), 0);
}

#[wasm_bindgen_test]
fn typed_get_set() {
    let map: Map<JsString, JsString> = Map::new_typed();
    let key = JsString::from("test_key");
    let value = JsString::from("test_value");
    map.set(&key, &value);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(map.get(&key), "test_value");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(map.get(&key).unwrap(), "test_value");
    assert_eq!(map.size(), 1);
}

#[wasm_bindgen_test]
fn typed_has() {
    let map: Map<JsString, JsString> = Map::new_typed();
    let key = JsString::from("my_key");
    assert!(!map.has(&key));
    map.set(&key, &JsString::from("value"));
    assert!(map.has(&key));
}

#[wasm_bindgen_test]
fn typed_delete() {
    let map: Map<JsString, JsString> = Map::new_typed();
    let key = JsString::from("delete_me");
    map.set(&key, &JsString::from("val"));
    assert!(map.has(&key));
    assert!(map.delete(&key));
    assert!(!map.has(&key));
    assert!(!map.delete(&key));
}

#[wasm_bindgen_test]
fn typed_clear() {
    let map: Map<JsString, JsString> = Map::new_typed();
    map.set(&JsString::from("a"), &JsString::from("1"));
    map.set(&JsString::from("b"), &JsString::from("2"));
    assert_eq!(map.size(), 2);
    map.clear();
    assert_eq!(map.size(), 0);
}

#[wasm_bindgen_test]
fn typed_values_iterator() {
    let map: Map<JsString, JsString> = Map::new_typed();
    map.set(&JsString::from("a"), &JsString::from("one"));
    map.set(&JsString::from("b"), &JsString::from("two"));
    map.set(&JsString::from("c"), &JsString::from("three"));

    let values: Vec<JsString> = map.values().into_iter().map(|v| v.unwrap()).collect();
    assert_eq!(values.len(), 3);
    assert!(values.iter().any(|v| *v == "one"));
    assert!(values.iter().any(|v| *v == "two"));
    assert!(values.iter().any(|v| *v == "three"));
}

#[wasm_bindgen_test]
fn typed_entries_iterator() {
    let map = Map::new_typed();
    map.set(&JsString::from("k1"), &JsString::from("v1"));
    map.set(&JsString::from("k2"), &JsString::from("v2"));

    let entries = map.entries_typed();
    let entries2: Iterator<ArrayTuple<(JsString, JsValue)>> = entries.unchecked_into();

    let mut count = 0;
    for result in entries2.into_iter() {
        let arr = result.unwrap();
        assert_eq!(arr.len(), 2);
        count += 1;
    }
    assert_eq!(count, 2);
}

#[wasm_bindgen_test]
fn new_from_entries() {
    let entries: Array<ArrayTuple<(JsString, JsString)>> = Array::new_typed();
    entries.push(&ArrayTuple::new2(
        &JsString::from("a"),
        &JsString::from("1"),
    ));
    entries.push(&ArrayTuple::new2(
        &JsString::from("b"),
        &JsString::from("2"),
    ));
    entries.push(&ArrayTuple::new2(
        &JsString::from("c"),
        &JsString::from("3"),
    ));

    let map: Map<JsString, JsString> = Map::new_from_entries(&entries);
    assert_eq!(map.size(), 3);
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(map.get(&JsString::from("a")), "1");
        assert_eq!(map.get(&JsString::from("b")), "2");
        assert_eq!(map.get(&JsString::from("c")), "3");
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(map.get(&JsString::from("a")).unwrap(), "1");
        assert_eq!(map.get(&JsString::from("b")).unwrap(), "2");
        assert_eq!(map.get(&JsString::from("c")).unwrap(), "3");
    }
}

#[wasm_bindgen_test]
fn typed_try_for_each() {
    let map: Map<JsString, JsString> = Map::new_typed();
    map.set(&JsString::from("a"), &JsString::from("1"));
    map.set(&JsString::from("b"), &JsString::from("2"));

    let mut keys = Vec::new();
    map.for_each(&mut |key: JsString, _value: JsString| {
        keys.push(key);
    });
    assert_eq!(keys.len(), 2);
}

#[wasm_bindgen_test]
fn get_option() {
    let map: Map<JsString, JsString> = Map::new_typed();
    let key = JsString::from("test_key");
    let value = JsString::from("test_value");

    // Key not present - should return None
    assert_eq!(map.get_checked(&key), None);

    // Add key
    map.set(&key, &value);

    // Key present - should return Some
    assert_eq!(map.get_checked(&key), Some(JsString::from("test_value")));

    // Non-existent key - should return None
    assert_eq!(map.get_checked(&JsString::from("missing")), None);
}
