use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsError};
use wasm_bindgen_futures::{future_to_promise_typed, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn promise_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    let promise: Promise<JsValue> = Promise::new(&mut |_, _| ());
    #[cfg(js_sys_unstable_apis)]
    let promise: Promise<JsValue> = Promise::new(
        &mut |_: Function<fn(JsValue) -> Undefined>, _: Function<fn(JsValue) -> Undefined>| (),
    );
    assert!(promise.is_instance_of::<Promise>());
    assert!(promise.is_instance_of::<Object>());
    let _: &Object = promise.as_ref();
}

#[wasm_bindgen(module = "tests/wasm/Promise.js")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type TestValue;

    #[wasm_bindgen(constructor)]
    fn new(value: &JsString) -> TestValue;

    #[wasm_bindgen(method, getter)]
    fn value(this: &TestValue) -> JsString;

    #[wasm_bindgen(method, catch)]
    fn transform(this: &TestValue, suffix: &JsString) -> Result<TestValue, JsValue>;

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
    let promise = Promise::resolve(&test_val);

    check_test_value_promise(&promise).await.unwrap();

    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "hello");
}

#[wasm_bindgen_test]
async fn test_promise_map_with_closure() {
    let test_val = TestValue::new(&JsString::from("start"));
    let promise = Promise::resolve(&test_val);

    let closure = Closure::new(|val: TestValue| {
        val.transform(&JsString::from("_mapped"))
            .map_err(|e| JsError::new(&e.as_string().unwrap_or_default()))
    });
    let result_promise = promise.then_map(&closure);

    let result: TestValue = JsFuture::from(result_promise).await.unwrap();
    assert_eq!(result.value(), "start_mapped");
}

#[wasm_bindgen_test]
async fn test_promise_map_type_transformation() {
    let test_val = TestValue::new(&JsString::from("value"));
    let promise = Promise::resolve(&test_val);

    let closure = Closure::new(|val: TestValue| Ok(TestResult::new(true, &val)));
    let result_promise = promise.then_map(&closure);

    let result = JsFuture::from(result_promise).await.unwrap();
    assert!(result.success());
}

#[wasm_bindgen_test]
async fn test_promise_then_with_closure() {
    let test_val = TestValue::new(&JsString::from("then_test"));
    let promise = Promise::resolve(&test_val);

    #[cfg(not(js_sys_unstable_apis))]
    let closure = Closure::new(|val: TestValue| {
        assert_eq!(val.value(), "then_test");
    });
    #[cfg(js_sys_unstable_apis)]
    let closure = Closure::new(|val: TestValue| {
        assert_eq!(val.value(), "then_test");
        Ok::<_, JsError>(())
    });

    #[allow(deprecated)]
    let result_promise = promise.then(&closure);

    let _ = JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_promise_catch_with_closure() {
    let error_obj = Object::new();
    js_sys::Reflect::set(
        &error_obj,
        &JsString::from("message"),
        &"error occurred".into(),
    )
    .unwrap();

    let promise: Promise<TestValue> = Promise::reject_typed(&error_obj);

    #[cfg(not(js_sys_unstable_apis))]
    let closure = Closure::new(|error: JsValue| {
        let msg = js_sys::Reflect::get(&error, &"message".into()).unwrap();
        assert_eq!(msg.as_string(), Some("error occurred".to_string()));
    });
    #[cfg(js_sys_unstable_apis)]
    let closure = Closure::new(|error: JsValue| {
        let error: Object = error.unchecked_into();
        let msg = js_sys::Reflect::get_str(&error, &"message".into())
            .unwrap()
            .unwrap();
        assert_eq!(msg.as_string(), Some("error occurred".to_string()));
        Ok::<TestValue, JsError>(TestValue::new(&JsString::from("recovered")))
    });

    #[allow(deprecated)]
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
    let promise = Promise::resolve(&test_val);
    let processed = process_test_value_promise(promise);

    let result = JsFuture::from(processed).await.unwrap();
    assert_eq!(result.value(), "original_processed");
}

#[wasm_bindgen_test]
async fn test_promise_chaining_through_js() {
    let val1 = TestValue::new(&JsString::from("first"));
    let val2 = TestValue::new(&JsString::from("second"));

    let promise1 = Promise::resolve(&val1);
    let promise2 = Promise::resolve(&val2);
    let chained = chain_test_value_promises(promise1, promise2);

    let result = JsFuture::from(chained).await.unwrap();
    assert_eq!(result.value(), "first+second");
}

#[wasm_bindgen]
pub fn rust_create_test_value_promise(value: &str) -> Promise<TestValue> {
    let test_val = TestValue::new(&JsString::from(format!("rust:{}", value)));
    Promise::resolve(&test_val)
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn rust_process_test_value_promise(promise: Promise<TestValue>) -> Promise<TestValue> {
    use std::sync::Once;
    static mut CLOSURE: Option<Closure<dyn FnMut(TestValue) -> Result<TestValue, JsError>>> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            CLOSURE = Some(Closure::new(|val: TestValue| {
                val.transform(&JsString::from("_rust_processed"))
                    .map_err(|e| JsError::new(&e.as_string().unwrap_or_default()))
            }));
        });
        promise.then_map(CLOSURE.as_ref().unwrap())
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
        Ok(val)
    };

    let promise: Promise<TestValue> = future_to_promise_typed(future);
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "async_value");
}

