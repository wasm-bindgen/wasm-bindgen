use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn entries() {
    #[cfg(not(js_sys_unstable_apis))]
    let s = Set::new(&JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    let s: Set<JsValue> = Set::new();
    s.add(&1.into());
    let iter = s.entries_typed();
    let obj = iter.next().unwrap();
    assert!(!obj.done());
    let array = Array::from_iterable(&obj.value()).unwrap();
    assert_eq!(array.length(), 2);
    array.for_each(&mut |a, _, _| {
        assert_eq!(a, 1);
    });

    assert!(iter.next().unwrap().done());
}

#[wasm_bindgen_test]
fn keys() {
    #[cfg(not(js_sys_unstable_apis))]
    let s = Set::new(&JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    let s: Set<JsValue> = Set::new();
    s.add(&1.into());
    let iter = s.keys();
    let obj = iter.next().unwrap();
    assert!(!obj.done());
    assert_eq!(obj.value(), 1);
    assert!(iter.next().unwrap().done());
}

#[wasm_bindgen_test]
fn values() {
    #[cfg(not(js_sys_unstable_apis))]
    let s = Set::new(&JsValue::undefined());
    #[cfg(js_sys_unstable_apis)]
    let s: Set<JsValue> = Set::new();
    s.add(&1.into());
    let iter = s.values();
    let obj = iter.next().unwrap();
    assert!(!obj.done());
    assert_eq!(obj.value(), 1);
    assert!(iter.next().unwrap().done());
}
