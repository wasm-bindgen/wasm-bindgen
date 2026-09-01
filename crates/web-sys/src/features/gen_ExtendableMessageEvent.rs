#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "ExtendableEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ExtendableMessageEvent",
        typescript_type = "ExtendableMessageEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub type ExtendableMessageEvent;
    #[wasm_bindgen(method, getter, js_class = "ExtendableMessageEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn data(this: &ExtendableMessageEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableMessageEvent",
        js_name = "origin"
    )]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn origin(this: &ExtendableMessageEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableMessageEvent",
        js_name = "origin"
    )]
    #[doc = "Like `origin()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn origin_js_string(this: &ExtendableMessageEvent) -> ::js_sys::JsString;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableMessageEvent",
        js_name = "lastEventId"
    )]
    #[doc = "Getter for the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn last_event_id(this: &ExtendableMessageEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableMessageEvent",
        js_name = "lastEventId"
    )]
    #[doc = "Like `last_event_id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn last_event_id_js_string(this: &ExtendableMessageEvent) -> ::js_sys::JsString;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableMessageEvent",
        js_name = "source"
    )]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn source(this: &ExtendableMessageEvent) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, getter, js_class = "ExtendableMessageEvent", js_name = "ports")]
    #[doc = "Getter for the `ports` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn ports(this: &ExtendableMessageEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableMessageEvent")]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn new(type_: &str) -> Result<ExtendableMessageEvent, JsValue>;
    #[cfg(feature = "ExtendableMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableMessageEvent")]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`, `ExtendableMessageEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableMessageEventInit,
    ) -> Result<ExtendableMessageEvent, JsValue>;
}