#[wasm_bindgen_test]
async fn test_future_to_promise_error() {
    let future = async { Err(JsValue::from("future failed")) };
    let promise = future_to_promise_typed::<TestValue, _>(future);

    let result = JsFuture::from(promise).await;
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.as_string(), Some("future failed".to_string()));
}

#[wasm_bindgen_test]
async fn test_future_to_promise_chaining_with_closure() {
    let future = async {
        let val = TestValue::new(&JsString::from("chained"));
        Ok(val)
    };

    let promise = future_to_promise_typed(future);
    let closure = Closure::new(|val: TestValue| {
        val.transform(&JsString::from("_then"))
            .map_err(|e| JsError::new(&e.as_string().unwrap_or_default()))
    });
    let chained = promise.then_map(&closure);

    let result: TestValue = JsFuture::from(chained).await.unwrap();
    assert_eq!(result.value(), "chained_then");
}

#[wasm_bindgen_test]
async fn test_future_to_promise_async_computation() {
    let future = async {
        let base = TestValue::new(&JsString::from("computed"));
        let transformed = base.transform(&JsString::from("_async"))?;
        Ok(transformed)
    };

    let promise = future_to_promise_typed(future);
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value(), "computed_async");
}

#[wasm_bindgen_test]
async fn test_promise_finally() {
    let test_val = TestValue::new(&JsString::from("finally_test"));
    let promise: Promise<TestValue> = Promise::resolve(&test_val);

    let closure = Closure::new(|| {});

    #[allow(deprecated)]
    let result_promise = promise.finally(&closure);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "finally_test");
}

#[wasm_bindgen_test]
async fn test_promise_then2() {
    let test_val = TestValue::new(&JsString::from("then2_test"));
    let promise: Promise<TestValue> = Promise::resolve(&test_val);

    let resolve = Closure::new(|val: TestValue| {
        assert_eq!(val.value(), "then2_test");
        Ok(())
    });
    let reject = Closure::new(|_err: JsValue| {
        panic!("should not reject");
    });
    let result_promise = promise.then_with_reject(&resolve, &reject);

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
        Ok(())
    });
    let result_promise = promise.then_with_reject(&resolve, &reject);

    JsFuture::from(result_promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_promise_map2() {
    let test_val = TestValue::new(&JsString::from("map2_test"));
    let promise: Promise<TestValue> = Promise::resolve(&test_val);

    let resolve = Closure::new(|val: TestValue| {
        val.transform(&JsString::from("_resolved"))
            .map_err(|e| JsError::new(&e.as_string().unwrap_or_default()))
    });
    let reject =
        Closure::new(|_err: JsValue| Ok::<_, JsError>(TestValue::new(&JsString::from("rejected"))));
    let result_promise: Promise<TestValue> = promise.then_with_reject(&resolve, &reject);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "map2_test_resolved");
}

