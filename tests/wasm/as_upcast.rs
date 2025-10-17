use js_sys::Promise;
use wasm_bindgen::convert::{AsUpcast, Upcast};
use wasm_bindgen::{prelude::*, UpcastCore};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/as_upcast.js")]
extern "C" {
    #[derive(UpcastCore)]
    type Parent;

    #[derive(UpcastCore)]
    #[wasm_bindgen(extends = Parent)]
    type Child;

    #[wasm_bindgen(constructor)]
    fn new(value: i32) -> Parent;

    #[wasm_bindgen(constructor)]
    fn new_child(value: i32) -> Child;

    fn process_parent(obj: impl AsUpcast<Parent>) -> i32;

    fn process_promise(p: impl AsUpcast<Promise<Parent>>) -> Promise<i32>;
}

// Manually implement inheritance upcast
impl Upcast<Parent> for Child {}

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
