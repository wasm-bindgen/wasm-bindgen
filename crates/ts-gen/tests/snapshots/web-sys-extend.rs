#[allow(unused_imports)]
use js_sys::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[doc = r" Extension trait for awaiting `js_sys::Promise<T>`."]
#[doc = r""]
#[doc = r" Since `IntoFuture` can't be implemented for `js_sys::Promise` from"]
#[doc = r" generated code (orphan rule), use `.into_future().await` instead:"]
#[doc = r" ```ignore"]
#[doc = r" use bindings::PromiseExt;"]
#[doc = r" let data: ArrayBuffer = promise.into_future().await?;"]
#[doc = r" ```"]
#[allow(dead_code)]
pub trait PromiseExt {
    type Output;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<Self::Output>;
}
impl<T: 'static + wasm_bindgen::convert::FromWasmAbi> PromiseExt for js_sys::Promise<T> {
    type Output = T;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<T> {
        wasm_bindgen_futures::JsFuture::from(self)
    }
}
#[allow(dead_code)]
use ::web_sys::Request;
#[allow(dead_code)]
use ::web_sys::RequestInit;
#[allow(dead_code)]
use ::web_sys::Response;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = RequestInit , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FetchOptions;
    #[doc = " Number of times to retry on failure."]
    #[wasm_bindgen(method, getter)]
    pub fn retries(this: &FetchOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_retries(this: &FetchOptions, val: f64);
    #[doc = " Timeout in milliseconds."]
    #[wasm_bindgen(method, getter)]
    pub fn timeout(this: &FetchOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_timeout(this: &FetchOptions, val: f64);
    #[doc = " Custom priority hint."]
    #[wasm_bindgen(method, getter)]
    pub fn priority(this: &FetchOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_priority(this: &FetchOptions, val: &str);
}
impl FetchOptions {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> FetchOptionsBuilder {
        FetchOptionsBuilder { inner: Self::new() }
    }
}
pub struct FetchOptionsBuilder {
    inner: FetchOptions,
}
#[allow(unused_mut)]
impl FetchOptionsBuilder {
    pub fn retries(mut self, val: f64) -> Self {
        self.inner.set_retries(val);
        self
    }
    pub fn timeout(mut self, val: f64) -> Self {
        self.inner.set_timeout(val);
        self
    }
    pub fn priority(mut self, val: &str) -> Self {
        self.inner.set_priority(val);
        self
    }
    pub fn build(self) -> FetchOptions {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Response , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResponseExt;
    #[doc = " Parse the body as JSON and return a typed result."]
    #[wasm_bindgen(method, js_name = "jsonExt")]
    pub fn json_ext(this: &ResponseExt) -> Promise;
    #[doc = " Parse the body as JSON and return a typed result."]
    #[wasm_bindgen(method, catch, js_name = "jsonExt")]
    pub fn try_json_ext(this: &ResponseExt) -> Result<Promise, JsValue>;
    #[doc = " Get the response body as a Uint8Array."]
    #[wasm_bindgen(method)]
    pub fn bytes(this: &ResponseExt) -> Promise<ArrayBuffer>;
    #[doc = " Get the response body as a Uint8Array."]
    #[wasm_bindgen(method, catch, js_name = "bytes")]
    pub fn try_bytes(this: &ResponseExt) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[doc = " Whether the response was served from cache."]
    #[wasm_bindgen(method, getter)]
    pub fn cached(this: &ResponseExt) -> bool;
    #[doc = " Timing info in milliseconds."]
    #[wasm_bindgen(method, getter)]
    pub fn timing(this: &ResponseExt) -> f64;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    pub fn fetch(input: &Request) -> Promise<ResponseExt>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch(input: &Request) -> Result<Promise<ResponseExt>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str(input: &str) -> Promise<ResponseExt>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_str(input: &str) -> Result<Promise<ResponseExt>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_request_and_init(
        input: &Request,
        init: &FetchOptions,
    ) -> Promise<ResponseExt>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_request_and_init(
        input: &Request,
        init: &FetchOptions,
    ) -> Result<Promise<ResponseExt>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str_and_init(input: &str, init: &FetchOptions) -> Promise<ResponseExt>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " Perform a fetch with extended options, returning an extended response."]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_str_and_init(
        input: &str,
        init: &FetchOptions,
    ) -> Result<Promise<ResponseExt>, JsValue>;
}