#[wasm_bindgen_test]
async fn test_promise_map2_reject() {
    let promise: Promise<TestValue> = Promise::reject_typed(&JsValue::from("error"));

    let resolve = Closure::new(|val: TestValue| {
        val.transform(&JsString::from("_resolved"))
            .map_err(|e| JsError::new(&e.as_string().unwrap_or_default()))
    });
    let reject = Closure::new(|_err: JsValue| {
        Ok::<_, JsError>(TestValue::new(&JsString::from("recovered")))
    });
    let result_promise: Promise<TestValue> = promise.then_with_reject(&resolve, &reject);

    let result = JsFuture::from(result_promise).await.unwrap();
    let result: TestValue = result.unchecked_into();
    assert_eq!(result.value(), "recovered");
}

#[wasm_bindgen_test]
async fn all_settled_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve(&Number::from(1)));
    arr.push(&Promise::resolve(&Number::from(2)));
    arr.push(&Promise::reject_typed(&JsValue::from("error")));

    let result_promise = Promise::all_settled_iterable(&arr);
    let result = JsFuture::from(result_promise).await.unwrap();
    let states: Array<PromiseState<Number>> = result.unchecked_into();

    assert_eq!(states.length(), 3);
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert!(states.get(0).is_fulfilled());
        assert!(states.get(1).is_fulfilled());
        assert!(states.get(2).is_rejected());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert!(states.get(0).unwrap().is_fulfilled());
        assert!(states.get(1).unwrap().is_fulfilled());
        assert!(states.get(2).unwrap().is_rejected());
    }
}

#[wasm_bindgen_test]
fn any_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve(&Number::from(42)));
    arr.push(&Promise::resolve(&Number::from(100)));

    #[cfg(not(js_sys_unstable_apis))]
    let result = Promise::any_iterable(&arr);
    #[cfg(js_sys_unstable_apis)]
    let result = Promise::any(&arr);
    assert!(JsValue::from(result).is_object());
}

#[wasm_bindgen_test]
fn race_iterable() {
    let arr: Array<Promise<Number>> = Array::new_typed();
    arr.push(&Promise::resolve(&Number::from(1)));
    arr.push(&Promise::resolve(&Number::from(2)));

    let result = Promise::race_iterable(&arr);
    assert!(JsValue::from(result).is_object());
}
#[wasm_bindgen]
pub fn rust_create_number_promise(value: f64) -> Promise<Number> {
    Promise::resolve(&Number::from(value))
}

#[wasm_bindgen]
pub fn rust_create_string_promise(value: &str) -> Promise<JsString> {
    Promise::resolve(&JsString::from(value))
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn rust_double_number_promise(promise: Promise<Number>) -> Promise<Number> {
    use std::sync::Once;
    static mut CLOSURE: Option<Closure<dyn FnMut(Number) -> Result<Number, JsError>>> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            CLOSURE = Some(Closure::new(|n: Number| {
                Ok(Number::from(n.value_of() * 2.0))
            }));
        });
        promise.then_map(CLOSURE.as_ref().unwrap())
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
    let promise = Promise::resolve(&Number::from(21.0));
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

#[wasm_bindgen_test]
async fn covariance() {
    use wasm_bindgen::prelude::Upcast;

    async fn accepts_jsvalue_promise(promise: Promise<JsValue>) -> JsValue {
        JsFuture::from(promise).await.unwrap()
    }

    let number_promise = Promise::resolve(&Number::from(42.0));

    let result = accepts_jsvalue_promise(number_promise.upcast_into()).await;
    let num: Number = result.unchecked_into();
    assert_eq!(num.value_of(), 42.0);

    let string_promise = Promise::resolve(&JsString::from("hello"));
    let result = accepts_jsvalue_promise(string_promise.upcast_into()).await;
    let s: JsString = result.unchecked_into();
    assert_eq!(s, JsString::from("hello"));
}

