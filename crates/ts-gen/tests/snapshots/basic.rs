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
use JsValue as DurableObjectState;
#[allow(dead_code)]
use JsValue as FormData;
#[allow(dead_code)]
use JsValue as Headers;
#[allow(dead_code)]
use JsValue as Navigator;
#[allow(dead_code)]
use JsValue as ReadableStream;
#[allow(dead_code)]
use JsValue as RequestInfo;
#[allow(dead_code)]
use JsValue as RequestInit;
#[allow(dead_code)]
use JsValue as ServiceWorkerGlobalScope;
#[allow(dead_code)]
use JsValue as URLSearchParams;
#[allow(dead_code)]
use JsValue as WebSocket;
#[allow(dead_code)]
use JsValue as WritableStream;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DOMException;
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<DOMException, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "DOMException")]
    pub fn new_with_message(message: &str) -> Result<DOMException, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "DOMException")]
    pub fn new_with_message_and_name(message: &str, name: &str) -> Result<DOMException, JsValue>;
    #[doc = " The error message."]
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &DOMException) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &DOMException) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn code(this: &DOMException) -> f64;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Body , extends = Object)]
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
    #[doc = " Returns a new Response with a network error."]
    # [wasm_bindgen (static_method_of = Response)]
    pub fn error() -> Response;
    #[doc = " Returns a new Response with a network error."]
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "error")]
    pub fn try_error() -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response)]
    pub fn redirect(url: &str) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect(url: &str) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response , js_name = "redirect")]
    pub fn redirect_with_status(url: &str, status: f64) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "redirect")]
    pub fn try_redirect_with_status(url: &str, status: f64) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response)]
    pub fn json(any: &JsValue) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json(any: &JsValue) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response , js_name = "json")]
    pub fn json_with_response_init(any: &JsValue, maybe_init: &ResponseInit) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json_with_response_init(
        any: &JsValue,
        maybe_init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    # [wasm_bindgen (static_method_of = Response , js_name = "json")]
    pub fn json_with_response(any: &JsValue, maybe_init: &Response) -> Response;
    # [wasm_bindgen (static_method_of = Response , catch , js_name = "json")]
    pub fn try_json_with_response(
        any: &JsValue,
        maybe_init: &Response,
    ) -> Result<Response, JsValue>;
    #[doc = " Creates a clone of this response."]
    #[wasm_bindgen(method)]
    pub fn clone(this: &Response) -> Response;
    #[doc = " Creates a clone of this response."]
    #[wasm_bindgen(method, catch, js_name = "clone")]
    pub fn try_clone(this: &Response) -> Result<Response, JsValue>;
    #[doc = " The HTTP status code."]
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &Response) -> f64;
    #[wasm_bindgen(method, getter, js_name = "statusText")]
    pub fn status_text(this: &Response) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &Response) -> Headers;
    #[wasm_bindgen(method, getter)]
    pub fn ok(this: &Response) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &Response) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Response) -> Option<ReadableStream>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Body;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Body) -> Option<ReadableStream>;
    #[wasm_bindgen(method, getter, js_name = "bodyUsed")]
    pub fn body_used(this: &Body) -> bool;
    #[doc = " Returns the body as an ArrayBuffer."]
    #[wasm_bindgen(method, js_name = "arrayBuffer")]
    pub fn array_buffer(this: &Body) -> Promise<ArrayBuffer>;
    #[doc = " Returns the body as an ArrayBuffer."]
    #[wasm_bindgen(method, catch, js_name = "arrayBuffer")]
    pub fn try_array_buffer(this: &Body) -> Result<Promise<ArrayBuffer>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn text(this: &Body) -> Promise<JsString>;
    #[wasm_bindgen(method, catch, js_name = "text")]
    pub fn try_text(this: &Body) -> Result<Promise<JsString>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn json(this: &Body) -> Promise;
    #[wasm_bindgen(method, catch, js_name = "json")]
    pub fn try_json(this: &Body) -> Result<Promise, JsValue>;
    #[wasm_bindgen(method)]
    pub fn blob(this: &Body) -> Promise<Blob>;
    #[wasm_bindgen(method, catch, js_name = "blob")]
    pub fn try_blob(this: &Body) -> Result<Promise<Blob>, JsValue>;
    #[wasm_bindgen(method, js_name = "formData")]
    pub fn form_data(this: &Body) -> Promise<FormData>;
    #[wasm_bindgen(method, catch, js_name = "formData")]
    pub fn try_form_data(this: &Body) -> Result<Promise<FormData>, JsValue>;
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
    pub fn set_headers_with_array(this: &ResponseInit, val: &Array<Array<JsString>>);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_record(this: &ResponseInit, val: &Object<JsString>);
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
    pub fn headers_with_array(mut self, val: &Array<Array<JsString>>) -> Self {
        self.inner.set_headers_with_array(val);
        self
    }
    pub fn headers_with_record(mut self, val: &Object<JsString>) -> Self {
        self.inner.set_headers_with_record(val);
        self
    }
    pub fn build(self) -> ResponseInit {
        self.inner
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueContentType {
    #[wasm_bindgen(js_name = "text")]
    Text,
    #[wasm_bindgen(js_name = "bytes")]
    Bytes,
    #[wasm_bindgen(js_name = "json")]
    Json,
    #[wasm_bindgen(js_name = "v8")]
    V8,
}
#[allow(dead_code)]
pub type BodyInit = JsValue;
#[allow(dead_code)]
pub type HeadersInit = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DurableObject;
    #[wasm_bindgen(method, getter)]
    pub fn ctx(this: &DurableObject) -> DurableObjectState;
    #[wasm_bindgen(method, setter)]
    pub fn set_ctx(this: &DurableObject, val: &DurableObjectState);
    #[wasm_bindgen(method, getter)]
    pub fn env(this: &DurableObject) -> Object;
    #[wasm_bindgen(method, setter)]
    pub fn set_env(this: &DurableObject, val: &Object);
    #[wasm_bindgen(method)]
    pub fn alarm(this: &DurableObject) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "alarm")]
    pub fn try_alarm(this: &DurableObject) -> Result<Promise<Undefined>, JsValue>;
    #[wasm_bindgen(method, js_name = "webSocketMessage")]
    pub fn web_socket_message(
        this: &DurableObject,
        ws: &WebSocket,
        message: &str,
    ) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "webSocketMessage")]
    pub fn try_web_socket_message(
        this: &DurableObject,
        ws: &WebSocket,
        message: &str,
    ) -> Result<Promise<Undefined>, JsValue>;
    #[wasm_bindgen(method, js_name = "webSocketMessage")]
    pub fn web_socket_message_with_array_buffer(
        this: &DurableObject,
        ws: &WebSocket,
        message: &ArrayBuffer,
    ) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "webSocketMessage")]
    pub fn try_web_socket_message_with_array_buffer(
        this: &DurableObject,
        ws: &WebSocket,
        message: &ArrayBuffer,
    ) -> Result<Promise<Undefined>, JsValue>;
}
pub mod web_assembly {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (extends = Object , js_namespace = "WebAssembly")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type Module;
        #[wasm_bindgen(constructor, catch)]
        pub fn new(bytes: &ArrayBuffer) -> Result<Module, JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (extends = Object , js_namespace = "WebAssembly")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type Instance;
        #[wasm_bindgen(constructor, catch)]
        pub fn new(module: &Module) -> Result<Instance, JsValue>;
        #[wasm_bindgen(constructor, catch, js_name = "Instance")]
        pub fn new_with_imports(module: &Module, imports: &Object) -> Result<Instance, JsValue>;
        #[wasm_bindgen(method, getter)]
        pub fn exports(this: &Instance) -> Object;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = "WebAssembly")]
        pub fn compile(bytes: &ArrayBuffer) -> Promise<Module>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "compile", js_namespace = "WebAssembly")]
        pub fn try_compile(bytes: &ArrayBuffer) -> Result<Promise<Module>, JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = "WebAssembly")]
        pub fn instantiate(module: &Module) -> Promise<Instance>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "instantiate", js_namespace = "WebAssembly")]
        pub fn try_instantiate(module: &Module) -> Result<Promise<Instance>, JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = "instantiate", js_namespace = "WebAssembly")]
        pub fn instantiate_with_imports(module: &Module, imports: &Object) -> Promise<Instance>;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "instantiate", js_namespace = "WebAssembly")]
        pub fn try_instantiate_with_imports(
            module: &Module,
            imports: &Object,
        ) -> Result<Promise<Instance>, JsValue>;
    }
}
#[wasm_bindgen]
extern "C" {
    pub fn fetch(input: &RequestInfo) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch(input: &RequestInfo) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetch")]
    pub fn fetch_with_init(input: &RequestInfo, init: &RequestInit) -> Promise<Response>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    pub fn try_fetch_with_init(
        input: &RequestInfo,
        init: &RequestInit,
    ) -> Result<Promise<Response>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    pub fn atob(data: &str) -> String;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "atob")]
    pub fn try_atob(data: &str) -> Result<String, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    pub fn btoa(data: &str) -> String;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "btoa")]
    pub fn try_btoa(data: &str) -> Result<String, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    pub static navigator: Navigator;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    pub static self_: ServiceWorkerGlobalScope;
}
pub mod sockets {
    use super::*;
    use js_sys::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        pub fn connect(address: &str) -> Socket;
    }
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "connect")]
        pub fn try_connect(address: &str) -> Result<Socket, JsValue>;
    }
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        #[wasm_bindgen(js_name = "connect")]
        pub fn connect_with_options(address: &str, options: &SocketOptions) -> Socket;
    }
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "connect")]
        pub fn try_connect_with_options(
            address: &str,
            options: &SocketOptions,
        ) -> Result<Socket, JsValue>;
    }
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type Socket;
        #[wasm_bindgen(method)]
        pub fn close(this: &Socket) -> Promise<Undefined>;
        #[wasm_bindgen(method, catch, js_name = "close")]
        pub fn try_close(this: &Socket) -> Result<Promise<Undefined>, JsValue>;
        #[wasm_bindgen(method, getter)]
        pub fn closed(this: &Socket) -> Promise<Undefined>;
        #[wasm_bindgen(method, getter)]
        pub fn opened(this: &Socket) -> Promise<Undefined>;
        #[wasm_bindgen(method, getter)]
        pub fn readable(this: &Socket) -> ReadableStream;
        #[wasm_bindgen(method, getter)]
        pub fn writable(this: &Socket) -> WritableStream;
        #[wasm_bindgen(method, js_name = "startTls")]
        pub fn start_tls(this: &Socket) -> Socket;
        #[wasm_bindgen(method, catch, js_name = "startTls")]
        pub fn try_start_tls(this: &Socket) -> Result<Socket, JsValue>;
    }
    #[wasm_bindgen(module = "cloudflare:sockets")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type SocketOptions;
        #[wasm_bindgen(method, getter, js_name = "secureTransport")]
        pub fn secure_transport(this: &SocketOptions) -> Option<String>;
        #[wasm_bindgen(method, setter, js_name = "secureTransport")]
        pub fn set_secure_transport(this: &SocketOptions, val: &str);
        #[wasm_bindgen(method, getter, js_name = "allowHalfOpen")]
        pub fn allow_half_open(this: &SocketOptions) -> Option<bool>;
        #[wasm_bindgen(method, setter, js_name = "allowHalfOpen")]
        pub fn set_allow_half_open(this: &SocketOptions, val: bool);
    }
    impl SocketOptions {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_imports)]
            use wasm_bindgen::JsCast;
            JsCast::unchecked_into(js_sys::Object::new())
        }
        pub fn builder() -> SocketOptionsBuilder {
            SocketOptionsBuilder { inner: Self::new() }
        }
    }
    pub struct SocketOptionsBuilder {
        inner: SocketOptions,
    }
    #[allow(unused_mut)]
    impl SocketOptionsBuilder {
        pub fn secure_transport(mut self, val: &str) -> Self {
            self.inner.set_secure_transport(val);
            self
        }
        pub fn allow_half_open(mut self, val: bool) -> Self {
            self.inner.set_allow_half_open(val);
            self
        }
        pub fn build(self) -> SocketOptions {
            self.inner
        }
    }
}
