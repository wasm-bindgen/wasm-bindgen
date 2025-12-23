use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise_typed, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn promise_inheritance() {
    let promise = Promise::new(&mut |_, _| ());
    assert!(promise.is_instance_of::<Promise>());
    assert!(promise.is_instance_of::<Object>());
    let _: &Object = promise.as_ref();
}

#[wasm_bindgen(module = "tests/wasm/Promise.js")]
extern "C" {
    pub type TestValue;

    #[wasm_bindgen(constructor)]
    fn new(value: &JsString) -> TestValue;

    #[wasm_bindgen(method, getter)]
    fn value(this: &TestValue) -> JsString;

    #[wasm_bindgen(method)]
    fn transform(this: &TestValue, suffix: &JsString) -> TestValue;

    type TestResult;

    #[wasm_bindgen(constructor)]
    fn new(success: bool, data: &TestValue) -> TestResult;

    #[wasm_bindgen(method, getter)]
    fn success(this: &TestResult) -> bool;

    #[wasm_bindgen(method, getter)]
    fn data(this: &TestResult) -> TestResult;
}

#[wasm_bindgen(module = "tests/wasm/Promise.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createTestValuePromise")]
    fn create_test_value_promise(value: &str) -> Promise<TestValue>;

    #[wasm_bindgen(js_name = "createTestResultPromise")]
    fn create_test_result_promise(success: bool, value: &str) -> Promise<TestResult>;

    #[wasm_bindgen(js_name = "processTestValuePromise")]
    fn process_test_value_promise(promise: Promise<TestValue>) -> Promise<TestValue>;

    #[wasm_bindgen(js_name = "chainTestValuePromises")]
    fn chain_test_value_promises(
        promise1: Promise<TestValue>,
        promise2: Promise<TestValue>,
    ) -> Promise<TestValue>;

    #[wasm_bindgen(js_name = "checkTestValuePromise", catch)]
    async fn check_test_value_promise(p: &Promise<TestValue>) -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
async fn test_promise_resolve_infers_type() {
    let test_val = TestValue::new(&JsString::from("hello"));
    let promise = Promise::resolve_typed(&test_val);

    check_test_value_promise(&promise).await.unwrap();

    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "hello");
}

#[wasm_bindgen_test]
async fn test_promise_map_with_closure() {
    let test_val = TestValue::new(&JsString::from("start"));
    let promise = Promise::resolve_typed(&test_val);

    let closure = Closure::new(|val: TestValue| val.transform(&JsString::from("_mapped")));
    let result_promise = promise.map(&closure);

    let result = JsFuture::from(result_promise).await.unwrap();
    assert_eq!(result.value(), "start_mapped");
}

#[wasm_bindgen_test]
async fn test_promise_map_type_transformation() {
    let test_val = TestValue::new(&JsString::from("value"));
    let promise = Promise::resolve_typed(&test_val);

    let closure = Closure::new(|val: TestValue| TestResult::new(true, &val));
    let result_promise = promise.map(&closure);

    let result = JsFuture::from(result_promise).await.unwrap();
    assert!(result.success());
}

#[wasm_bindgen_test]
async fn test_promise_then_with_closure() {
    let test_val = TestValue::new(&JsString::from("then_test"));
    let promise = Promise::resolve_typed(&test_val);

    let closure = Closure::new(|val: TestValue| {
        assert_eq!(val.value(), "then_test");
    });
    let result_promise = promise.then(&closure);

    JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_promise_catch_with_closure() {
    let error_obj = Object::new();
    js_sys::Reflect::set(&error_obj, &"message".into(), &"error occurred".into()).unwrap();

    let promise: Promise<TestValue> = Promise::reject_typed(&error_obj);

    let closure = Closure::new(|error: JsValue| {
        let msg = js_sys::Reflect::get(&error, &"message".into()).unwrap();
        assert_eq!(msg.as_string(), Some("error occurred".to_string()));
    });
    let result_promise = promise.catch(&closure);

    JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_imported_promise_function() {
    let promise = create_test_value_promise("from_js");
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "from_js");
}

#[wasm_bindgen_test]
async fn test_imported_promise_with_nested_type() {
    let promise = create_test_result_promise(true, "nested");
    let result = JsFuture::from(promise).await.unwrap();
    assert!(result.success());
}

#[wasm_bindgen_test]
async fn test_promise_processing_through_js() {
    let test_val = TestValue::new(&JsString::from("original"));
    let promise = Promise::resolve_typed(&test_val);
    let processed = process_test_value_promise(promise);

    let result = JsFuture::from(processed).await.unwrap();
    assert_eq!(result.value(), "original_processed");
}

#[wasm_bindgen_test]
async fn test_promise_chaining_through_js() {
    let val1 = TestValue::new(&JsString::from("first"));
    let val2 = TestValue::new(&JsString::from("second"));

    let promise1 = Promise::resolve_typed(&val1);
    let promise2 = Promise::resolve_typed(&val2);
    let chained = chain_test_value_promises(promise1, promise2);

    let result = JsFuture::from(chained).await.unwrap();
    assert_eq!(result.value(), "first+second");
}

#[wasm_bindgen]
pub fn rust_create_test_value_promise(value: &str) -> Promise<TestValue> {
    let test_val = TestValue::new(&JsString::from(format!("rust:{}", value)));
    Promise::resolve_typed(&test_val)
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn rust_process_test_value_promise(promise: Promise<TestValue>) -> Promise<TestValue> {
    use std::sync::Once;
    static mut CLOSURE: Option<Closure<dyn FnMut(TestValue) -> TestValue>> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            CLOSURE = Some(Closure::new(|val: TestValue| {
                val.transform(&JsString::from("_rust_processed"))
            }));
        });
        promise.map(CLOSURE.as_ref().unwrap())
    }
}

