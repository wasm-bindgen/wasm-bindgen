#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PopupBlockedEvent",
        typescript_type = "PopupBlockedEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub type PopupBlockedEvent;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "requestingWindow"
    )]
    #[doc = "Getter for the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*"]
    pub fn requesting_window(this: &PopupBlockedEvent) -> Option<Window>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowURI"
    )]
    #[doc = "Getter for the `popupWindowURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_uri(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowURI"
    )]
    #[doc = "Like `popup_window_uri()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_uri_js_string(this: &PopupBlockedEvent) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowName"
    )]
    #[doc = "Getter for the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_name(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowName"
    )]
    #[doc = "Like `popup_window_name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_name_js_string(this: &PopupBlockedEvent) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowFeatures"
    )]
    #[doc = "Getter for the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_features(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowFeatures"
    )]
    #[doc = "Like `popup_window_features()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_features_js_string(this: &PopupBlockedEvent) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn new(type_: &str) -> Result<PopupBlockedEvent, JsValue>;
    #[cfg(feature = "PopupBlockedEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopupBlockedEventInit,
    ) -> Result<PopupBlockedEvent, JsValue>;
}