#[wasm_bindgen_test]
async fn rust_string_element_upcasts() {
    use wasm_bindgen::prelude::Upcast;
    use wasm_bindgen::JsGeneric;

    fn is_js_generic<T: JsGeneric>(_: &T) {}

    async fn accepts_jsvalue_promise(promise: Promise<JsValue>) -> JsValue {
        JsFuture::from(promise).await.unwrap()
    }
    async fn accepts_js_string_promise(promise: Promise<JsString>) -> JsString {
        JsFuture::from(promise).await.unwrap().unchecked_into()
    }

    let promise: Promise<String> = Promise::resolve(&JsString::from("owned")).unchecked_into();
    is_js_generic(&promise);
    let s = accepts_js_string_promise(promise.clone().upcast_into()).await;
    assert_eq!(s, JsString::from("owned"));
    let value = accepts_jsvalue_promise(promise.upcast_into()).await;
    assert_eq!(value.as_string().as_deref(), Some("owned"));

    let promise: Promise<&str> = Promise::resolve(&JsString::from("borrowed")).unchecked_into();
    let value = accepts_jsvalue_promise(promise.upcast_into()).await;
    assert_eq!(value.as_string().as_deref(), Some("borrowed"));
}

#[wasm_bindgen_test]
#[cfg(js_sys_unstable_apis)]
async fn test_promise_all_with_upcast() {
    // Create an array of Promise<TestValue> where TestValue extends Object
    let arr: Array<Promise<TestValue>> = Array::new_typed();
    arr.push(&Promise::resolve(&TestValue::new(&JsString::from("first"))));
    arr.push(&Promise::resolve(&TestValue::new(&JsString::from(
        "second",
    ))));
    arr.push(&Promise::resolve(&TestValue::new(&JsString::from("third"))));

    // Promise::all should accept Array<Promise<TestValue>> and return Promise<Array<JsValue>>
    // because TestValue: Upcast<JsValue> (via extends = Object)
    let result_promise = Promise::all(&arr);
    let result_promise: Promise<Array<JsValue>> = result_promise.upcast_into();
    let result_array: Array<JsValue> = JsFuture::from(result_promise).await.unwrap();

    assert_eq!(result_array.length(), 3);

    // The values should still be TestValue instances, just typed as JsValue
    let first: TestValue = result_array.get(0).unwrap().unchecked_into();
    let second: TestValue = result_array.get(1).unwrap().unchecked_into();
    let third: TestValue = result_array.get(2).unwrap().unchecked_into();

    assert_eq!(first.value(), "first");
    assert_eq!(second.value(), "second");
    assert_eq!(third.value(), "third");
}

fn promise_has(name: &str) -> bool {
    Reflect::get_str(global().as_ref(), &"Promise".into())
        .ok()
        .flatten()
        .map(|p| Reflect::has_str(p.unchecked_ref::<Object>(), &name.into()).unwrap_or(false))
        .unwrap_or(false)
}

#[wasm_bindgen_test]
async fn try_resolves() {
    if !promise_has("try") {
        return;
    }
    let mut calls = 0;
    let promise: Promise<Number> = Promise::try_(&mut || {
        calls += 1;
        Ok::<_, JsError>(Number::from(42))
    });
    assert_eq!(calls, 1);
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value_of(), 42.0);

    let promise: Promise<JsString> = Promise::try_(&mut || Ok::<_, JsError>(JsString::from("s")));
    assert_eq!(JsFuture::from(promise).await.unwrap(), "s");
}

#[wasm_bindgen_test]
async fn try_rejects() {
    if !promise_has("try") {
        return;
    }
    let promise: Promise<Number> = Promise::try_(&mut || Err::<Number, _>(JsError::new("boom")));
    let err = JsFuture::from(promise).await.unwrap_err();
    assert!(err.is_instance_of::<Error>());
    assert_eq!(err.unchecked_into::<Error>().message(), "boom");
}

#[wasm_bindgen_test]
async fn try_adopts_promise() {
    if !promise_has("try") {
        return;
    }
    let promise: Promise<Number> =
        Promise::try_(&mut || Ok::<_, JsError>(Promise::resolve(&Number::from(7))));
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value_of(), 7.0);

    let promise: Promise<Number> = Promise::try_(&mut || {
        Ok::<_, JsError>(Promise::<Number>::reject_typed(&JsValue::from("inner")))
    });
    let err = JsFuture::from(promise).await.unwrap_err();
    assert_eq!(err.as_string(), Some("inner".to_string()));
}

#[wasm_bindgen_test]
async fn with_resolvers_resolve() {
    if !promise_has("withResolvers") {
        return;
    }
    let resolvers = Promise::with_resolvers::<Number, JsValue>();
    assert!(resolvers.is_instance_of::<Object>());
    let promise: Promise<Number> = resolvers.promise();
    assert!(promise.is_instance_of::<Promise>());

    let resolve: Function<fn(Number) -> Undefined> = resolvers.resolve();
    let ret = resolve.call(&JsValue::NULL, (&Number::from(7),)).unwrap();
    assert!(JsValue::from(ret).is_undefined());

    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.value_of(), 7.0);
}

