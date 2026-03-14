use crate::generated::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn return_promise() {
    let f = TestPromises::new().unwrap();

    #[cfg(not(wbg_next_unstable))]
    {
        let v = JsFuture::from(f.string_promise()).await.unwrap();
        let v = v.as_string().unwrap();
        assert_eq!(v, "abc");
    }

    #[cfg(wbg_next_unstable)]
    {
        let v: String = JsFuture::from(f.string_promise()).await.unwrap().into();
        assert_eq!(v, "abc");
    }
}

/// Test that `Promise<any>` returns just `Promise` (not `Promise<JsValue>`)
/// and that both generics and non-generics variants are identical (no cfg branching needed)
#[wasm_bindgen_test]
async fn return_any_promise() {
    let f = TestPromises::new().unwrap();

    // Both generics and non-generics should work the same - returns Promise (not Promise<JsValue>)
    let promise: Promise = f.any_promise();
    let v = JsFuture::from(promise).await.unwrap();

    // The resolved value is an object { foo: "bar", num: 42 }
    let foo_prop = js_sys::Reflect::get(&v, &"foo".into()).unwrap();
    assert_eq!(foo_prop.as_string().unwrap(), "bar");

    let num = js_sys::Reflect::get(&v, &"num".into()).unwrap();
    assert_eq!(num.as_f64().unwrap(), 42.0);
}

/// Test that Promise<DOMString> as an argument generates two overloads:
/// one accepting a Promise (unsuffixed) and one accepting the resolved type directly.
#[wasm_bindgen_test]
fn promise_arg_overload_with_promise() {
    let f = TestPromises::new().unwrap();

    // The canonical overload: pass a Promise<JsString>
    let p: Promise<js_sys::JsString> = Promise::resolve(&js_sys::JsString::from("hello"));
    f.wait_for_string(&p);
}

#[wasm_bindgen_test]
fn promise_arg_overload_with_value() {
    let f = TestPromises::new().unwrap();

    // The value overload: pass a &str directly
    f.wait_for_string_with_str("hello");
}

/// Test that Promise<any> as an argument generates two overloads.
#[wasm_bindgen_test]
fn promise_any_arg_overload_with_promise() {
    let f = TestPromises::new().unwrap();

    let p: Promise = Promise::resolve::<wasm_bindgen::JsValue>(&42.into());
    f.wait_for_any(&p);
}

#[wasm_bindgen_test]
fn promise_any_arg_overload_with_value() {
    let f = TestPromises::new().unwrap();

    f.wait_for_any_with_any(&42.into());
}

/// Test that optional Promise<DOMString> generates three overloads:
/// no-arg, promise (suffixed with arg name), and value.
#[wasm_bindgen_test]
fn optional_promise_arg_no_arg() {
    let f = TestPromises::new().unwrap();

    // No-arg overload (unsuffixed)
    f.maybe_wait_for_string();
}

#[wasm_bindgen_test]
fn optional_promise_arg_with_promise() {
    let f = TestPromises::new().unwrap();

    // Promise overload (suffixed with arg name since no-arg takes the canonical name)
    let p: Promise<js_sys::JsString> = Promise::resolve(&js_sys::JsString::from("hello"));
    f.maybe_wait_for_string_with_p(&p);
}

#[wasm_bindgen_test]
fn optional_promise_arg_with_value() {
    let f = TestPromises::new().unwrap();

    // Value overload
    f.maybe_wait_for_string_with_str("hello");
}

/// Test that Promise<DOMString> attribute generates a getter and typed setters
/// without trailing underscores.
#[wasm_bindgen_test]
fn promise_attribute_setter_with_promise() {
    let f = TestPromises::new().unwrap();

    // The canonical setter (unsuffixed) takes a Promise
    let p: Promise<js_sys::JsString> = Promise::resolve(&js_sys::JsString::from("hello"));
    f.set_promise_value(&p);
}

#[wasm_bindgen_test]
fn promise_attribute_setter_with_value() {
    let f = TestPromises::new().unwrap();

    // The value setter (suffixed with type name)
    f.set_promise_value_str("hello");
}
