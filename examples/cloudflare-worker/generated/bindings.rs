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
pub mod console {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.log()`** static method outputs a message to the console."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/log_static)"]
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn log(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.log()`** static method outputs a message to the console."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/log_static)"]
        #[wasm_bindgen(variadic, catch, js_name = "log", js_namespace = "console")]
        pub fn try_log(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.error()`** static method outputs a message to the console at the 'error' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/error_static)"]
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn error(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.error()`** static method outputs a message to the console at the 'error' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/error_static)"]
        #[wasm_bindgen(variadic, catch, js_name = "error", js_namespace = "console")]
        pub fn try_error(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.warn()`** static method outputs a warning message to the console at the 'warning' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/warn_static)"]
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn warn(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.warn()`** static method outputs a warning message to the console at the 'warning' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/warn_static)"]
        #[wasm_bindgen(variadic, catch, js_name = "warn", js_namespace = "console")]
        pub fn try_warn(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.info()`** static method outputs a message to the console at the 'info' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/info_static)"]
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn info(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.info()`** static method outputs a message to the console at the 'info' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/info_static)"]
        #[wasm_bindgen(variadic, catch, js_name = "info", js_namespace = "console")]
        pub fn try_info(args: &[JsValue]) -> Result<(), JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.debug()`** static method outputs a message to the console at the 'debug' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/debug_static)"]
        #[wasm_bindgen(variadic, js_namespace = "console")]
        pub fn debug(args: &[JsValue]);
    }
    #[wasm_bindgen]
    extern "C" {
        #[doc = " The **`console.debug()`** static method outputs a message to the console at the 'debug' log level."]
        #[doc = " "]
        #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/debug_static)"]
        #[wasm_bindgen(variadic, catch, js_name = "debug", js_namespace = "console")]
        pub fn try_debug(args: &[JsValue]) -> Result<(), JsValue>;
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ReadableStream;
    #[doc = " The **`locked`** read-only property of the ReadableStream interface returns whether or not the readable stream is locked to a reader."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/locked)"]
    #[wasm_bindgen(method, getter)]
    pub fn locked(this: &ReadableStream) -> bool;
    #[doc = " The **`cancel()`** method of the ReadableStream interface returns a Promise that resolves when the stream is canceled."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/cancel)"]
    #[wasm_bindgen(method)]
    pub fn cancel(this: &ReadableStream) -> Promise<Undefined>;
    #[doc = " The **`cancel()`** method of the ReadableStream interface returns a Promise that resolves when the stream is canceled."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/cancel)"]
    #[wasm_bindgen(method, catch, js_name = "cancel")]
    pub fn try_cancel(this: &ReadableStream) -> Result<Promise<Undefined>, JsValue>;
    #[doc = " The **`getReader()`** method of the ReadableStream interface creates a reader and locks the stream to it."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/getReader)"]
    #[wasm_bindgen(method, js_name = "getReader")]
    pub fn get_reader(this: &ReadableStream) -> JsValue;
    #[doc = " The **`getReader()`** method of the ReadableStream interface creates a reader and locks the stream to it."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/getReader)"]
    #[wasm_bindgen(method, catch, js_name = "getReader")]
    pub fn try_get_reader(this: &ReadableStream) -> Result<JsValue, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Headers;
    #[doc = " The **`Headers()`** constructor creates a new Headers object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/Headers)"]
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Headers, JsValue>;
    #[doc = " The **`Headers()`** constructor creates a new Headers object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/Headers)"]
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_headers(init: &Headers) -> Result<Headers, JsValue>;
    #[doc = " The **`Headers()`** constructor creates a new Headers object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/Headers)"]
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_record(init: &Object<JsString>) -> Result<Headers, JsValue>;
    #[doc = " The **`Headers()`** constructor creates a new Headers object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/Headers)"]
    #[wasm_bindgen(constructor, catch, js_name = "Headers")]
    pub fn new_with_array(
        init: &Array<ArrayTuple<(JsString, JsString)>>,
    ) -> Result<Headers, JsValue>;
    #[doc = " The **`append()`** method of the Headers interface appends a new value onto an existing header inside a `Headers` object, or adds the header if it does not already exist."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/append)"]
    #[wasm_bindgen(method)]
    pub fn append(this: &Headers, name: &str, value: &str);
    #[doc = " The **`append()`** method of the Headers interface appends a new value onto an existing header inside a `Headers` object, or adds the header if it does not already exist."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/append)"]
    #[wasm_bindgen(method, catch, js_name = "append")]
    pub fn try_append(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;
    #[doc = " The **`delete()`** method of the Headers interface deletes a header from the current `Headers` object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/delete)"]
    #[wasm_bindgen(method)]
    pub fn delete(this: &Headers, name: &str);
    #[doc = " The **`delete()`** method of the Headers interface deletes a header from the current `Headers` object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/delete)"]
    #[wasm_bindgen(method, catch, js_name = "delete")]
    pub fn try_delete(this: &Headers, name: &str) -> Result<(), JsValue>;
    #[doc = " The **`get()`** method of the Headers interface returns a byte string of all the values of a header within a `Headers` object with a given name."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/get)"]
    #[wasm_bindgen(method)]
    pub fn get(this: &Headers, name: &str) -> Option<String>;
    #[doc = " The **`get()`** method of the Headers interface returns a byte string of all the values of a header within a `Headers` object with a given name."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/get)"]
    #[wasm_bindgen(method, catch, js_name = "get")]
    pub fn try_get(this: &Headers, name: &str) -> Result<Option<String>, JsValue>;
    #[doc = " The **`has()`** method of the Headers interface returns a boolean stating whether a `Headers` object contains a certain header."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/has)"]
    #[wasm_bindgen(method)]
    pub fn has(this: &Headers, name: &str) -> bool;
    #[doc = " The **`has()`** method of the Headers interface returns a boolean stating whether a `Headers` object contains a certain header."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/has)"]
    #[wasm_bindgen(method, catch, js_name = "has")]
    pub fn try_has(this: &Headers, name: &str) -> Result<bool, JsValue>;
    #[doc = " The **`set()`** method of the Headers interface sets a new value for an existing header inside a `Headers` object, or adds the header if it does not already exist."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/set)"]
    #[wasm_bindgen(method)]
    pub fn set(this: &Headers, name: &str, value: &str);
    #[doc = " The **`set()`** method of the Headers interface sets a new value for an existing header inside a `Headers` object, or adds the header if it does not already exist."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/set)"]
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
    #[doc = " The **`Request()`** constructor creates a new Request object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/Request)"]
    #[wasm_bindgen(constructor, catch)]
    pub fn new(input: &Request) -> Result<Request, JsValue>;
    #[doc = " The **`Request()`** constructor creates a new Request object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/Request)"]
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_str(input: &str) -> Result<Request, JsValue>;
    #[doc = " The **`Request()`** constructor creates a new Request object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/Request)"]
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;
    #[doc = " The **`Request()`** constructor creates a new Request object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/Request)"]
    #[wasm_bindgen(constructor, catch, js_name = "Request")]
    pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue>;
    #[doc = " The **`method`** read-only property of the Request interface contains the request's method (`GET`, `POST`, etc.)"]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/method)"]
    #[wasm_bindgen(method, getter)]
    pub fn method(this: &Request) -> String;
    #[doc = " The **`url`** read-only property of the Request interface contains the URL of the request."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/url)"]
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &Request) -> String;
    #[doc = " The **`headers`** read-only property of the Request interface contains the Headers object associated with the request."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/headers)"]
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &Request) -> Headers;
    #[doc = " The **`body`** read-only property of the Request interface is a ReadableStream of the body contents."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/body)"]
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Request) -> Option<ReadableStream>;
    #[doc = " The **`clone()`** method of the Request interface creates a copy of the current `Request` object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/clone)"]
    #[wasm_bindgen(method)]
    pub fn clone(this: &Request) -> Request;
    #[doc = " The **`clone()`** method of the Request interface creates a copy of the current `Request` object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/clone)"]
    #[wasm_bindgen(method, catch, js_name = "clone")]
    pub fn try_clone(this: &Request) -> Result<Request, JsValue>;
    #[doc = " The **`arrayBuffer()`** method of the Request interface reads the request body and returns it as a promise that resolves with an ArrayBuffer."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/arrayBuffer)"]
    #[wasm_bindgen(method, js_name = "arrayBuffer")]
    pub fn array_buffer(this: &Request) -> Promise<ArrayBuffer>;
    #[doc = " The **`arrayBuffer()`** method of the Request interface reads the request body and returns it as a promise that resolves with an ArrayBuffer."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/arrayBuffer)"]
    #[wasm_bindgen(method, catch, js_name = "arrayBuffer")]
    pub fn try_array_buffer(this: &Request) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[doc = " The **`text()`** method of the Request interface reads the request body and returns it as a promise that resolves with a string."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/text)"]
    #[wasm_bindgen(method)]
    pub fn text(this: &Request) -> Promise<JsString>;
    #[doc = " The **`text()`** method of the Request interface reads the request body and returns it as a promise that resolves with a string."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/text)"]
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
    #[doc = " A string to set request's method."]
    #[wasm_bindgen(method, getter)]
    pub fn method(this: &RequestInit) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_method(this: &RequestInit, val: &str);
    #[doc = " A Headers object, an object literal, or an array of two-item arrays to set request's headers."]
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
    #[doc = " A BodyInit object or null to set request's body."]
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &RequestInit) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_body(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_array_buffer(this: &RequestInit, val: &ArrayBuffer);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_null(this: &RequestInit, val: &Null);
    #[doc = " A string indicating how the request will interact with the browser's cache to set request's redirect."]
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
    pub fn body(mut self, val: &str) -> Self {
        self.inner.set_body(val);
        self
    }
    pub fn body_with_array_buffer(mut self, val: &ArrayBuffer) -> Self {
        self.inner.set_body_with_array_buffer(val);
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
    pub fn build(self) -> RequestInit {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Response;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str(body: &str) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer(body: &ArrayBuffer) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream(body: &ReadableStream) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_null(body: &Null) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_str_and_init(body: &str, init: &ResponseInit) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_array_buffer_and_init(
        body: &ArrayBuffer,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_readable_stream_and_init(
        body: &ReadableStream,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[doc = " The **`Response()`** constructor creates a new Response object."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)"]
    #[wasm_bindgen(constructor, catch, js_name = "Response")]
    pub fn new_with_null_and_init(body: &Null, init: &ResponseInit) -> Result<Response, JsValue>;
    #[doc = " The **`Response.redirect()`** static method returns a `Response` resulting in a redirect to the specified URL."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect_static)"]
    # [wasm_bindgen (static_method_of = Response)]
    pub fn redirect(url: &str) -> Response;
    #[doc = " The **`Response.redirect()`** static method returns a `Response` resulting in a redirect to the specified URL."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect_static)"]
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect(url: &str) -> Result<Response, JsValue>;
    #[doc = " The **`Response.redirect()`** static method returns a `Response` resulting in a redirect to the specified URL."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect_static)"]
    # [wasm_bindgen (static_method_of = Response , js_name = "redirect")]
    pub fn redirect_with_status(url: &str, status: f64) -> Response;
    #[doc = " The **`Response.redirect()`** static method returns a `Response` resulting in a redirect to the specified URL."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect_static)"]
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect_with_status(url: &str, status: f64) -> Result<Response, JsValue>;
    #[doc = " The **`Response.json()`** static method returns a `Response` that contains the provided JSON data as body, and a `Content-Type` header which is set to `application/json`."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/json_static)"]
    # [wasm_bindgen (static_method_of = Response)]
    pub fn json(data: &JsValue) -> Response;
    #[doc = " The **`Response.json()`** static method returns a `Response` that contains the provided JSON data as body, and a `Content-Type` header which is set to `application/json`."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/json_static)"]
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json(data: &JsValue) -> Result<Response, JsValue>;
    #[doc = " The **`Response.json()`** static method returns a `Response` that contains the provided JSON data as body, and a `Content-Type` header which is set to `application/json`."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/json_static)"]
    # [wasm_bindgen (static_method_of = Response , js_name = "json")]
    pub fn json_with_init(data: &JsValue, init: &ResponseInit) -> Response;
    #[doc = " The **`Response.json()`** static method returns a `Response` that contains the provided JSON data as body, and a `Content-Type` header which is set to `application/json`."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/json_static)"]
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json_with_init(data: &JsValue, init: &ResponseInit) -> Result<Response, JsValue>;
    #[doc = " The **`status`** read-only property of the Response interface contains the HTTP status codes of the response."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/status)"]
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &Response) -> f64;
    #[doc = " The **`statusText`** read-only property of the Response interface contains the status message corresponding to the HTTP status code in Response.status."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/statusText)"]
    #[wasm_bindgen(method, getter, js_name = "statusText")]
    pub fn status_text(this: &Response) -> String;
    #[doc = " The **`ok`** read-only property of the Response interface contains a Boolean stating whether the response was successful (status in the range 200-299) or not."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/ok)"]
    #[wasm_bindgen(method, getter)]
    pub fn ok(this: &Response) -> bool;
    #[doc = " The **`headers`** read-only property of the Response interface contains the Headers object associated with the response."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/headers)"]
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &Response) -> Headers;
    #[doc = " The **`body`** read-only property of the Response interface is a ReadableStream of the body contents."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/body)"]
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Response) -> Option<ReadableStream>;
    #[doc = " The **`clone()`** method of the Response interface creates a clone of a response object, identical in every way, but stored in a different variable."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/clone)"]
    #[wasm_bindgen(method)]
    pub fn clone(this: &Response) -> Response;
    #[doc = " The **`clone()`** method of the Response interface creates a clone of a response object, identical in every way, but stored in a different variable."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/clone)"]
    #[wasm_bindgen(method, catch, js_name = "clone")]
    pub fn try_clone(this: &Response) -> Result<Response, JsValue>;
    #[doc = " The **`arrayBuffer()`** method of the Response interface reads the response body and returns it as a promise that resolves with an ArrayBuffer."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/arrayBuffer)"]
    #[wasm_bindgen(method, js_name = "arrayBuffer")]
    pub fn array_buffer(this: &Response) -> Promise<ArrayBuffer>;
    #[doc = " The **`arrayBuffer()`** method of the Response interface reads the response body and returns it as a promise that resolves with an ArrayBuffer."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/arrayBuffer)"]
    #[wasm_bindgen(method, catch, js_name = "arrayBuffer")]
    pub fn try_array_buffer(this: &Response) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[doc = " The **`text()`** method of the Response interface reads the response body and returns it as a promise that resolves with a string."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/text)"]
    #[wasm_bindgen(method)]
    pub fn text(this: &Response) -> Promise<JsString>;
    #[doc = " The **`text()`** method of the Response interface reads the response body and returns it as a promise that resolves with a string."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/text)"]
    #[wasm_bindgen(method, catch, js_name = "text")]
    pub fn try_text(this: &Response) -> Result<Promise<JsString>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResponseInit;
    #[doc = " The status code for the response."]
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &ResponseInit) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_status(this: &ResponseInit, val: f64);
    #[doc = " The status message associated with the status code."]
    #[wasm_bindgen(method, getter, js_name = "statusText")]
    pub fn status_text(this: &ResponseInit) -> Option<String>;
    #[wasm_bindgen(method, setter, js_name = "statusText")]
    pub fn set_status_text(this: &ResponseInit, val: &str);
    #[doc = " A Headers object, an object literal, or an array of two-item arrays to set response's headers."]
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
    #[doc = " The **`waitUntil()`** method extends the lifetime of the event. It accepts a Promise-based task which the Workers runtime will execute before the handler terminates but without blocking the response."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#waituntil)"]
    #[wasm_bindgen(method, js_name = "waitUntil")]
    pub fn wait_until(this: &ExecutionContext, promise: &Promise);
    #[doc = " The **`waitUntil()`** method extends the lifetime of the event. It accepts a Promise-based task which the Workers runtime will execute before the handler terminates but without blocking the response."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#waituntil)"]
    #[wasm_bindgen(method, catch, js_name = "waitUntil")]
    pub fn try_wait_until(this: &ExecutionContext, promise: &Promise) -> Result<(), JsValue>;
    #[doc = " The **`passThroughOnException()`** method prevents a runtime error response when the Worker script throws an unhandled exception. Instead, the request will be forwarded to the origin server as if the Worker did not exist."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#passthroughonexception)"]
    #[wasm_bindgen(method, js_name = "passThroughOnException")]
    pub fn pass_through_on_exception(this: &ExecutionContext);
    #[doc = " The **`passThroughOnException()`** method prevents a runtime error response when the Worker script throws an unhandled exception. Instead, the request will be forwarded to the origin server as if the Worker did not exist."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#passthroughonexception)"]
    #[wasm_bindgen(method, catch, js_name = "passThroughOnException")]
    pub fn try_pass_through_on_exception(this: &ExecutionContext) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ExportedHandler;
    #[doc = " The **`fetch()`** handler is called when a Worker receives an HTTP request. It is the main entry point for handling requests."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/)"]
    #[wasm_bindgen(method)]
    pub fn fetch(
        this: &ExportedHandler,
        request: &Request,
        env: &JsValue,
        ctx: &ExecutionContext,
    ) -> Promise<Response>;
    #[doc = " The **`fetch()`** handler is called when a Worker receives an HTTP request. It is the main entry point for handling requests."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/)"]
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
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    pub fn fetch(input: &Request) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch(input: &Request) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str(input: &str) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_str(input: &str) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_request_and_init(input: &Request, init: &RequestInit) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_str_and_init(input: &str, init: &RequestInit) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[doc = " The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)"]
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
    #[doc = " Run a text-to-image AI model with the given inputs. Returns a ReadableStream containing the generated image data."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)"]
    #[wasm_bindgen(method)]
    pub fn run(this: &Ai, model: &str, inputs: &AiTextToImageInput) -> Promise<ReadableStream>;
    #[doc = " Run a text-to-image AI model with the given inputs. Returns a ReadableStream containing the generated image data."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)"]
    #[wasm_bindgen(method, catch, js_name = "run")]
    pub fn try_run(
        this: &Ai,
        model: &str,
        inputs: &AiTextToImageInput,
    ) -> Result<Promise<ReadableStream>, JsValue>;
    #[doc = " Run a text-to-image AI model with the given inputs. Returns a ReadableStream containing the generated image data."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)"]
    #[wasm_bindgen(method, js_name = "run")]
    pub fn run_with_ai_text_generation_input(
        this: &Ai,
        model: &str,
        inputs: &AiTextGenerationInput,
    ) -> Promise<ReadableStream>;
    #[doc = " Run a text-to-image AI model with the given inputs. Returns a ReadableStream containing the generated image data."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)"]
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
    #[doc = " A text description of the image you want to generate."]
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &AiTextToImageInput) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &AiTextToImageInput, val: &str);
    #[doc = " Specify what to exclude from the generated images."]
    #[wasm_bindgen(method, getter)]
    pub fn negative_prompt(this: &AiTextToImageInput) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_negative_prompt(this: &AiTextToImageInput, val: &str);
    #[doc = " The height of the generated image in pixels."]
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &AiTextToImageInput, val: f64);
    #[doc = " The width of the generated image in pixels."]
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &AiTextToImageInput, val: f64);
    #[doc = " An array of integers that represent an input image for img2img."]
    #[wasm_bindgen(method, getter)]
    pub fn image(this: &AiTextToImageInput) -> Option<Array<Number>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_image(this: &AiTextToImageInput, val: &Array<Number>);
    #[doc = " Base64-encoded string of an input image for img2img."]
    #[wasm_bindgen(method, getter, js_name = "image_b64")]
    pub fn image_b_64(this: &AiTextToImageInput) -> Option<String>;
    #[wasm_bindgen(method, setter, js_name = "image_b64")]
    pub fn set_image_b_64(this: &AiTextToImageInput, val: &str);
    #[doc = " An array of integers that represent a mask image for inpainting."]
    #[wasm_bindgen(method, getter)]
    pub fn mask(this: &AiTextToImageInput) -> Option<Array<Number>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_mask(this: &AiTextToImageInput, val: &Array<Number>);
    #[doc = " The number of diffusion steps; higher values can improve quality but take longer."]
    #[wasm_bindgen(method, getter)]
    pub fn num_steps(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_num_steps(this: &AiTextToImageInput, val: f64);
    #[doc = " How much the generated image should be similar to the input image for img2img. A value between 0 and 1."]
    #[wasm_bindgen(method, getter)]
    pub fn strength(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_strength(this: &AiTextToImageInput, val: f64);
    #[doc = " Controls how closely the generated image should adhere to the prompt; higher values make the image more aligned with the prompt."]
    #[wasm_bindgen(method, getter)]
    pub fn guidance(this: &AiTextToImageInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_guidance(this: &AiTextToImageInput, val: f64);
    #[doc = " Random seed for reproducibility of the image generation."]
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
    #[doc = " The input text prompt for the model to generate a response."]
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &AiTextGenerationInput) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &AiTextGenerationInput, val: &str);
    #[doc = " A system-level prompt that provides context or instructions to the model."]
    #[wasm_bindgen(method, getter)]
    pub fn system_prompt(this: &AiTextGenerationInput) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_system_prompt(this: &AiTextGenerationInput, val: &str);
    #[doc = " The maximum number of tokens to generate in the response."]
    #[wasm_bindgen(method, getter)]
    pub fn max_tokens(this: &AiTextGenerationInput) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_max_tokens(this: &AiTextGenerationInput, val: f64);
    #[doc = " Controls the randomness of the output; higher values produce more random results."]
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
    #[doc = " The generated text response from the model."]
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
    #[doc = " The Workers AI binding for running machine learning models."]
    #[doc = " "]
    #[doc = " [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/configuration/bindings/)"]
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
