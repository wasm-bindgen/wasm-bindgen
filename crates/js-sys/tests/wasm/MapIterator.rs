use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn entries() {
    let map: Map<JsString, Number> = Map::new_typed();
    map.set(&"uno".into(), &1.into());

    let entries = map.entries_typed();

    let next = entries.next().unwrap();
    assert!(!next.done());
    assert!(next.value().is_object());

    assert_eq!(
        Reflect::get_str(&next.value(), &"0".into())
            .unwrap()
            .unwrap(),
        "uno"
    );
    assert_eq!(
        Reflect::get_str(&next.value(), &"1".into())
            .unwrap()
            .unwrap(),
        1
    );

    assert_eq!(next.value().get0(), "uno");
    assert_eq!(next.value().get1(), 1);

    let next = entries.next().unwrap();
    assert!(next.done());
    assert!(next.value().is_undefined());
}

#[wasm_bindgen_test]
fn keys() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"uno".into(), &1.into());

    let keys = map.keys();

    let next = keys.next().unwrap();
    assert!(!next.done());
    assert_eq!(next.value(), "uno");

    let next = keys.next().unwrap();
    assert!(next.done());
    assert!(next.value().is_undefined());
}

#[wasm_bindgen_test]
fn values() {
    let map: Map<JsValue, JsValue> = Map::new();
    map.set(&"uno".into(), &1.into());

    let values = map.values();

    let next = values.next().unwrap();
    assert!(!next.done());
    assert_eq!(next.value(), 1);

    let next = values.next().unwrap();
    assert!(next.done());
    assert!(next.value().is_undefined());
}
