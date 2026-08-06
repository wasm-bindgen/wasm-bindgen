#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "NavigationHistoryEntry",
        typescript_type = "NavigationHistoryEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationHistoryEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub type NavigationHistoryEntry;
    #[wasm_bindgen(method, getter, js_class = "NavigationHistoryEntry", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn url(this: &NavigationHistoryEntry) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "NavigationHistoryEntry", js_name = "key")]
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn key(this: &NavigationHistoryEntry) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "NavigationHistoryEntry", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn id(this: &NavigationHistoryEntry) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "NavigationHistoryEntry", js_name = "index")]
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn index(this: &NavigationHistoryEntry) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationHistoryEntry",
        js_name = "sameDocument"
    )]
    #[doc = "Getter for the `sameDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/sameDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn same_document(this: &NavigationHistoryEntry) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationHistoryEntry",
        js_name = "ondispose"
    )]
    #[doc = "Getter for the `ondispose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/ondispose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn ondispose(this: &NavigationHistoryEntry) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "NavigationHistoryEntry",
        js_name = "ondispose"
    )]
    #[doc = "Setter for the `ondispose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/ondispose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn set_ondispose(this: &NavigationHistoryEntry, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, js_class = "NavigationHistoryEntry", js_name = "getState")]
    #[doc = "The `getState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/getState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`*"]
    pub fn get_state(this: &NavigationHistoryEntry) -> ::wasm_bindgen::JsValue;
}
