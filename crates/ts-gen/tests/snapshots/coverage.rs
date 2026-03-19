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
use JsValue as Headers;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type StringMap;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type NumberIndexed;
    #[wasm_bindgen(method, getter)]
    pub fn length(this: &NumberIndexed) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_length(this: &NumberIndexed, val: f64);
}
impl NumberIndexed {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> NumberIndexedBuilder {
        NumberIndexedBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct NumberIndexedBuilder {
    inner: NumberIndexed,
    required: u64,
}
#[allow(unused_mut)]
impl NumberIndexedBuilder {
    pub fn length(mut self, val: f64) -> Self {
        self.inner.set_length(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<NumberIndexed, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `length`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(NumberIndexed),
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
    pub type MixedWithIndex;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &MixedWithIndex) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &MixedWithIndex, val: &str);
}
impl MixedWithIndex {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> MixedWithIndexBuilder {
        MixedWithIndexBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct MixedWithIndexBuilder {
    inner: MixedWithIndex,
    required: u64,
}
#[allow(unused_mut)]
impl MixedWithIndexBuilder {
    pub fn name(mut self, val: &str) -> Self {
        self.inner.set_name(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<MixedWithIndex, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `name`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(MixedWithIndex),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[allow(dead_code)]
pub type HasName = Object;
#[allow(dead_code)]
pub type HasAge = Object;
#[allow(dead_code)]
pub type Person = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Serializable;
    #[wasm_bindgen(method)]
    pub fn serialize(this: &Serializable) -> String;
    #[wasm_bindgen(method, catch, js_name = "serialize")]
    pub fn try_serialize(this: &Serializable) -> Result<String, JsValue>;
}
#[allow(dead_code)]
pub type SerializablePerson = JsValue;
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Up = 0u32,
    Down = 1u32,
    Left = 2u32,
    Right = 3u32,
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HttpStatus {
    Ok = 200u32,
    NotFound = 404u32,
    InternalServerError = 500u32,
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type GlobalMixin;
    #[wasm_bindgen(method, js_name = "customMethod")]
    pub fn custom_method(this: &GlobalMixin);
    #[wasm_bindgen(method, catch, js_name = "customMethod")]
    pub fn try_custom_method(this: &GlobalMixin) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "globalHelper")]
    pub fn global_helper(x: f64) -> String;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "globalHelper")]
    pub fn try_global_helper(x: f64) -> Result<String, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = "GLOBAL_VERSION")]
    pub static global_version: String;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DefaultProcessor;
    #[wasm_bindgen(constructor, catch)]
    pub fn new(config: &Object) -> Result<DefaultProcessor, JsValue>;
    #[wasm_bindgen(method)]
    pub fn process(this: &DefaultProcessor, input: &str) -> Promise<JsString>;
    #[wasm_bindgen(method, catch, js_name = "process")]
    pub fn try_process(this: &DefaultProcessor, input: &str) -> Result<Promise<JsString>, JsValue>;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &DefaultProcessor) -> String;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "createProcessor")]
    pub fn create_processor(name: &str) -> DefaultProcessor;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "createProcessor")]
    pub fn try_create_processor(name: &str) -> Result<DefaultProcessor, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TreeNode;
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &TreeNode) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &TreeNode, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn children(this: &TreeNode) -> Array<TreeNode>;
    #[wasm_bindgen(method, setter)]
    pub fn set_children(this: &TreeNode, val: &Array<TreeNode>);
    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &TreeNode) -> Option<TreeNode>;
    #[wasm_bindgen(method, setter)]
    pub fn set_parent(this: &TreeNode, val: &TreeNode);
}
impl TreeNode {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> TreeNodeBuilder {
        TreeNodeBuilder {
            inner: Self::new(),
            required: 3u64,
        }
    }
}
pub struct TreeNodeBuilder {
    inner: TreeNode,
    required: u64,
}
#[allow(unused_mut)]
impl TreeNodeBuilder {
    pub fn value(mut self, val: &str) -> Self {
        self.inner.set_value(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn children(mut self, val: &Array<TreeNode>) -> Self {
        self.inner.set_children(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn parent(mut self, val: &TreeNode) -> Self {
        self.inner.set_parent(val);
        self
    }
    pub fn build(self) -> Result<TreeNode, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `value`");
            }
            if self.required & 2u64 != 0 {
                missing.push("missing required property `children`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(TreeNode),
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
    pub type LinkedList;
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &LinkedList) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &LinkedList, val: &JsValue);
    #[wasm_bindgen(method, getter)]
    pub fn next(this: &LinkedList) -> Option<LinkedList>;
    #[wasm_bindgen(method, setter)]
    pub fn set_next(this: &LinkedList, val: &LinkedList);
    #[wasm_bindgen(method, setter, js_name = "next")]
    pub fn set_next_with_null(this: &LinkedList, val: &Null);
}
impl LinkedList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> LinkedListBuilder {
        LinkedListBuilder {
            inner: Self::new(),
            required: 3u64,
        }
    }
}
pub struct LinkedListBuilder {
    inner: LinkedList,
    required: u64,
}
#[allow(unused_mut)]
impl LinkedListBuilder {
    pub fn data(mut self, val: &JsValue) -> Self {
        self.inner.set_data(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn next(mut self, val: &LinkedList) -> Self {
        self.inner.set_next(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn next_with_null(mut self, val: &Null) -> Self {
        self.inner.set_next_with_null(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn build(self) -> Result<LinkedList, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `data`");
            }
            if self.required & 2u64 != 0 {
                missing.push("missing required property `next`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(LinkedList),
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
    pub type Iterable;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AsyncIterable;
}
#[wasm_bindgen]
extern "C" {
    pub fn parse(input: &str, reviver: &Function) -> Object;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "parse")]
    pub fn try_parse(input: &str, reviver: &Function) -> Result<Object, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    pub fn stringify(value: &JsValue, replacer: &Function, space: f64) -> String;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "stringify")]
    pub fn try_stringify(
        value: &JsValue,
        replacer: &Function,
        space: f64,
    ) -> Result<String, JsValue>;
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SignedValues {
    NegativeOne = -1i32,
    Zero = 0i32,
    One = 1i32,
    Max = 2147483647i32,
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Serializable , extends = GlobalMixin , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MultiExtend;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MultiExtend) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &MultiExtend, val: &str);
}
impl MultiExtend {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> MultiExtendBuilder {
        MultiExtendBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct MultiExtendBuilder {
    inner: MultiExtend,
    required: u64,
}
#[allow(unused_mut)]
impl MultiExtendBuilder {
    pub fn id(mut self, val: &str) -> Self {
        self.inner.set_id(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<MultiExtend, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `id`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(MultiExtend),
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
    pub type EventTarget;
    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventTarget, r#type: &str, listener: &Function);
    #[wasm_bindgen(method, catch, js_name = "addEventListener")]
    pub fn try_add_event_listener(
        this: &EventTarget,
        r#type: &str,
        listener: &Function,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "removeEventListener")]
    pub fn remove_event_listener(this: &EventTarget, r#type: &str, listener: &Function);
    #[wasm_bindgen(method, catch, js_name = "removeEventListener")]
    pub fn try_remove_event_listener(
        this: &EventTarget,
        r#type: &str,
        listener: &Function,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "dispatchEvent")]
    pub fn dispatch_event(this: &EventTarget, event: &Object) -> bool;
    #[wasm_bindgen(method, catch, js_name = "dispatchEvent")]
    pub fn try_dispatch_event(this: &EventTarget, event: &Object) -> Result<bool, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EventEmitter;
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<EventEmitter, JsValue>;
    # [wasm_bindgen (static_method_of = EventEmitter , js_name = "listenerCount")]
    pub fn listener_count(emitter: &EventEmitter, event: &str) -> f64;
    # [wasm_bindgen (static_method_of = EventEmitter , catch , js_name = "listenerCount")]
    pub fn try_listener_count(emitter: &EventEmitter, event: &str) -> Result<f64, JsValue>;
    #[wasm_bindgen(method)]
    pub fn on(this: &EventEmitter, event: &str, listener: &Function) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "on")]
    pub fn try_on(
        this: &EventEmitter,
        event: &str,
        listener: &Function,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, variadic)]
    pub fn emit(this: &EventEmitter, event: &str, args: &[JsValue]) -> bool;
    #[wasm_bindgen(method, variadic, catch, js_name = "emit")]
    pub fn try_emit(this: &EventEmitter, event: &str, args: &[JsValue]) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_name = "removeAllListeners")]
    pub fn remove_all_listeners(this: &EventEmitter) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "removeAllListeners")]
    pub fn try_remove_all_listeners(this: &EventEmitter) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = "removeAllListeners")]
    pub fn remove_all_listeners_with_event(this: &EventEmitter, event: &str) -> JsValue;
    #[wasm_bindgen(method, catch, js_name = "removeAllListeners")]
    pub fn try_remove_all_listeners_with_event(
        this: &EventEmitter,
        event: &str,
    ) -> Result<JsValue, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Storage;
    #[wasm_bindgen(method, js_name = "getItem")]
    pub fn get_item(this: &Storage, key: &str) -> Option<String>;
    #[wasm_bindgen(method, catch, js_name = "getItem")]
    pub fn try_get_item(this: &Storage, key: &str) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(method, js_name = "setItem")]
    pub fn set_item(this: &Storage, key: &str, value: &str);
    #[wasm_bindgen(method, catch, js_name = "setItem")]
    pub fn try_set_item(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "removeItem")]
    pub fn remove_item(this: &Storage, key: &str);
    #[wasm_bindgen(method, catch, js_name = "removeItem")]
    pub fn try_remove_item(this: &Storage, key: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn clear(this: &Storage);
    #[wasm_bindgen(method, catch, js_name = "clear")]
    pub fn try_clear(this: &Storage) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter)]
    pub fn length(this: &Storage) -> f64;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Cache;
    #[wasm_bindgen(method)]
    pub fn get(this: &Cache, key: &str) -> Promise<JsOption<Map<JsString, Array<JsString>>>>;
    #[wasm_bindgen(method, catch, js_name = "get")]
    pub fn try_get(
        this: &Cache,
        key: &str,
    ) -> Result<Promise<JsOption<Map<JsString, Array<JsString>>>>, JsValue>;
    #[wasm_bindgen(method)]
    pub fn set(
        this: &Cache,
        key: &str,
        value: &Map<JsString, Array<JsString>>,
    ) -> Promise<Undefined>;
    #[wasm_bindgen(method, catch, js_name = "set")]
    pub fn try_set(
        this: &Cache,
        key: &str,
        value: &Map<JsString, Array<JsString>>,
    ) -> Result<Promise<Undefined>, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FetchOptions;
    #[wasm_bindgen(method, getter)]
    pub fn method(this: &FetchOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_method(this: &FetchOptions, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &FetchOptions) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_headers(this: &FetchOptions, val: &Headers);
    #[wasm_bindgen(method, setter, js_name = "headers")]
    pub fn set_headers_with_record(this: &FetchOptions, val: &Object<JsString>);
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &FetchOptions) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_body(this: &FetchOptions, val: &str);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_array_buffer(this: &FetchOptions, val: &ArrayBuffer);
    #[wasm_bindgen(method, setter, js_name = "body")]
    pub fn set_body_with_null(this: &FetchOptions, val: &Null);
    #[wasm_bindgen(method, getter)]
    pub fn redirect(this: &FetchOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_redirect(this: &FetchOptions, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn signal(this: &FetchOptions) -> Option<AbortSignal>;
    #[wasm_bindgen(method, setter)]
    pub fn set_signal(this: &FetchOptions, val: &AbortSignal);
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
    pub fn signal(mut self, val: &AbortSignal) -> Self {
        self.inner.set_signal(val);
        self
    }
    pub fn build(self) -> FetchOptions {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type SimpleConfig;
    #[wasm_bindgen(method, getter)]
    pub fn verbose(this: &SimpleConfig) -> Option<bool>;
    #[wasm_bindgen(method, setter)]
    pub fn set_verbose(this: &SimpleConfig, val: bool);
}
impl SimpleConfig {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> SimpleConfigBuilder {
        SimpleConfigBuilder { inner: Self::new() }
    }
}
pub struct SimpleConfigBuilder {
    inner: SimpleConfig,
}
#[allow(unused_mut)]
impl SimpleConfigBuilder {
    pub fn verbose(mut self, val: bool) -> Self {
        self.inner.set_verbose(val);
        self
    }
    pub fn build(self) -> SimpleConfig {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type NotificationOptions;
    #[wasm_bindgen(method, getter)]
    pub fn body(this: &NotificationOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_body(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn icon(this: &NotificationOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_icon(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn tag(this: &NotificationOptions) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_tag(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &NotificationOptions) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &NotificationOptions, val: &JsValue);
}
impl NotificationOptions {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> NotificationOptionsBuilder {
        NotificationOptionsBuilder { inner: Self::new() }
    }
}
pub struct NotificationOptionsBuilder {
    inner: NotificationOptions,
}
#[allow(unused_mut)]
impl NotificationOptionsBuilder {
    pub fn body(mut self, val: &str) -> Self {
        self.inner.set_body(val);
        self
    }
    pub fn icon(mut self, val: &str) -> Self {
        self.inner.set_icon(val);
        self
    }
    pub fn tag(mut self, val: &str) -> Self {
        self.inner.set_tag(val);
        self
    }
    pub fn data(mut self, val: &JsValue) -> Self {
        self.inner.set_data(val);
        self
    }
    pub fn build(self) -> NotificationOptions {
        self.inner
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MutableWidget;
    #[wasm_bindgen(method, getter)]
    pub fn label(this: &MutableWidget) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &MutableWidget, val: &str);
    #[wasm_bindgen(method, setter, js_name = "label")]
    pub fn set_label_with_f64(this: &MutableWidget, val: f64);
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MutableWidget) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn callback(this: &MutableWidget) -> Function;
    #[wasm_bindgen(method, setter)]
    pub fn set_callback(this: &MutableWidget, val: &Function);
}
impl MutableWidget {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            JsValue::from(js_sys::Object::new()).unchecked_into()
        }
    }
}
pub mod my_module {
    use super::*;
    use js_sys::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "my-module")]
    extern "C" {
        #[wasm_bindgen(js_name = "doWork")]
        pub fn do_work(input: &str) -> Promise<JsString>;
    }
    #[wasm_bindgen(module = "my-module")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "doWork")]
        pub fn try_do_work(input: &str) -> Result<Promise<JsString>, JsValue>;
    }
    #[wasm_bindgen(module = "my-module")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type WorkResult;
        #[wasm_bindgen(method, getter)]
        pub fn success(this: &WorkResult) -> bool;
        #[wasm_bindgen(method, setter)]
        pub fn set_success(this: &WorkResult, val: bool);
        #[wasm_bindgen(method, getter)]
        pub fn data(this: &WorkResult) -> Option<String>;
        #[wasm_bindgen(method, setter)]
        pub fn set_data(this: &WorkResult, val: &str);
    }
    impl WorkResult {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_imports)]
            use wasm_bindgen::JsCast;
            JsCast::unchecked_into(js_sys::Object::new())
        }
        pub fn builder() -> WorkResultBuilder {
            WorkResultBuilder {
                inner: Self::new(),
                required: 1u64,
            }
        }
    }
    pub struct WorkResultBuilder {
        inner: WorkResult,
        required: u64,
    }
    #[allow(unused_mut)]
    impl WorkResultBuilder {
        pub fn success(mut self, val: bool) -> Self {
            self.inner.set_success(val);
            self.required &= 18446744073709551614u64;
            self
        }
        pub fn data(mut self, val: &str) -> Self {
            self.inner.set_data(val);
            self
        }
        pub fn build(self) -> Result<WorkResult, JsValue> {
            if self.required != 0 {
                let mut missing = Vec::new();
                if self.required & 1u64 != 0 {
                    missing.push("missing required property `success`");
                }
                return Err(JsValue::from_str(&format!(
                    "{}: {}",
                    stringify!(WorkResult),
                    missing.join(", ")
                )));
            }
            Ok(self.inner)
        }
    }
}
