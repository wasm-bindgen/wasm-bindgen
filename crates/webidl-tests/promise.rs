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