#[wasm_bindgen_test]
async fn test_rust_exported_promise() {
    let promise = rust_create_test_value_promise("exported");
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "rust:exported");
}

#[wasm_bindgen_test]
async fn test_rust_promise_processing() {
    let original = rust_create_test_value_promise("input");
    let processed = rust_process_test_value_promise(original);

    let result = JsFuture::from(processed).await.unwrap();
    assert_eq!(result.value(), "rust:input_rust_processed");
}

#[wasm_bindgen_test]
async fn test_round_trip_rust_js_rust() {
    let rust_promise = rust_create_test_value_promise("round_trip");
    let js_processed = process_test_value_promise(rust_promise);
    let final_promise = rust_process_test_value_promise(js_processed);

    let result = JsFuture::from(final_promise).await.unwrap();
    assert_eq!(result.value(), "rust:round_trip_processed_rust_processed");
}

#[wasm_bindgen_test]
async fn test_future_to_promise_success() {
    let future = async {
        let val = TestValue::new(&JsString::from("async_value"));
        Ok(val.into())
    };

    let promise: Promise<TestValue> = future_to_promise_typed(future);
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "async_value");
}

#[wasm_bindgen_test]
async fn test_future_to_promise_error() {
    let future = async { Err(JsValue::from("future failed")) };
    let promise: Promise<TestValue> = future_to_promise_typed(future);

    let result = JsFuture::from(promise).await;
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.as_string(), Some("future failed".to_string()));
}

#[wasm_bindgen_test]
async fn test_future_to_promise_chaining_with_closure() {
    let future = async {
        let val = TestValue::new(&JsString::from("chained"));
        Ok(val.into())
    };

    let promise: Promise<TestValue> = future_to_promise_typed(future);
    let closure = Closure::new(|val: TestValue| val.transform(&JsString::from("_then")));
    let chained = promise.map(&closure);

    let result = JsFuture::from(chained).await.unwrap();
    assert_eq!(result.value(), "chained_then");
}

#[wasm_bindgen_test]
async fn test_future_to_promise_async_computation() {
    let future = async {
        let base = TestValue::new(&JsString::from("computed"));
        let transformed = base.transform(&JsString::from("_async"));
        Ok(transformed.into())
    };

    let promise: Promise<TestValue> = future_to_promise_typed(future);
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "computed_async");
}

#[wasm_bindgen_test]
async fn test_promise_finally() {
    let test_val = TestValue::new(&JsString::from("finally_test"));
    let promise: Promise<TestValue> = Promise::resolve_typed(&test_val);

    let closure = Closure::new(|| {});
    let result_promise = promise.finally(&closure);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "finally_test");
}