#[wasm_bindgen_test]
async fn with_resolvers_reject() {
    if !promise_has("withResolvers") {
        return;
    }
    let resolvers = Promise::with_resolvers::<JsString, JsValue>();
    let reject: Function<fn(JsValue) -> Undefined> = resolvers.reject();
    reject.call1(&JsValue::NULL, &JsValue::from("bad")).unwrap();

    let err = JsFuture::from(resolvers.promise()).await.unwrap_err();
    assert_eq!(err.as_string(), Some("bad".to_string()));
}

// Typed rejections: `Promise<T, E>`.
#[wasm_bindgen(module = "tests/wasm/Promise.js")]
extern "C" {
    #[wasm_bindgen(js_name = stringOrTypeError)]
    fn string_or_type_error(value: &str, fail: bool) -> Promise<String, TypeError>;

    // The same binding through erased generics in both parameters.
    #[wasm_bindgen(js_name = stringOrTypeError)]
    fn generic_or_type_error<T, E>(value: T, fail: bool) -> Promise<T, E>;

    #[wasm_bindgen(js_name = rejectTypeError)]
    fn reject_type_error(message: &str) -> Promise<Number, TypeError>;
}

#[wasm_bindgen_test]
async fn typed_rejection_awaits_err() {
    assert_eq!(string_or_type_error("v", false).await.unwrap(), "v");

    let err: TypeError = string_or_type_error("bad", true).await.unwrap_err();
    assert!(err.is_instance_of::<TypeError>());
    assert_eq!(err.message(), "bad");

    let fut: JsFuture<String, TypeError> = JsFuture::from(string_or_type_error("f", true));
    assert_eq!(fut.await.unwrap_err().message(), "f");
}

#[wasm_bindgen_test]
async fn typed_rejection_generic_import() {
    let promise: Promise<JsString, TypeError> = generic_or_type_error(JsString::from("v"), false);
    assert_eq!(promise.await.unwrap(), "v");

    let promise: Promise<JsString, TypeError> = generic_or_type_error(JsString::from("g"), true);
    assert_eq!(promise.await.unwrap_err().message(), "g");
}

#[wasm_bindgen_test]
async fn typed_rejection_round_trip() {
    if !promise_has("withResolvers") {
        return;
    }
    let resolvers = Promise::with_resolvers::<Number, TypeError>();
    let promise: Promise<Number, TypeError> = resolvers.promise();
    let resolve: Function<fn(Number) -> Undefined> = resolvers.resolve();
    resolve.call(&JsValue::NULL, (&Number::from(7),)).unwrap();
    assert_eq!(promise.await.unwrap().value_of(), 7.0);

    let resolvers = Promise::with_resolvers::<Number, TypeError>();
    let reject: Function<fn(TypeError) -> Undefined> = resolvers.reject();
    reject
        .call(&JsValue::NULL, (&TypeError::new("nope"),))
        .unwrap();
    let err: TypeError = resolvers.promise().await.unwrap_err();
    assert_eq!(err.message(), "nope");
}

