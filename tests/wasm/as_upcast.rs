use js_sys::Promise;
use wasm_bindgen::convert::AsUpcast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/as_upcast.js")]
extern "C" {
    type Parent;

    #[wasm_bindgen(extends = Parent)]
    type Child;

    #[wasm_bindgen(constructor)]
    fn new(value: i32) -> Parent;

    #[wasm_bindgen(constructor)]
    fn new_child(value: i32) -> Child;

    fn process_parent(obj: impl AsUpcast<Parent>) -> i32;

    #[wasm_bindgen(js_name = "process_parent")]
    fn process_parent_ref<'a>(obj: impl AsUpcast<&'a Parent>) -> i32;

    fn process_promise(p: impl AsUpcast<Promise<Parent>>) -> Promise<i32>;
}

#[wasm_bindgen_test]
fn test_as_upcast_by_value() {
    let parent = Parent::new(42);

    let result = process_parent(parent);
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn test_as_upcast_child_to_parent() {
    let child = Child::new_child(99);

    let result = process_parent(child);
    assert_eq!(result, 99);
}

#[wasm_bindgen_test]
async fn test_as_upcast_generic_promise() {
    let child = Child::new_child(123);
    let promise_child: Promise<Child> = Promise::resolve(&child);

    let result_promise = process_promise(promise_child);
    let future = wasm_bindgen_futures::JsFuture::from(result_promise);
    let result = future.await.unwrap();

    assert_eq!(result, 123);
}

#[wasm_bindgen_test]
fn test_as_upcast_ref_with_lifetime() {
    let parent = Parent::new(42);

    let result = process_parent_ref(&parent);
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn test_as_upcast_ref_child_to_parent() {
    let child = Child::new_child(99);

    let result = process_parent_ref(&child);
    assert_eq!(result, 99);
}