#[wasm_bindgen_test]
async fn test_promise_then2() {
    let test_val = TestValue::new(&JsString::from("then2_test"));
    let promise: Promise<TestValue> = Promise::resolve_typed(&test_val);

    let resolve = Closure::new(|val: TestValue| {
        assert_eq!(val.value(), "then2_test");
    });
    let reject = Closure::new(|_err: JsValue| {
        panic!("should not reject");
    });
    let result_promise = promise.then2(&resolve, &reject);

    JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_promise_then2_reject() {
    let promise: Promise<TestValue> = Promise::reject_typed(&JsValue::from("error"));

    let resolve = Closure::new(|_val: TestValue| {
        panic!("should not resolve");
    });
    let reject = Closure::new(|err: JsValue| {
        assert_eq!(err.as_string(), Some("error".to_string()));
    });
    let result_promise = promise.then2(&resolve, &reject);

    JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_promise_map2() {
    let test_val = TestValue::new(&JsString::from("map2_test"));
    let promise: Promise<TestValue> = Promise::resolve_typed(&test_val);

    let resolve = Closure::new(|val: TestValue| val.transform(&JsString::from("_resolved")));
    let reject = Closure::new(|_err: JsValue| TestValue::new(&JsString::from("rejected")));
    let result_promise: Promise<TestValue> = promise.map2(&resolve, &reject);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "map2_test_resolved");
}

#[wasm_bindgen_test]
async fn test_promise_map2_reject() {
    let promise: Promise<TestValue> = Promise::reject_typed(&JsValue::from("error"));

    let resolve = Closure::new(|val: TestValue| val.transform(&JsString::from("_resolved")));
    let reject = Closure::new(|_err: JsValue| TestValue::new(&JsString::from("recovered")));
    let result_promise: Promise<TestValue> = promise.map2(&resolve, &reject);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "recovered");
}

#[wasm_bindgen_test]
async fn all_settled_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve_typed(&Number::from(1)));
    arr.push(&Promise::resolve_typed(&Number::from(2)));
    arr.push(&Promise::reject_typed(&JsValue::from("error")));

    let result_promise = Promise::all_settled_iterable::<Number, _, _>(&arr);
    let result = JsFuture::from(result_promise).await.unwrap();
    let states: Array<PromiseState<Number>> = result.unchecked_into();

    assert_eq!(states.length(), 3);
    assert!(states.get(0).is_fulfilled());
    assert!(states.get(1).is_fulfilled());
    assert!(states.get(2).is_rejected());
}

#[wasm_bindgen_test]
fn any_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve_typed(&Number::from(42)));
    arr.push(&Promise::resolve_typed(&Number::from(100)));

    let result = Promise::any_iterable::<Number, _, _>(&arr);
    assert!(JsValue::from(result).is_object());
}

#[wasm_bindgen_test]
fn race_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve_typed(&Number::from(1)));
    arr.push(&Promise::resolve_typed(&Number::from(2)));

    let result = Promise::race_iterable::<Number, _, _>(&arr);
    assert!(JsValue::from(result).is_object());
}

// Exported functions with standard js-sys types
#[wasm_bindgen]
pub fn rust_create_number_promise(value: f64) -> Promise<Number> {
    Promise::resolve_typed(&Number::from(value))
}

#[wasm_bindgen]
pub fn rust_create_string_promise(value: &str) -> Promise<JsString> {
    Promise::resolve_typed(&JsString::from(value))
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn rust_double_number_promise(promise: Promise<Number>) -> Promise<Number> {
    use std::sync::Once;
    static mut CLOSURE: Option<Closure<dyn FnMut(Number) -> Number>> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            CLOSURE = Some(Closure::new(|n: Number| Number::from(n.value_of() * 2.0)));
        });
        promise.map(CLOSURE.as_ref().unwrap())
    }
}

#[wasm_bindgen_test]
async fn rust_export_number_promise() {
    let promise = rust_create_number_promise(42.0);
    let result = JsFuture::from(promise).await.unwrap();
    let num: Number = result.unchecked_into();
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
async fn rust_export_string_promise() {
    let promise = rust_create_string_promise("hello");
    let result = JsFuture::from(promise).await.unwrap();
    let s: JsString = result.unchecked_into();
    assert_eq!(s, JsString::from("hello"));
}

#[wasm_bindgen_test]
async fn rust_export_transform_number_promise() {
    let promise = Promise::resolve_typed(&Number::from(21.0));
    let doubled = rust_double_number_promise(promise);
    let result = JsFuture::from(doubled).await.unwrap();
    let num: Number = result.unchecked_into();
    assert_eq!(num.value_of(), 42.0);
}

#[wasm_bindgen_test]
async fn rust_export_number_promise_chaining() {
    let promise = rust_create_number_promise(10.0);
    let doubled = rust_double_number_promise(promise);
    let quadrupled = rust_double_number_promise(doubled);
    let result = JsFuture::from(quadrupled).await.unwrap();
    let num: Number = result.unchecked_into();
    assert_eq!(num.value_of(), 40.0);
}
