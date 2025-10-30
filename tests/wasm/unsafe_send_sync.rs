//! Tests for the unsafe-single-threaded-traits feature
//!
//! This test file verifies that when the feature is enabled, JS types
//! become Send and Sync, allowing futures to be Send.

#![cfg(feature = "unsafe-single-threaded-traits")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

#[wasm_bindgen_test]
fn jsvalue_is_send_and_sync() {
    assert_send::<JsValue>();
    assert_sync::<JsValue>();
}

#[wasm_bindgen_test]
fn closure_is_send_and_sync() {
    assert_send::<Closure<dyn Fn()>>();
    assert_sync::<Closure<dyn Fn()>>();

    assert_send::<Closure<dyn FnMut()>>();
    assert_sync::<Closure<dyn FnMut()>>();
}

#[wasm_bindgen_test]
fn closure_capturing_jsvalue_is_send() {
    fn check_send<T: Send>(_: T) {}

    let js_val = JsValue::from_str("captured");
    let closure = Closure::wrap(Box::new(move || {
        let _ = &js_val;
    }) as Box<dyn Fn()>);

    check_send(closure);
}

#[wasm_bindgen_test]
fn jsfuture_is_send_and_sync() {
    assert_send::<JsFuture>();
    assert_sync::<JsFuture>();
}

#[wasm_bindgen_test]
fn future_with_jsvalue_is_send() {
    fn check_future_is_send<F: Send>(_f: F) {}

    let future = async {
        let js_val = JsValue::from_str("test");

        js_val
    };

    check_future_is_send(future);
}

#[wasm_bindgen_test]
fn future_holding_jsvalue_across_await_is_send() {
    fn check_send<F: Send>(_: F) {}

    let captured_val = JsValue::from_str("outer");

    let future = async move {
        let js_val = JsValue::from_str("inner");

        std::future::ready(()).await;

        (captured_val, js_val)
    };

    check_send(future);
}

#[wasm_bindgen]
extern "C" {
    type ExternalJsType;
}

#[wasm_bindgen_test]
fn external_types_are_send_and_sync() {
    assert_send::<ExternalJsType>();
    assert_sync::<ExternalJsType>();
}

#[wasm_bindgen]
pub struct SendSyncTestStruct {
    _value: JsValue,
}

#[wasm_bindgen_test]
fn custom_types_with_jsvalue_are_send() {
    assert_send::<SendSyncTestStruct>();
    assert_sync::<SendSyncTestStruct>();
}
