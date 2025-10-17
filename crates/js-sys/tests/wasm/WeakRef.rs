use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    type SomeValue;
    #[wasm_bindgen(method, setter, structural)]
    fn set_some(this: &SomeValue, val: JsValue);
}

fn some_value() -> Object {
    let value = SomeValue::from(JsValue::from(Object::new()));
    value.set_some("value".into());
    Object::from(JsValue::from(value))
}

#[wasm_bindgen_test]
fn new() {
    let value = some_value();
    let weak_ref = WeakRef::new(&value);
    assert!(JsValue::from(weak_ref).is_object());
}

#[wasm_bindgen_test]
fn deref() {
    let value = some_value();
    let weak_ref = WeakRef::new(&value);
    let dereffed = weak_ref.deref();
    assert!(dereffed.is_some());
    let dereffed_obj = dereffed.unwrap();
    assert_eq!(&value, &dereffed_obj);

    // Check that we can still access properties of the dereferenced object
    let prop = Reflect::get_str(&dereffed_obj, &"some".into())
        .unwrap()
        .unwrap();
    assert_eq!(prop, "value");
}

#[wasm_bindgen_test]
fn weakref_inheritance() {
    let value = some_value();
    let weak_ref = WeakRef::new(&value);
    assert!(weak_ref.is_instance_of::<WeakRef>());
    assert!(weak_ref.is_instance_of::<Object>());
    let _: &Object = weak_ref.as_ref();
}

// Typed WeakRef tests
#[wasm_bindgen_test]
fn typed_new() {
    let value = some_value();
    let weak_ref: WeakRef<Object> = WeakRef::new(&value);
    assert!(JsValue::from(weak_ref).is_object());
}

#[wasm_bindgen_test]
fn typed_deref() {
    let value = some_value();
    let weak_ref: WeakRef<Object> = WeakRef::new(&value);
    let dereffed: Option<Object> = weak_ref.deref();
    assert!(dereffed.is_some());
    let dereffed_obj = dereffed.unwrap();
    assert_eq!(&value, &dereffed_obj);
}
