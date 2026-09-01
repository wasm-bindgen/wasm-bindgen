#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigationDestination",
        typescript_type = "NavigationDestination"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationDestination` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub type NavigationDestination;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn url(this: &NavigationDestination) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "url")]
    #[doc = "Like `url()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn url_js_string(this: &NavigationDestination) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "key")]
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn key(this: &NavigationDestination) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "key")]
    #[doc = "Like `key()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn key_js_string(this: &NavigationDestination) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn id(this: &NavigationDestination) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "id")]
    #[doc = "Like `id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn id_js_string(this: &NavigationDestination) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "NavigationDestination", js_name = "index")]
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn index(this: &NavigationDestination) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationDestination",
        js_name = "sameDocument"
    )]
    #[doc = "Getter for the `sameDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/sameDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn same_document(this: &NavigationDestination) -> bool;
    #[wasm_bindgen(method, js_class = "NavigationDestination", js_name = "getState")]
    #[doc = "The `getState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/getState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`*"]
    pub fn get_state(this: &NavigationDestination) -> ::wasm_bindgen::JsValue;
}
