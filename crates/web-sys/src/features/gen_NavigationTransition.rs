#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigationTransition",
        typescript_type = "NavigationTransition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationTransition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationTransition`*"]
    pub type NavigationTransition;
    #[cfg(feature = "NavigationApiType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationTransition",
        js_name = "navigationType"
    )]
    #[doc = "Getter for the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/navigationType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationApiType`, `NavigationTransition`*"]
    pub fn navigation_type(this: &NavigationTransition) -> NavigationApiType;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[wasm_bindgen(method, getter, js_class = "NavigationTransition", js_name = "from")]
    #[doc = "Getter for the `from` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/from)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryEntry`, `NavigationTransition`*"]
    pub fn from(this: &NavigationTransition) -> NavigationHistoryEntry;
    #[cfg(feature = "NavigationDestination")]
    #[wasm_bindgen(method, getter, js_class = "NavigationTransition", js_name = "to")]
    #[doc = "Getter for the `to` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/to)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationDestination`, `NavigationTransition`*"]
    pub fn to(this: &NavigationTransition) -> NavigationDestination;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationTransition",
        js_name = "committed"
    )]
    #[doc = "Getter for the `committed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/committed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationTransition`*"]
    pub fn committed(this: &NavigationTransition) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationTransition",
        js_name = "finished"
    )]
    #[doc = "Getter for the `finished` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/finished)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationTransition`*"]
    pub fn finished(this: &NavigationTransition) -> ::js_sys::Promise;
}
