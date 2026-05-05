use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

fn make_callback() -> Function<fn(JsValue) -> Undefined> {
    let closure: Closure<dyn FnMut(JsValue)> = Closure::new(|_held: JsValue| {});
    Function::from_closure(closure)
}

#[wasm_bindgen_test]
fn new() {
    let cb = make_callback();
    let registry = FinalizationRegistry::new(&cb);
    assert!(registry.is_instance_of::<FinalizationRegistry>());
    assert!(registry.is_instance_of::<Object>());
    let _: &Object = registry.as_ref();
}

#[wasm_bindgen_test]
fn register_and_unregister() {
    let cb = make_callback();
    let registry = FinalizationRegistry::new(&cb);

    let target = Object::new();
    let token = Object::new();
    registry.register_with_token(target.as_ref(), &"held".into(), token.as_ref());

    // Unregistering with the matching token should remove the registration.
    let removed = registry.unregister(token.as_ref());
    assert!(removed);

    // Unregistering again is a no-op and returns false.
    let removed_again = registry.unregister(token.as_ref());
    assert!(!removed_again);
}

#[wasm_bindgen_test]
fn unregister_unknown_token_is_false() {
    let cb = make_callback();
    let registry = FinalizationRegistry::new(&cb);

    let token = Object::new();
    let removed = registry.unregister(token.as_ref());
    assert!(!removed);
}

#[wasm_bindgen_test]
fn register_without_token() {
    let cb = make_callback();
    let registry = FinalizationRegistry::new(&cb);

    let target = Object::new();
    // Registering without an unregister token should not throw.
    registry.register(target.as_ref(), &JsValue::from(42));
}

#[wasm_bindgen_test]
fn held_value_can_be_arbitrary() {
    let cb = make_callback();
    let registry = FinalizationRegistry::new(&cb);

    let target = Object::new();
    let token = Object::new();
    // heldValue may be any JS value, including primitives, strings, objects.
    registry.register_with_token(target.as_ref(), &JsValue::null(), token.as_ref());
    assert!(registry.unregister(token.as_ref()));

    registry.register_with_token(target.as_ref(), &"a string".into(), token.as_ref());
    assert!(registry.unregister(token.as_ref()));

    registry.register_with_token(target.as_ref(), &JsValue::from(1.5), token.as_ref());
    assert!(registry.unregister(token.as_ref()));
}
