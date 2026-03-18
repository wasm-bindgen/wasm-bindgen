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
pub mod console {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn log(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, catch, js_name = "log", js_namespace = "console")]
        pub fn try_log(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn error(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, catch, js_name = "error", js_namespace = "console")]
        pub fn try_error(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn warn(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, catch, js_name = "warn", js_namespace = "console")]
        pub fn try_warn(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn info(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, catch, js_name = "info", js_namespace = "console")]
        pub fn try_info(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn debug(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(variadic, catch, js_name = "debug", js_namespace = "console")]
        pub fn try_debug(args: &[JsValue]) -> Result<(), JsValue>;
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ReadableStream;
    #[wasm_bindgen(method, getter)]
    pub fn locked(this: &ReadableStream) -> bool;
    #[wasm_bindgen(method)]
    pub fn cancel(this: &ReadableStream) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "cancel")]
    pub fn try_cancel(this: &ReadableStream) -> Result<Promise<Undefined>, JsValue>;
    #[wasm_bindgen(method, js_name = "getReader")]
    pub fn get_reader(this: &ReadableStream) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "getReader")]
    pub fn try_get_reader(this: &ReadableStream) -> Result<JsValue, JsValue>;
}
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
    pub fn set_body(this: &RequestInit, val: Option<&str>);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_array_buffer(this: &RequestInit, val: Option<&ArrayBuffer>);
    #[wasm_bindgen(method, getter)]
    pub fn redirect(this: &RequestInit) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_redirect(this: &RequestInit, val: &str);
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
    pub fn body(mut self, val: Option<&str>) -> Self {
        self.inner.set_body(val);
        self
    }
    pub fn body_with_array_buffer(mut self, val: Option<&ArrayBuffer>) -> Self {
        self.inner.set_body_with_array_buffer(val);
        self
    }
    pub fn redirect(mut self, val: &str) -> Self {
        self.inner.set_redirect(val);
        self
    }
    pub fn build(self) -> RequestInit {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Response;
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str(body: Option<&str>) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer(body: Option<&ArrayBuffer>) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream(body: Option<&ReadableStream>) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str_and_init(
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer_and_init(
        body: Option<&ArrayBuffer>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream_and_init(
        body: Option<&ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
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
    pub type ExportedHandler;
    #[wasm_bindgen(method)]
    pub fn fetch(
        this: &ExportedHandler,
        request: &Request,
        env: &JsValue,
        ctx: &ExecutionContext,
    ) -> Promise<Response>;
    #[wasm_bindgen(method, catch, js_name = "fetch")]
    pub fn try_fetch(
        this: &ExportedHandler,
        request: &Request,
        env: &JsValue,
        ctx: &ExecutionContext,
    ) -> Result<Promise<Response>, JsValue>;
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
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Ai;
    #[wasm_bindgen(method)]
    pub fn run(this: &Ai, model: &str, inputs: &AiTextToImageInput) -> Promise<ReadableStream>;
    #[wasm_bindgen(method, catch, js_name = "run")]
    pub fn try_run(
        this: &Ai,
        model: &str,
        inputs: &AiTextToImageInput,
    ) -> Result<Promise<ReadableStream>, JsValue>;
    #[wasm_bindgen(method, js_name = "run")]
    pub fn run_with_ai_text_generation_input(
        this: &Ai,
        model: &str,
        inputs: &AiTextGenerationInput,
    ) -> Promise<ReadableStream>;
    #[wasm_bindgen(method, catch, js_name = "run")]
    pub fn try_run_with_ai_text_generation_input(
        this: &Ai,
        model: &str,
        inputs: &AiTextGenerationInput,
    ) -> Result<Promise<ReadableStream>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AiTextToImageInput;
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &AiTextToImageInput) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &AiTextToImageInput, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn negative_prompt(this: &AiTextToImageInput) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_negative_prompt(this: &AiTextToImageInput, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &AiTextToImageInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &AiTextToImageInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn image(this: &AiTextToImageInput) -> Option<Array<Number>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_image(this: &AiTextToImageInput, val: &Array<Number>);
    #[wasm_bindgen(method, getter, js_name = "image_b64")]
    pub fn image_b_64(this: &AiTextToImageInput) -> Option<String>;
    #[wasm_bindgen(method, setter, js_name = "image_b64")]
    pub fn set_image_b_64(this: &AiTextToImageInput, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn mask(this: &AiTextToImageInput) -> Option<Array<Number>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_mask(this: &AiTextToImageInput, val: &Array<Number>);
    #[wasm_bindgen(method, getter)]
    pub fn num_steps(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_num_steps(this: &AiTextToImageInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn strength(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_strength(this: &AiTextToImageInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn guidance(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_guidance(this: &AiTextToImageInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn seed(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_seed(this: &AiTextToImageInput, val: f64);
}
impl AiTextToImageInput {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> AiTextToImageInputBuilder {
        AiTextToImageInputBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct AiTextToImageInputBuilder {
    inner: AiTextToImageInput,
    required: u64,
}
#[allow(unused_mut)]
impl AiTextToImageInputBuilder {
    pub fn prompt(mut self, val: &str) -> Self {
        self.inner.set_prompt(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn negative_prompt(mut self, val: &str) -> Self {
        self.inner.set_negative_prompt(val);
        self
    }
    pub fn height(mut self, val: f64) -> Self {
        self.inner.set_height(val);
        self
    }
    pub fn width(mut self, val: f64) -> Self {
        self.inner.set_width(val);
        self
    }
    pub fn image(mut self, val: &Array<Number>) -> Self {
        self.inner.set_image(val);
        self
    }
    pub fn image_b_64(mut self, val: &str) -> Self {
        self.inner.set_image_b_64(val);
        self
    }
    pub fn mask(mut self, val: &Array<Number>) -> Self {
        self.inner.set_mask(val);
        self
    }
    pub fn num_steps(mut self, val: f64) -> Self {
        self.inner.set_num_steps(val);
        self
    }
    pub fn strength(mut self, val: f64) -> Self {
        self.inner.set_strength(val);
        self
    }
    pub fn guidance(mut self, val: f64) -> Self {
        self.inner.set_guidance(val);
        self
    }
    pub fn seed(mut self, val: f64) -> Self {
        self.inner.set_seed(val);
        self
    }
    pub fn build(self) -> Result<AiTextToImageInput, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `prompt`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(AiTextToImageInput),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AiTextGenerationInput;
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &AiTextGenerationInput) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &AiTextGenerationInput, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn system_prompt(this: &AiTextGenerationInput) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_system_prompt(this: &AiTextGenerationInput, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn max_tokens(this: &AiTextGenerationInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_max_tokens(this: &AiTextGenerationInput, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn temperature(this: &AiTextGenerationInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_temperature(this: &AiTextGenerationInput, val: f64);
}
impl AiTextGenerationInput {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> AiTextGenerationInputBuilder {
        AiTextGenerationInputBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct AiTextGenerationInputBuilder {
    inner: AiTextGenerationInput,
    required: u64,
}
#[allow(unused_mut)]
impl AiTextGenerationInputBuilder {
    pub fn prompt(mut self, val: &str) -> Self {
        self.inner.set_prompt(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn system_prompt(mut self, val: &str) -> Self {
        self.inner.set_system_prompt(val);
        self
    }
    pub fn max_tokens(mut self, val: f64) -> Self {
        self.inner.set_max_tokens(val);
        self
    }
    pub fn temperature(mut self, val: f64) -> Self {
        self.inner.set_temperature(val);
        self
    }
    pub fn build(self) -> Result<AiTextGenerationInput, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `prompt`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(AiTextGenerationInput),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AiTextGenerationOutput;
    #[wasm_bindgen(method, getter)]
    pub fn response(this: &AiTextGenerationOutput) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_response(this: &AiTextGenerationOutput, val: &str);
}
impl AiTextGenerationOutput {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> AiTextGenerationOutputBuilder {
        AiTextGenerationOutputBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct AiTextGenerationOutputBuilder {
    inner: AiTextGenerationOutput,
    required: u64,
}
#[allow(unused_mut)]
impl AiTextGenerationOutputBuilder {
    pub fn response(mut self, val: &str) -> Self {
        self.inner.set_response(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<AiTextGenerationOutput, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `response`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(AiTextGenerationOutput),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Env;
    #[wasm_bindgen(method, getter, js_name = "AI")]
    pub fn ai(this: &Env) -> Ai;
    #[wasm_bindgen(method, setter, js_name = "AI")]
    pub fn set_ai(this: &Env, val: &Ai);
}
impl Env {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> EnvBuilder {
        EnvBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct EnvBuilder {
    inner: Env,
    required: u64,
}
#[allow(unused_mut)]
impl EnvBuilder {
    pub fn ai(mut self, val: &Ai) -> Self {
        self.inner.set_ai(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<Env, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `AI`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(Env),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
