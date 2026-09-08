#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "NavigateEvent",
        typescript_type = "NavigateEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigateEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub type NavigateEvent;
    #[cfg(feature = "NavigationApiType")]
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "navigationType")]
    #[doc = "Getter for the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/navigationType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`, `NavigationApiType`*"]
    pub fn navigation_type(this: &NavigateEvent) -> NavigationApiType;
    #[cfg(feature = "NavigationDestination")]
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "destination")]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`, `NavigationDestination`*"]
    pub fn destination(this: &NavigateEvent) -> NavigationDestination;
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "canIntercept")]
    #[doc = "Getter for the `canIntercept` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/canIntercept)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn can_intercept(this: &NavigateEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "userInitiated")]
    #[doc = "Getter for the `userInitiated` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/userInitiated)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn user_initiated(this: &NavigateEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "hashChange")]
    #[doc = "Getter for the `hashChange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/hashChange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn hash_change(this: &NavigateEvent) -> bool;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "signal")]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `NavigateEvent`*"]
    pub fn signal(this: &NavigateEvent) -> AbortSignal;
    #[cfg(feature = "FormData")]
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "formData")]
    #[doc = "Getter for the `formData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `NavigateEvent`*"]
    pub fn form_data(this: &NavigateEvent) -> Option<FormData>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigateEvent",
        js_name = "downloadRequest"
    )]
    #[doc = "Getter for the `downloadRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/downloadRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn download_request(this: &NavigateEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigateEvent",
        js_name = "downloadRequest"
    )]
    #[doc = "Like `download_request()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/downloadRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn download_request_js_string(this: &NavigateEvent) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "info")]
    #[doc = "Getter for the `info` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/info)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn info(this: &NavigateEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigateEvent",
        js_name = "hasUAVisualTransition"
    )]
    #[doc = "Getter for the `hasUAVisualTransition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/hasUAVisualTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn has_ua_visual_transition(this: &NavigateEvent) -> bool;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "NavigateEvent", js_name = "sourceElement")]
    #[doc = "Getter for the `sourceElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/sourceElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NavigateEvent`*"]
    pub fn source_element(this: &NavigateEvent) -> Option<Element>;
    #[cfg(feature = "NavigateEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "NavigateEvent")]
    #[doc = "The `new NavigateEvent(..)` constructor, creating a new instance of `NavigateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/NavigateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`, `NavigateEventInit`*"]
    pub fn new(type_: &str, event_init_dict: &NavigateEventInit) -> Result<NavigateEvent, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "NavigateEvent")]
    #[doc = "The `intercept()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/intercept)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn intercept(this: &NavigateEvent) -> Result<(), JsValue>;
    #[cfg(feature = "NavigationInterceptOptions")]
    #[wasm_bindgen(catch, method, js_class = "NavigateEvent", js_name = "intercept")]
    #[doc = "The `intercept()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/intercept)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`, `NavigationInterceptOptions`*"]
    pub fn intercept_with_options(
        this: &NavigateEvent,
        options: &NavigationInterceptOptions,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "NavigateEvent")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEvent`*"]
    pub fn scroll(this: &NavigateEvent) -> Result<(), JsValue>;
}
