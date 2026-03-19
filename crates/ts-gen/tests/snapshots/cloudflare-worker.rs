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
use JsValue as AbortSignal;
#[allow(dead_code)]
use JsValue as Blob;
#[allow(dead_code)]
use JsValue as E;
#[allow(dead_code)]
use JsValue as FormData;
#[allow(dead_code)]
use JsValue as IterableIterator;
#[allow(dead_code)]
use JsValue as ReadableStream;
#[allow(dead_code)]
use JsValue as URLSearchParams;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Headers;
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Headers, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_headers(init: &Headers) -> Result<Headers, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_record(init: &Object<JsString>) -> Result<Headers, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_array(
        init: &Array<ArrayTuple<(JsString, JsString)>>,
    ) -> Result<Headers, JsValue>;
    #[wasm_bindgen(method)]
    pub fn append(this: &Headers, name: &str, value: &str);
    #[wasm_bindgen(method, catch, js_name = "append")]
    pub fn try_append(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn delete(this: &Headers, name: &str);
    #[wasm_bindgen(method, catch, js_name = "delete")]
    pub fn try_delete(this: &Headers, name: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn get(this: &Headers, name: &str) -> Option<String>;
    #[wasm_bindgen(method, catch, js_name = "get")]
    pub fn try_get(this: &Headers, name: &str) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn has(this: &Headers, name: &str) -> bool;
    #[wasm_bindgen(method, catch, js_name = "has")]
    pub fn try_has(this: &Headers, name: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(method)]
    pub fn set(this: &Headers, name: &str, value: &str);
    #[wasm_bindgen(method, catch, js_name = "set")]
    pub fn try_set(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn entries(this: &Headers) -> IterableIterator;
    #[wasm_bindgen(method, catch, js_name = "entries")]
    pub fn try_entries(this: &Headers) -> Result<IterableIterator, JsValue>;
    #[wasm_bindgen(method)]
    pub fn keys(this: &Headers) -> IterableIterator;
    #[wasm_bindgen(method, catch, js_name = "keys")]
    pub fn try_keys(this: &Headers) -> Result<IterableIterator, JsValue>;
    #[wasm_bindgen(method)]
    pub fn values(this: &Headers) -> IterableIterator;
    #[wasm_bindgen(method, catch, js_name = "values")]
    pub fn try_values(this: &Headers) -> Result<IterableIterator, JsValue>;
}
#[allow(dead_code)]
pub type HeadersInit = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Request;
    #[wasm_bindgen(constructor, catch)]
    pub fn new(input: &Request) -> Result<Request, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_str(input: &str) -> Result<Request, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue>;
    #[wasm_bindgen(method, getter)]
    pub fn method(this: &Request) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &Request) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &Request) -> Headers;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Request) -> Option<ReadableStream>;
    #[wasm_bindgen(method, getter, js_name = "bodyUsed")]
    pub fn body_used(this: &Request) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn redirect(this: &Request) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn signal(this: &Request) -> AbortSignal;
    #[wasm_bindgen(method)]
    pub fn clone(this: &Request) -> Request;
    #[wasm_bindgen(method, catch, js_name = "clone")]
    pub fn try_clone(this: &Request) -> Result<Request, JsValue>;
    #[wasm_bindgen(method, js_name = "arrayBuffer")]
    pub fn array_buffer(this: &Request) -> Promise<ArrayBuffer>;
    #[wasm_bindgen(method, catch, js_name = "arrayBuffer")]
    pub fn try_array_buffer(this: &Request) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn text(this: &Request) -> Promise<JsString>;
    #[wasm_bindgen(method, catch, js_name = "text")]
    pub fn try_text(this: &Request) -> Result<Promise<JsString>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn json(this: &Request) -> Promise;
    #[wasm_bindgen(method, catch, js_name = "json")]
    pub fn try_json(this: &Request) -> Result<Promise, JsValue>;
    #[wasm_bindgen(method)]
    pub fn blob(this: &Request) -> Promise<Blob>;
    #[wasm_bindgen(method, catch, js_name = "blob")]
    pub fn try_blob(this: &Request) -> Result<Promise<Blob>, JsValue>;
    #[wasm_bindgen(method, js_name = "formData")]
    pub fn form_data(this: &Request) -> Promise<FormData>;
    #[wasm_bindgen(method, catch, js_name = "formData")]
    pub fn try_form_data(this: &Request) -> Result<Promise<FormData>, JsValue>;
}
#[allow(dead_code)]
pub type RequestInfo = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type RequestInit;
    #[wasm_bindgen(method, getter)]
    pub fn method(this: &RequestInit) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_method(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &RequestInit) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_headers(this: &RequestInit, val: &Headers);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_record(this: &RequestInit, val: &Object<JsString>);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_array(
        this: &RequestInit,
        val: &Array<ArrayTuple<(JsString, JsString)>>,
    );
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &RequestInit) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_body(this: &RequestInit, val: &ReadableStream);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_str(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_array_buffer(this: &RequestInit, val: &ArrayBuffer);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_blob(this: &RequestInit, val: &Blob);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_url_search_params(this: &RequestInit, val: &URLSearchParams);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_form_data(this: &RequestInit, val: &FormData);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_null(this: &RequestInit, val: &Null);
    #[wasm_bindgen(method, getter)]
    pub fn redirect(this: &RequestInit) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_redirect(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn signal(this: &RequestInit) -> Option<AbortSignal>;
    #[wasm_bindgen(method, setter)]
    pub fn set_signal(this: &RequestInit, val: &AbortSignal);
}
impl RequestInit {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> RequestInitBuilder {
        RequestInitBuilder { inner: Self::new() }
    }
}
pub struct RequestInitBuilder {
    inner: RequestInit,
}
#[allow(unused_mut)]
impl RequestInitBuilder {
    pub fn method(mut self, val: &str) -> Self {
        self.inner.set_method(val);
        self
    }
    pub fn headers(mut self, val: &Headers) -> Self {
        self.inner.set_headers(val);
        self
    }
    pub fn headers_with_record(mut self, val: &Object<JsString>) -> Self {
        self.inner.set_headers_with_record(val);
        self
    }
    pub fn headers_with_array(mut self, val: &Array<ArrayTuple<(JsString, JsString)>>) -> Self {
        self.inner.set_headers_with_array(val);
        self
    }
    pub fn body(mut self, val: &ReadableStream) -> Self {
        self.inner.set_body(val);
        self
    }
    pub fn body_with_str(mut self, val: &str) -> Self {
        self.inner.set_body_with_str(val);
        self
    }
    pub fn body_with_array_buffer(mut self, val: &ArrayBuffer) -> Self {
        self.inner.set_body_with_array_buffer(val);
        self
    }
    pub fn body_with_blob(mut self, val: &Blob) -> Self {
        self.inner.set_body_with_blob(val);
        self
    }
    pub fn body_with_url_search_params(mut self, val: &URLSearchParams) -> Self {
        self.inner.set_body_with_url_search_params(val);
        self
    }
    pub fn body_with_form_data(mut self, val: &FormData) -> Self {
        self.inner.set_body_with_form_data(val);
        self
    }
    pub fn body_with_null(mut self, val: &Null) -> Self {
        self.inner.set_body_with_null(val);
        self
    }
    pub fn redirect(mut self, val: &str) -> Self {
        self.inner.set_redirect(val);
        self
    }
    pub fn signal(mut self, val: &AbortSignal) -> Self {
        self.inner.set_signal(val);
        self
    }
    pub fn build(self) -> RequestInit {
        self.inner
    }
}
#[allow(dead_code)]
pub type BodyInit = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Response;
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream(body: &ReadableStream) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str(body: &str) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer(body: &ArrayBuffer) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_blob(body: &Blob) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_url_search_params(body: &URLSearchParams) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_form_data(body: &FormData) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_null(body: &Null) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream_and_init(
        body: &ReadableStream,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str_and_init(body: &str, init: &ResponseInit) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer_and_init(
        body: &ArrayBuffer,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_blob_and_init(body: &Blob, init: &ResponseInit) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_url_search_params_and_init(
        body: &URLSearchParams,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_form_data_and_init(
        body: &FormData,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_null_and_init(body: &Null, init: &ResponseInit) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response)]
    pub fn redirect(url: &str) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect(url: &str) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response , js_name = "redirect")]
    pub fn redirect_with_status(url: &str, status: f64) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect_with_status(url: &str, status: f64) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response)]
    pub fn json(data: &JsValue) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json(data: &JsValue) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response , js_name = "json")]
    pub fn json_with_init(data: &JsValue, init: &ResponseInit) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json_with_init(data: &JsValue, init: &ResponseInit) -> Result<Response, JsValue>;
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &Response) -> f64;
    #[wasm_bindgen(method, getter, js_name = "statusText")]
    pub fn status_text(this: &Response) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn ok(this: &Response) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &Response) -> Headers;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Response) -> Option<ReadableStream>;
    #[wasm_bindgen(method, getter, js_name = "bodyUsed")]
    pub fn body_used(this: &Response) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &Response) -> String;
    #[wasm_bindgen(method)]
    pub fn clone(this: &Response) -> Response;
    #[wasm_bindgen(method, catch, js_name = "clone")]
    pub fn try_clone(this: &Response) -> Result<Response, JsValue>;
    #[wasm_bindgen(method, js_name = "arrayBuffer")]
    pub fn array_buffer(this: &Response) -> Promise<ArrayBuffer>;
    #[wasm_bindgen(method, catch, js_name = "arrayBuffer")]
    pub fn try_array_buffer(this: &Response) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn text(this: &Response) -> Promise<JsString>;
    #[wasm_bindgen(method, catch, js_name = "text")]
    pub fn try_text(this: &Response) -> Result<Promise<JsString>, JsValue>;
    #[wasm_bindgen(method, js_name = "json")]
    pub fn json_1(this: &Response) -> Promise;
    #[wasm_bindgen(method, catch, js_name = "json")]
    pub fn try_json_1(this: &Response) -> Result<Promise, JsValue>;
    #[wasm_bindgen(method)]
    pub fn blob(this: &Response) -> Promise<Blob>;
    #[wasm_bindgen(method, catch, js_name = "blob")]
    pub fn try_blob(this: &Response) -> Result<Promise<Blob>, JsValue>;
    #[wasm_bindgen(method, js_name = "formData")]
    pub fn form_data(this: &Response) -> Promise<FormData>;
    #[wasm_bindgen(method, catch, js_name = "formData")]
    pub fn try_form_data(this: &Response) -> Result<Promise<FormData>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResponseInit;
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &ResponseInit) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_status(this: &ResponseInit, val: f64);
    #[wasm_bindgen(method, getter, js_name = "statusText")]
    pub fn status_text(this: &ResponseInit) -> Option<String>;
    #[wasm_bindgen(method, setter, js_name = "statusText")]
    pub fn set_status_text(this: &ResponseInit, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &ResponseInit) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_headers(this: &ResponseInit, val: &Headers);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_record(this: &ResponseInit, val: &Object<JsString>);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_array(
        this: &ResponseInit,
        val: &Array<ArrayTuple<(JsString, JsString)>>,
    );
}
impl ResponseInit {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> ResponseInitBuilder {
        ResponseInitBuilder { inner: Self::new() }
    }
}
pub struct ResponseInitBuilder {
    inner: ResponseInit,
}
#[allow(unused_mut)]
impl ResponseInitBuilder {
    pub fn status(mut self, val: f64) -> Self {
        self.inner.set_status(val);
        self
    }
    pub fn status_text(mut self, val: &str) -> Self {
        self.inner.set_status_text(val);
        self
    }
    pub fn headers(mut self, val: &Headers) -> Self {
        self.inner.set_headers(val);
        self
    }
    pub fn headers_with_record(mut self, val: &Object<JsString>) -> Self {
        self.inner.set_headers_with_record(val);
        self
    }
    pub fn headers_with_array(mut self, val: &Array<ArrayTuple<(JsString, JsString)>>) -> Self {
        self.inner.set_headers_with_array(val);
        self
    }
    pub fn build(self) -> ResponseInit {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ExecutionContext;
    #[wasm_bindgen(method, js_name = "waitUntil")]
    pub fn wait_until(this: &ExecutionContext, promise: &Promise);
    #[wasm_bindgen(method, catch, js_name = "waitUntil")]
    pub fn try_wait_until(this: &ExecutionContext, promise: &Promise) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "passThroughOnException")]
    pub fn pass_through_on_exception(this: &ExecutionContext);
    #[wasm_bindgen(method, catch, js_name = "passThroughOnException")]
    pub fn try_pass_through_on_exception(this: &ExecutionContext) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Env;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ExportedHandler;
    #[wasm_bindgen(method)]
    pub fn fetch(
        this: &ExportedHandler,
        request: &Request,
        env: &E,
        ctx: &ExecutionContext,
    ) -> Promise<Response>;
    #[wasm_bindgen(method, catch, js_name = "fetch")]
    pub fn try_fetch(
        this: &ExportedHandler,
        request: &Request,
        env: &E,
        ctx: &ExecutionContext,
    ) -> Result<Promise<Response>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn scheduled(
        this: &ExportedHandler,
        controller: &ScheduledController,
        env: &E,
        ctx: &ExecutionContext,
    ) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "scheduled")]
    pub fn try_scheduled(
        this: &ExportedHandler,
        controller: &ScheduledController,
        env: &E,
        ctx: &ExecutionContext,
    ) -> Result<Promise<Undefined>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ScheduledController;
    #[wasm_bindgen(method, getter, js_name = "scheduledTime")]
    pub fn scheduled_time(this: &ScheduledController) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn cron(this: &ScheduledController) -> String;
    #[wasm_bindgen(method, js_name = "noRetry")]
    pub fn no_retry(this: &ScheduledController);
    #[wasm_bindgen(method, catch, js_name = "noRetry")]
    pub fn try_no_retry(this: &ScheduledController) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    pub fn fetch(input: &Request) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch(input: &Request) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str(input: &str) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_str(input: &str) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_request_and_init(input: &Request, init: &RequestInit) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str_and_init(input: &str, init: &RequestInit) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_str_and_init(
        input: &str,
        init: &RequestInit,
    ) -> Result<Promise<Response>, JsValue>;
}
