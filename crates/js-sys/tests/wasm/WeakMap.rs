use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    type SomeKey;
    #[wasm_bindgen(method, setter, structural)]
    fn set_some(this: &SomeKey, val: JsValue);
}

fn some_key() -> Object {
    let key = SomeKey::from(JsValue::from(Object::new()));
    key.set_some("key".into());
    Object::from(JsValue::from(key))
}

#[wasm_bindgen_test]
fn new() {
    assert!(JsValue::from(WeakMap::<Object, JsValue>::new()).is_object());
}

#[wasm_bindgen_test]
fn get_and_set() {
    let map: WeakMap<Object, JsValue> = WeakMap::new();
    let key = some_key();
    map.set(&key, &"value".into());
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(map.get(&key), "value");
        assert_eq!(map.get(&Object::new()), JsValue::undefined());
        assert_eq!(map.get(&some_key()), JsValue::undefined());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(map.get(&key).unwrap(), "value");
        assert_eq!(map.get(&Object::new()), None);
        assert_eq!(map.get(&some_key()), None);
    }
}

#[wasm_bindgen_test]
fn has() {
    let map: WeakMap<Object, JsValue> = WeakMap::new();
    let key = some_key();
    map.set(&key, &"value".into());
    assert!(map.has(&key));
    assert!(!map.has(&Object::new()));
    assert!(!map.has(&some_key()));
}

#[wasm_bindgen_test]
fn delete() {
    let map = WeakMap::<Object, JsString>::new_typed();
    let key = some_key();
    assert!(!map.has(&key));
    map.set(&key, &"value".into());
    assert!(map.has(&key));
    map.delete(&key);
    assert!(!map.has(&key));
}

#[wasm_bindgen_test]
fn weakmap_inheritance() {
    let map: WeakMap<Object, JsValue> = WeakMap::new();
    assert!(map.is_instance_of::<WeakMap>());
    assert!(map.is_instance_of::<Object>());
    let _: &Object = map.as_ref();
}

#[wasm_bindgen_test]
fn new_typed() {
    let map: WeakMap<Object, Object> = WeakMap::new_typed();
    assert!(JsValue::from(map).is_object());
}

#[wasm_bindgen_test]
fn typed_get_set() {
    let map: WeakMap<Object, Object> = WeakMap::new_typed();
    let key = some_key();
    let value = Object::new();
    Reflect::set(&value, &"data".into(), &42.into()).unwrap();
    map.set(&key, &value);
    #[cfg(not(js_sys_unstable_apis))]
    let retrieved = map.get(&key);
    #[cfg(js_sys_unstable_apis)]
    let retrieved = map.get(&key).unwrap();
    let data = Reflect::get_str(&retrieved, &"data".into())
        .unwrap()
        .unwrap();
    assert_eq!(data, 42);
}

#[wasm_bindgen_test]
fn typed_has() {
    let map: WeakMap<Object, Object> = WeakMap::new_typed();
    let key = some_key();
    assert!(!map.has(&key));
    map.set(&key, &Object::new());
    assert!(map.has(&key));
}

#[wasm_bindgen_test]
fn typed_delete() {
    let map: WeakMap<Object, Object> = WeakMap::new_typed();
    let key = some_key();
    map.set(&key, &Object::new());
    assert!(map.has(&key));
    assert!(map.delete(&key));
    assert!(!map.has(&key));
    assert!(!map.delete(&key));
}

#[wasm_bindgen_test]
fn default() {
    let map = WeakMap::default();
    let key = some_key();
    map.set(&key, &"value".into());
    assert!(map.has(&key));
}

#[wasm_bindgen_test]
fn get_option() {
    let map: WeakMap<Object, JsString> = WeakMap::new_typed();
    let key = some_key();
    let value = JsString::from("test_value");

    assert_eq!(map.get_checked(&key), None);

    map.set(&key, &value);

    assert_eq!(map.get_checked(&key), Some(JsString::from("test_value")));

    assert_eq!(map.get_checked(&some_key()), None);
}
