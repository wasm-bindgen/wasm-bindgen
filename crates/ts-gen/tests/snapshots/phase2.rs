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
use JsValue as Blob;
#[allow(dead_code)]
use JsValue as ReadableStream;
#[allow(dead_code)]
use JsValue as Request;
#[allow(dead_code)]
use JsValue as Response;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Writable;
    #[wasm_bindgen(method)]
    pub fn write(this: &Writable, data: &str) -> bool;
    #[wasm_bindgen(method, catch, js_name = "write")]
    pub fn try_write(this: &Writable, data: &str) -> Result<bool, JsValue>;
}
#[allow(dead_code)]
pub type WritableStream = Writable;
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Priority {
    Low = -1i32,
    Normal = 0i32,
    High = 1i32,
}
#[allow(dead_code)]
pub type StringOrNumber = JsValue;
#[allow(dead_code)]
pub type BodyInit = JsValue;
#[wasm_bindgen]
extern "C" {
    pub fn send(body: &ReadableStream);
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "send")]
    pub fn try_send(body: &ReadableStream) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "send")]
    pub fn send_with_str(body: &str);
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "send")]
    pub fn try_send_with_str(body: &str) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "send")]
    pub fn send_with_array_buffer(body: &ArrayBuffer);
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "send")]
    pub fn try_send_with_array_buffer(body: &ArrayBuffer) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "send")]
    pub fn send_with_blob(body: &Blob);
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "send")]
    pub fn try_send_with_blob(body: &Blob) -> Result<(), JsValue>;
}
#[allow(dead_code)]
pub type RequestInfo = JsValue;
#[wasm_bindgen]
extern "C" {
    pub fn fetch(input: &str) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch(input: &str) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_request(input: &Request) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_request(input: &Request) -> Result<Promise<Response>, JsValue>;
}