// `PromiseWithResolvers` is invariant in `T` and `E`, but still upcasts to its
// JS supertypes.
#[wasm_bindgen_test]
fn resolvers_upcast_to_supertypes() {
    use wasm_bindgen::prelude::Upcast;

    if !promise_has("withResolvers") {
        return;
    }
    let resolvers = Promise::with_resolvers::<Number, TypeError>();
    let same: PromiseWithResolvers<Number, TypeError> = resolvers.clone().upcast_into();
    let object: Object = resolvers.clone().upcast_into();
    let value: JsValue = resolvers.upcast_into();
    assert!(same.is_instance_of::<Object>());
    assert!(object.is_instance_of::<Object>());
    assert!(value.is_object());
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
async fn typed_rejection_constructors() {
    let promise: Promise<Number, TypeError> =
        Promise::new(
            &mut |_: Function<fn(Number) -> Undefined>,
                  reject: Function<fn(TypeError) -> Undefined>| {
                reject
                    .call(&JsValue::NULL, (&TypeError::new("exec"),))
                    .unwrap();
            },
        );
    assert_eq!(promise.await.unwrap_err().message(), "exec");

    let promise: Promise<Number, TypeError> = Promise::reject(&TypeError::new("r"));
    assert_eq!(promise.await.unwrap_err().message(), "r");
}

#[wasm_bindgen_test]
async fn typed_rejection_upcasts() {
    use wasm_bindgen::prelude::Upcast;

    let promise = reject_type_error("up");

    let as_error: Promise<Number, Error> = promise.clone().upcast_into();
    assert_eq!(as_error.await.unwrap_err().message(), "up");

    let as_default: Promise<Number> = promise.clone().upcast_into();
    assert!(as_default.await.unwrap_err().is_instance_of::<TypeError>());

    let as_js: Promise<JsValue, JsValue> = promise.clone().upcast_into();
    assert!(as_js.await.unwrap_err().is_instance_of::<TypeError>());

    let future: JsFuture<JsValue, Error> = JsFuture::from(promise).upcast_into();
    assert_eq!(future.await.unwrap_err().message(), "up");
}

#[wasm_bindgen_test]
async fn typed_rejection_handlers() {
    #[cfg(not(js_sys_unstable_apis))]
    let closure = Closure::new(|err: TypeError| assert_eq!(err.message(), "c"));
    #[cfg(js_sys_unstable_apis)]
    let closure = Closure::new(|err: TypeError| {
        assert_eq!(err.message(), "c");
        Ok::<Number, JsError>(Number::from(0))
    });
    reject_type_error("c").catch(&closure).await.unwrap();

    let resolve = Closure::new(|_: Number| Ok::<_, JsError>(Number::from(0)));
    let reject =
        Closure::new(|err: TypeError| Ok::<_, JsError>(Number::from(err.message().length())));
    let recovered: Promise<Number> = reject_type_error("abc").then_with_reject(&resolve, &reject);
    assert_eq!(recovered.await.unwrap().value_of(), 3.0);

    // `then` and `finally` without a rejection handler pass the rejection
    // through as `E`.
    #[cfg(not(js_sys_unstable_apis))]
    let on_resolve = Closure::new(|_: Number| panic!("should not resolve"));
    #[cfg(not(js_sys_unstable_apis))]
    let chained: Promise<JsValue, TypeError> = reject_type_error("t").then(&on_resolve);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(chained.await.unwrap_err().message(), "t");

    let on_finally = Closure::new(|| ());
    let chained: Promise<JsValue, TypeError> = reject_type_error("f").finally(&on_finally);
    assert_eq!(chained.await.unwrap_err().message(), "f");
}

#[wasm_bindgen_test]
async fn typed_rejection_combinators() {
    let joined = Promise::all_tuple((reject_type_error("a"), reject_type_error("b")));
    let err: TypeError = joined.await.unwrap_err();
    assert_eq!(err.message(), "a");

    let settled = Promise::all_settled_tuple((reject_type_error("s"),))
        .await
        .unwrap();
    let (state,) = settled.into_tuple();
    assert!(state.is_rejected());
    let reason: TypeError = state.get_reason().unwrap();
    assert_eq!(reason.message(), "s");
    let result: Result<Number, TypeError> = state.into();
    assert_eq!(result.unwrap_err().message(), "s");

    let arr: Array<Promise<Number, TypeError>> = Array::new_typed();
    arr.push(&reject_type_error("x"));
    arr.push(&reject_type_error("y"));

    #[cfg(js_sys_unstable_apis)]
    {
        let err: AggregateError = Promise::any(&arr).await.unwrap_err();
        assert_eq!(err.errors().length(), 2);
    }

    // The stable iterable combinators widen the rejection type to `JsValue`.
    let err: JsValue = Promise::any_iterable(&arr).await.unwrap_err();
    assert!(err.is_instance_of::<AggregateError>());
    let err: JsValue = Promise::all_iterable(&arr).await.unwrap_err();
    assert!(err.is_instance_of::<TypeError>());
    let err: JsValue = Promise::race_iterable(&arr).await.unwrap_err();
    assert!(err.is_instance_of::<TypeError>());
}
