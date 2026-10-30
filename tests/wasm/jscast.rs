use js_sys::{JsString, Map, Number, Object};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/jscast.js")]
extern "C" {
    type JsCast1;
    #[wasm_bindgen(constructor)]
    fn new() -> JsCast1;
    #[wasm_bindgen(method)]
    fn myval(this: &JsCast1) -> u32;

    type JsCast2;
    #[wasm_bindgen(constructor)]
    fn new() -> JsCast2;

    #[wasm_bindgen(extends = JsCast1)]
    type JsCast3;
    #[wasm_bindgen(constructor)]
    fn new() -> JsCast3;

    #[wasm_bindgen(extends = crate::jscast::JsCast1, extends = JsCast3)]
    type JsCast4;
    #[wasm_bindgen(constructor)]
    fn new() -> JsCast4;
}

#[wasm_bindgen_test]
fn instanceof_works() {
    let a = JsCast1::new();
    let b = JsCast2::new();
    let c = JsCast3::new();

    assert!(a.is_instance_of::<JsCast1>());
    assert!(!a.is_instance_of::<JsCast2>());
    assert!(!a.is_instance_of::<JsCast3>());

    assert!(!b.is_instance_of::<JsCast1>());
    assert!(b.is_instance_of::<JsCast2>());
    assert!(!b.is_instance_of::<JsCast3>());

    assert!(c.is_instance_of::<JsCast1>());
    assert!(!c.is_instance_of::<JsCast2>());
    assert!(c.is_instance_of::<JsCast3>());
}

#[wasm_bindgen_test]
fn casting() {
    let a = JsCast1::new();
    let b = JsCast2::new();
    let c = JsCast3::new();

    assert!(a.dyn_ref::<JsCast1>().is_some());
    assert!(a.dyn_ref::<JsCast2>().is_none());
    assert!(a.dyn_ref::<JsCast3>().is_none());

    assert!(b.dyn_ref::<JsCast1>().is_none());
    assert!(b.dyn_ref::<JsCast2>().is_some());
    assert!(b.dyn_ref::<JsCast3>().is_none());

    assert!(c.dyn_ref::<JsCast1>().is_some());
    assert!(c.dyn_ref::<JsCast2>().is_none());
    assert!(c.dyn_ref::<JsCast3>().is_some());
}

#[wasm_bindgen_test]
fn method_calling() {
    let a = JsCast1::new();
    let b = JsCast3::new();

    assert_eq!(a.myval(), 1);
    assert_eq!(b.dyn_ref::<JsCast1>().unwrap().myval(), 3);
    assert_eq!(b.unchecked_ref::<JsCast1>().myval(), 3);
    let c: &JsCast1 = b.as_ref();
    assert_eq!(c.myval(), 3);
}

#[wasm_bindgen_test]
fn multiple_layers_of_inheritance() {
    let a = JsCast4::new();
    assert!(a.is_instance_of::<JsCast4>());
    assert!(a.is_instance_of::<JsCast3>());
    assert!(a.is_instance_of::<JsCast1>());

    let _: &JsCast3 = a.as_ref();
    let b: &JsCast1 = a.as_ref();
    assert_eq!(b.myval(), 4);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map<JsString, Number>, extends = Object, js_name = Map)]
    type ScoreMap;
    #[wasm_bindgen(constructor, js_class = Map)]
    fn new() -> ScoreMap;

    #[wasm_bindgen(extends = Map<JsString, T>, extends = Object, js_name = Map)]
    type Scores<T = JsValue>;
    #[wasm_bindgen(constructor, js_class = Map)]
    fn new_scores<T>() -> Scores<T>;

    #[wasm_bindgen(extends = Map<T, T>, extends = Object, js_name = Map)]
    type Pairs<T>;
    #[wasm_bindgen(constructor, js_class = Map)]
    fn new_pairs<T>() -> Pairs<T>;
}

#[wasm_bindgen_test]
fn extends_with_type_arguments() {
    let key = JsString::from("a");

    let m = ScoreMap::new();
    m.set(&key, &Number::from(1.5));
    assert_eq!(m.get_checked(&key).unwrap().value_of(), 1.5);
    let parent: &Map<JsString, Number> = m.as_ref();
    assert_eq!(parent.size(), 1);
    let parent: Map<JsString, Number> = m.into();
    assert_eq!(parent.get_checked(&key).unwrap().value_of(), 1.5);

    let s: Scores<Number> = Scores::new_scores();
    s.set(&key, &Number::from(2));
    let parent: &Map<JsString, Number> = s.as_ref();
    assert_eq!(parent.get_checked(&key).unwrap().value_of(), 2.0);
    let _: &Object = s.as_ref();

    let p: Pairs<JsString> = Pairs::new_pairs();
    p.set(&key, &JsString::from("b"));
    let parent: Map<JsString, JsString> = p.into();
    assert_eq!(parent.get_checked(&key).unwrap(), "b");
}
