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
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SignedEnum {
    Negative = -1i32,
    Zero = 0i32,
    Positive = 1i32,
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UnsignedEnum {
    A = 0u32,
    B = 1u32,
    C = 2u32,
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AutoIncrement {
    Start = -2i32,
    Next = -1i32,
    Last = 0i32,
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EventTarget;
    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventTarget, r#type: &str, listener: &Function);
    #[wasm_bindgen(method, catch, js_name = "addEventListener")]
    pub fn try_add_event_listener(
        this: &EventTarget,
        r#type: &str,
        listener: &Function,
    ) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Serializable;
    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Serializable) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "toJSON")]
    pub fn try_to_json(this: &Serializable) -> Result<JsValue, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = Serializable , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EventEmitter;
    #[wasm_bindgen(method)]
    pub fn emit(this: &EventEmitter, event: &str) -> bool;
    #[wasm_bindgen(method, catch, js_name = "emit")]
    pub fn try_emit(this: &EventEmitter, event: &str) -> Result<bool, JsValue>;
}
pub mod node_js {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (extends = Object , js_namespace = "NodeJS")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type EventEmitter;
        #[wasm_bindgen(method)]
        pub fn on(this: &EventEmitter, event: &str, listener: &Function) -> JsValue;
        #[wasm_bindgen(method, catch, js_name = "on")]
        pub fn try_on(
            this: &EventEmitter,
            event: &str,
            listener: &Function,
        ) -> Result<JsValue, JsValue>;
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventEmitter , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Stream;
    #[wasm_bindgen(method)]
    pub fn pipe(this: &Stream, destination: &Stream) -> Stream;
    #[wasm_bindgen(method, catch, js_name = "pipe")]
    pub fn try_pipe(this: &Stream, destination: &Stream) -> Result<Stream, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Counter;
    #[wasm_bindgen(method)]
    pub fn count(this: &Counter) -> f64;
    #[wasm_bindgen(method, catch, js_name = "count")]
    pub fn try_count(this: &Counter) -> Result<f64, JsValue>;
    #[wasm_bindgen(method, js_name = "try_count")]
    pub fn try_count_1(this: &Counter) -> f64;
    #[wasm_bindgen(method, catch, js_name = "try_count")]
    pub fn try_try_count_1(this: &Counter) -> Result<f64, JsValue>;
    #[wasm_bindgen(method)]
    pub fn reset(this: &Counter);
    #[wasm_bindgen(method, catch, js_name = "reset")]
    pub fn try_reset(this: &Counter) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Duplex;
    #[wasm_bindgen(method)]
    pub fn read(this: &Duplex) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "read")]
    pub fn try_read(this: &Duplex) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method)]
    pub fn write(this: &Duplex, data: &ArrayBuffer) -> bool;
    #[wasm_bindgen(method, catch, js_name = "write")]
    pub fn try_write(this: &Duplex, data: &ArrayBuffer) -> Result<bool, JsValue>;
    #[wasm_bindgen(method)]
    pub fn end(this: &Duplex);
    #[wasm_bindgen(method, catch, js_name = "end")]
    pub fn try_end(this: &Duplex) -> Result<(), JsValue>;
}
pub mod intl {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (extends = Object , js_namespace = "Intl")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type Collator;
        #[wasm_bindgen(method)]
        pub fn compare(this: &Collator, a: &str, b: &str) -> f64;
        #[wasm_bindgen(method, catch, js_name = "compare")]
        pub fn try_compare(this: &Collator, a: &str, b: &str) -> Result<f64, JsValue>;
    }
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (extends = Object , js_namespace = "Intl")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type CollatorOptions;
        #[wasm_bindgen(method, getter)]
        pub fn usage(this: &CollatorOptions) -> Option<String>;
        #[wasm_bindgen(method, setter)]
        pub fn set_usage(this: &CollatorOptions, val: &str);
        #[wasm_bindgen(method, getter)]
        pub fn sensitivity(this: &CollatorOptions) -> Option<String>;
        #[wasm_bindgen(method, setter)]
        pub fn set_sensitivity(this: &CollatorOptions, val: &str);
    }
    impl CollatorOptions {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_imports)]
            use wasm_bindgen::JsCast;
            JsCast::unchecked_into(js_sys::Object::new())
        }
        pub fn builder() -> CollatorOptionsBuilder {
            CollatorOptionsBuilder { inner: Self::new() }
        }
    }
    pub struct CollatorOptionsBuilder {
        inner: CollatorOptions,
    }
    #[allow(unused_mut)]
    impl CollatorOptionsBuilder {
        pub fn usage(mut self, val: &str) -> Self {
            self.inner.set_usage(val);
            self
        }
        pub fn sensitivity(mut self, val: &str) -> Self {
            self.inner.set_sensitivity(val);
            self
        }
        pub fn build(self) -> CollatorOptions {
            self.inner
        }
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, js_name = "defaultLocale", js_namespace = "Intl")]
        pub static default_locale: String;
    }
}
