use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/typed_promise.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn checkPromise(p: &Promise<i32>) -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
async fn test_typed_promise_basic() {
    // Create a typed Promise<i32>
    let promise = Promise::resolve(&42i32.to_js());
    checkPromise(&promise).await.unwrap();

    let out: i32 = JsFuture::from(promise.clone()).await.unwrap().from_js();
    assert_eq!(out, 42);

    // Use map with typed closure
    let closure = Closure::new(|num: i32| (num + 1) as f64);
    let result_promise: Promise<f64> = promise.map(&closure);
    let out: f64 = JsFuture::from(result_promise).await.unwrap().from_js();
    assert_eq!(out, 43.0);

    // Alternative: cast and use typed promise
    let promise = Promise::resolve(&JsRef::from(42i32));
    let typed_promise: Promise<i32> = promise.unchecked_into();
    checkPromise(&typed_promise).await.unwrap();

    // String promise example
    let string_promise: Promise<JsString> =
        Promise::resolve(&JsString::from("test")).unchecked_into();
    let typed_future: JsFuture<_> = string_promise.into();
    let result: JsString = typed_future.await.unwrap().from_js();

    assert_eq!(result.as_string(), Some("test".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_option() {
    let promise: Promise<Option<JsString>> =
        Promise::resolve(&JsValue::from(Some(JsString::from("test")))).unchecked_into();

    let closure = Closure::new(|opt_num: Option<JsString>| match opt_num {
        Some(n) => JsString::from(format!("Got: {}", n)),
        None => JsString::from("Got nothing"),
    });
    let result_future: JsFuture<_> = promise.map(&closure).into();
    result_future.await.unwrap();

    let promise: Promise<Option<i32>> = Promise::resolve(&JsValue::from(Some(4))).unchecked_into();
    let closure = Closure::new(|opt_num: JsRef<Option<i32>>| {
        assert_eq!(opt_num.from_js(), Some(4));
    });
    let result_future: JsFuture<_> = promise.then(&closure).into();
    result_future.await.unwrap();
}

#[wasm_bindgen_test]
async fn test_typed_promise_catch() {
    let promise = Promise::reject(&JsValue::from("Error occurred"));
    let typed_promise: Promise<i32> = promise.unchecked_into();

    let closure = Closure::new(|_error: JsValue| ());
    let result_promise = typed_promise.catch(&closure);

    let typed_future: JsFuture = result_promise.into();
    let _ = typed_future.await.unwrap();
}

#[wasm_bindgen_test]
async fn test_typed_promise_different_types() {
    let promise = Promise::resolve(&JsValue::from(3.15f64));
    let typed_promise: Promise<f64> = promise.unchecked_into();
    let closure = Closure::new(|num: f64| num * 2.0);
    let result_promise = typed_promise.map(&closure);

    let typed_future: JsFuture<_> = result_promise.into();
    let result: f64 = typed_future.await.unwrap().from_js();
    assert_eq!(result, 6.3);

    let promise = Promise::resolve(&JsValue::from(true));
    let typed_promise: Promise<bool> = promise.unchecked_into();
    let closure = Closure::new(|b: bool| JsString::from(if b { "yes" } else { "no" }));
    let result_promise = typed_promise.map(&closure);

    let typed_future: JsFuture<JsString> = result_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result.as_string(), Some("yes".to_string()));
}

// TODO: Array<T> generic types are not yet supported
// #[wasm_bindgen_test]
// async fn test_typed_promise_js_objects() {
//     // New `_t` suffix convention for existing methods that
//     // don't support typing without that being a breaking change
//     let arr = Array::new();
//     arr.push(&JsValue::from(1));
//     arr.push(&JsValue::from(2));
//     arr.push(&JsValue::from(3));
//
//     let arr_typed: Array<i32> = arr.unchecked_into();
//     let promise: Promise<Array<i32>> = Promise::resolve(&JsValue::from(arr_typed)).unchecked_into();
//
//     let closure = Closure::new(|arr: JsRef<Array<i32>>| arr.from_js().length());
//     let result_promise = promise.map(&closure);
//
//     let typed_future: JsFuture<_> = result_promise.into();
//     let length: u32 = typed_future.await.unwrap().from_js();
//     assert_eq!(length, 3);
// }

#[wasm_bindgen(module = "tests/wasm/typed_promise.js")]
extern "C" {

    #[wasm_bindgen(js_name = "createTypedNumberPromise")]
    fn create_typed_number_promise(value: i32) -> Promise<i32>;

    #[wasm_bindgen(js_name = "createTypedStringPromise")]
    fn create_typed_string_promise(value: &str) -> Promise<JsString>;

    #[wasm_bindgen(js_name = "processTypedPromise")]
    fn process_typed_promise(promise: Promise<i32>) -> Promise<i32>;

    #[wasm_bindgen(js_name = "chainPromises")]
    fn chain_typed_promises(promise1: Promise<i32>, promise2: Promise<i32>) -> Promise<i32>;
}

#[wasm_bindgen]
pub fn rust_create_typed_number_promise(value: i32) -> Promise<i32> {
    Promise::resolve(&JsValue::from(value * 2)).unchecked_into()
}

#[wasm_bindgen]
pub fn rust_create_typed_string_promise(value: &str) -> Promise<JsString> {
    let js_str = JsString::from(format!("Rust: {}", value));
    Promise::resolve(&js_str).unchecked_into()
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn rust_process_typed_promise(promise: Promise<i32>) -> Promise<i32> {
    use std::sync::Once;
    static mut CLOSURE: Option<Closure<dyn FnMut(i32) -> i32>> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            CLOSURE = Some(Closure::new(|num: i32| num + 100));
        });
        promise.map(CLOSURE.as_ref().unwrap())
    }
}

