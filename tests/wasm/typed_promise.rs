use js_sys::{Array, JsString, Promise, TypedPromise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{typed_future_to_promise, JsFuture, TypedJsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn test_typed_promise_basic() {
    let promise = Promise::resolve(&JsValue::from(42i32));
    let typed_promise: TypedPromise<i32> = promise.typed_unchecked();

    let _generic: Promise = typed_promise.into();
}

#[wasm_bindgen_test]
async fn test_typed_promise_then() {
    let promise = Promise::resolve(&JsValue::from(42i32));
    let typed_promise = promise.typed_unchecked::<i32>();

    let result_promise = typed_promise.then(|num: i32| JsString::from(format!("Number: {}", num)));

    let typed_future: TypedJsFuture<JsString> = result_promise.into();
    let result = typed_future.await.unwrap();

    assert_eq!(result.as_string(), Some("Number: 42".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_chaining() {
    let promise = Promise::resolve(&JsValue::from(10i32));
    let typed_promise = promise.typed_unchecked::<i32>();

    let result_promise = typed_promise
        .then(|num: i32| num * 2)
        .then(|num: i32| num + 5);

    let typed_future: TypedJsFuture<i32> = result_promise.into();
    let result = typed_future.await.unwrap();

    assert_eq!(result, 25);
}

#[wasm_bindgen_test]
async fn test_typed_promise_option() {
    let promise = Promise::resolve(&JsValue::from(42i32));
    let typed_promise = promise.typed_unchecked::<Option<i32>>();

    let result_promise = typed_promise.then(|opt_num: Option<i32>| match opt_num {
        Some(n) => JsString::from(format!("Got: {}", n)),
        None => JsString::from("Got nothing"),
    });

    let typed_future: TypedJsFuture<JsString> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("Got: 42".to_string()));

    let promise = Promise::resolve(&JsValue::null());
    let typed_promise = promise.typed_unchecked::<Option<i32>>();

    let result_promise = typed_promise.then(|opt_num: Option<i32>| match opt_num {
        Some(n) => JsString::from(format!("Got: {}", n)),
        None => JsString::from("Got nothing"),
    });

    let typed_future: TypedJsFuture<JsString> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("Got nothing".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_catch() {
    let promise = Promise::reject(&JsValue::from("Error occurred"));
    let typed_promise = promise.typed_unchecked::<i32>();

    let result_promise = typed_promise.catch(|_error: JsValue| -1i32);

    let typed_future: TypedJsFuture<i32> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, -1);
}

#[wasm_bindgen_test]
async fn test_typed_promise_finally() {
    let promise = Promise::resolve(&JsValue::from(42i32));
    let typed_promise = promise.typed_unchecked::<i32>();

    let result_promise = typed_promise.finally(|| {});

    let typed_future: TypedJsFuture<i32> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
async fn test_typed_promise_different_types() {
    let promise = Promise::resolve(&JsValue::from(3.14f64));
    let typed_promise = promise.typed_unchecked::<f64>();
    let result_promise = typed_promise.then(|num: f64| num * 2.0);

    let typed_future: TypedJsFuture<f64> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 6.28);

    let promise = Promise::resolve(&JsValue::from(true));
    let typed_promise = promise.typed_unchecked::<bool>();
    let result_promise = typed_promise.then(|b: bool| JsString::from(if b { "yes" } else { "no" }));

    let typed_future: TypedJsFuture<JsString> = result_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("yes".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_js_objects() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    arr.push(&JsValue::from(3));

    let promise = Promise::resolve(&arr.into());
    let typed_promise = promise.typed_unchecked::<Array>();

    let result_promise = typed_promise.then(|arr: Array| arr.length());

    let typed_future: TypedJsFuture<u32> = result_promise.into();
    let length = typed_future.await.unwrap();
    assert_eq!(length, 3);
}

#[wasm_bindgen(module = "tests/wasm/typed_promise.js")]
extern "C" {

    #[wasm_bindgen(js_name = "createTypedNumberPromise")]
    fn create_typed_number_promise(value: i32) -> TypedPromise<i32>;

    #[wasm_bindgen(js_name = "createTypedStringPromise")]
    fn create_typed_string_promise(value: &str) -> TypedPromise<JsString>;

    #[wasm_bindgen(js_name = "processTypedPromise")]
    fn process_typed_promise(promise: TypedPromise<i32>) -> TypedPromise<i32>;

    #[wasm_bindgen(js_name = "chainTypedPromises")]
    fn chain_typed_promises(
        promise1: TypedPromise<i32>,
        promise2: TypedPromise<i32>,
    ) -> TypedPromise<i32>;
}

#[wasm_bindgen]
pub fn rust_create_typed_number_promise(value: i32) -> TypedPromise<i32> {
    let promise = Promise::resolve(&JsValue::from(value * 2));
    promise.typed_unchecked::<i32>()
}

#[wasm_bindgen]
pub fn rust_create_typed_string_promise(value: &str) -> TypedPromise<JsString> {
    let js_str = JsString::from(format!("Rust: {}", value));
    let promise = Promise::resolve(&js_str.into());
    promise.typed_unchecked::<JsString>()
}

#[wasm_bindgen]
pub fn rust_process_typed_promise(promise: TypedPromise<i32>) -> TypedPromise<i32> {
    promise.then(|num: i32| num + 100)
}

#[wasm_bindgen]
pub fn rust_chain_typed_promises(promise: TypedPromise<i32>) -> TypedPromise<i32> {
    promise.then(|num1: i32| num1 + 50)
}

#[wasm_bindgen_test]
async fn test_typed_promise_exports() {
    let typed_promise = rust_create_typed_number_promise(21);
    let typed_future: TypedJsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 42);

    let typed_promise = rust_create_typed_string_promise("hello");
    let typed_future: TypedJsFuture<JsString> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("Rust: hello".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_processing() {
    let original_typed = rust_create_typed_number_promise(10);
    let processed_typed = rust_process_typed_promise(original_typed);

    let typed_future: TypedJsFuture<i32> = processed_typed.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 120);
}

#[wasm_bindgen_test]
async fn test_typed_promise_imports() {
    let typed_promise = create_typed_number_promise(15);
    let typed_future: TypedJsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 30);

    let typed_promise = create_typed_string_promise("world");
    let typed_future: TypedJsFuture<JsString> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("JS: world".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_round_trip() {
    let rust_typed = rust_create_typed_number_promise(5);

    let js_processed = process_typed_promise(rust_typed);

    let final_typed = rust_process_typed_promise(js_processed);

    let result = TypedJsFuture::from(final_typed).await.unwrap();

    assert_eq!(result, 310);
}

#[wasm_bindgen_test]
async fn test_typed_js_future_basic() {
    let promise = Promise::resolve(&JsValue::from(42i32));
    let typed_future: TypedJsFuture<i32> = promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
async fn test_typed_js_future_from_typed_promise() {
    let promise = Promise::resolve(&JsValue::from(3.14f64));
    let typed_promise = promise.typed_unchecked::<f64>();
    let typed_future: TypedJsFuture<f64> = typed_promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result, 3.14);
}

#[wasm_bindgen_test]
async fn test_typed_js_future_chained() {
    let promise = Promise::resolve(&JsValue::from(10i32));
    let typed_promise = promise.typed_unchecked::<i32>();
    let chained_promise = typed_promise.then(|num: i32| num * 5);

    let typed_future: TypedJsFuture<i32> = chained_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 50);
}

#[wasm_bindgen_test]
async fn test_typed_js_future_option() {
    let promise = Promise::resolve(&JsValue::from(123i32));
    let typed_future: TypedJsFuture<Option<i32>> = promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result, Some(123));

    let null_promise = Promise::resolve(&JsValue::null());
    let typed_future: TypedJsFuture<Option<i32>> = null_promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result, None);
}

#[wasm_bindgen_test]
async fn test_typed_js_future_string() {
    let promise = Promise::resolve(&JsValue::from("hello world"));
    let typed_future: TypedJsFuture<JsString> = promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("hello world".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_js_future_error() {
    let promise = Promise::reject(&JsValue::from("error message"));
    let typed_future: TypedJsFuture<i32> = promise.into();

    let result = typed_future.await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.as_string(), Some("error message".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_resolve_basic() {
    let typed_promise = TypedPromise::resolve(42i32);

    let result_promise = typed_promise.then(|num: i32| num * 2);
    let typed_future: TypedJsFuture<i32> = result_promise.into();
    let final_result = typed_future.await.unwrap();
    assert_eq!(final_result, 84);
}

#[wasm_bindgen_test]
async fn test_typed_promise_resolve_different_types() {
    let int_promise = TypedPromise::resolve(123i32);
    let int_future: TypedJsFuture<i32> = int_promise.into();
    let int_result = int_future.await.unwrap();
    assert_eq!(int_result, 123);

    let float_promise = TypedPromise::resolve(3.14f64);
    let float_future: TypedJsFuture<f64> = float_promise.into();
    let float_result = float_future.await.unwrap();
    assert_eq!(float_result, 3.14);

    let bool_promise = TypedPromise::resolve(true);
    let bool_future: TypedJsFuture<bool> = bool_promise.into();
    let bool_result = bool_future.await.unwrap();
    assert_eq!(bool_result, true);

    let js_str = JsString::from("hello world");
    let string_promise = TypedPromise::resolve(js_str);
    let string_future: TypedJsFuture<JsString> = string_promise.into();
    let string_result = string_future.await.unwrap();
    assert_eq!(string_result.as_string(), Some("hello world".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_resolve_chaining() {
    let promise = TypedPromise::resolve(10i32)
        .then(|num: i32| num * 3)
        .then(|num: i32| num + 7);

    let typed_future: TypedJsFuture<i32> = promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 37);
}

#[wasm_bindgen_test]
async fn test_typed_promise_resolve_with_typed_future() {
    let typed_promise = TypedPromise::resolve(99i32);
    let typed_future: TypedJsFuture<i32> = typed_promise.into();

    let result = typed_future.await.unwrap();
    assert_eq!(result, 99);
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_basic() {
    let future = async { Ok(42i32) };
    let typed_promise = typed_future_to_promise(future);

    let typed_future: TypedJsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_different_types() {
    // Test with f64
    let future = async { Ok(3.14f64) };
    let typed_promise = typed_future_to_promise(future);
    let typed_future: TypedJsFuture<f64> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 3.14);

    // Test with bool
    let future = async { Ok(true) };
    let typed_promise = typed_future_to_promise(future);
    let typed_future: TypedJsFuture<bool> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, true);

    // Test with JsString
    let future = async { Ok(JsString::from("hello world")) };
    let typed_promise = typed_future_to_promise(future);
    let typed_future: TypedJsFuture<JsString> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result.as_string(), Some("hello world".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_error() {
    let future = async { Err(JsValue::from("future failed")) };
    let typed_promise: TypedPromise<i32> = typed_future_to_promise(future);

    let typed_future: TypedJsFuture<i32> = typed_promise.into();
    let result = typed_future.await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.as_string(), Some("future failed".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_async_computation() {
    let future = async {
        // Simulate some async work
        let base = 10i32;
        let multiplier = 4i32;
        Ok(base * multiplier)
    };

    let typed_promise = typed_future_to_promise(future);
    let typed_future: TypedJsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 40);
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_chaining() {
    let future = async { Ok(5i32) };
    let typed_promise = typed_future_to_promise(future);

    // Chain operations on the TypedPromise
    let chained = typed_promise.then(|num: i32| num * 10);
    let typed_future: TypedJsFuture<i32> = chained.into();
    let result = typed_future.await.unwrap();
    assert_eq!(result, 50);
}