#[wasm_bindgen_test]
async fn test_typed_promise_exports() {
    let typed_promise = rust_create_typed_number_promise(21);
    let typed_future: JsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result, 42);

    let typed_promise = rust_create_typed_string_promise("hello");
    let typed_future: JsFuture<JsString> = typed_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result.as_string(), Some("Rust: hello".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_processing() {
    let original_typed = rust_create_typed_number_promise(10);
    let processed_typed = rust_process_typed_promise(original_typed);

    let typed_future: JsFuture<i32> = processed_typed.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result, 120);
}

#[wasm_bindgen_test]
async fn test_typed_promise_imports() {
    let typed_promise = create_typed_number_promise(15);
    let typed_future: JsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result, 30);

    let typed_promise = create_typed_string_promise("world");
    let typed_future: JsFuture<JsString> = typed_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result.as_string(), Some("JS: world".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_promise_round_trip() {
    let rust_typed = rust_create_typed_number_promise(5);

    let js_processed = process_typed_promise(rust_typed);

    let final_typed = rust_process_typed_promise(js_processed);

    let result: i32 = JsFuture::from(final_typed).await.unwrap().from_js();

    assert_eq!(result, 310);
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_error() {
    let future = async { Err(JsValue::from("future failed")) };
    let typed_promise: Promise<i32> = future_to_promise(future);

    let typed_future: JsFuture<i32> = typed_promise.into();
    let result = typed_future.await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.as_string(), Some("future failed".to_string()));
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_async_computation() {
    let future = async {
        let base = 10i32;
        let multiplier = 4i32;
        Ok((base * multiplier).to_js())
    };

    let typed_promise = future_to_promise(future);
    let typed_future: JsFuture<i32> = typed_promise.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result, 40);
}

#[wasm_bindgen_test]
async fn test_typed_future_to_promise_chaining() {
    let future = async { Ok(5i32.to_js()) };
    let typed_promise = future_to_promise(future);

    let closure = Closure::new(|num: i32| num * 10);
    let chained = typed_promise.map(&closure);
    let typed_future: JsFuture<i32> = chained.into();
    let result = typed_future.await.unwrap().from_js();
    assert_eq!(result, 50);
}
