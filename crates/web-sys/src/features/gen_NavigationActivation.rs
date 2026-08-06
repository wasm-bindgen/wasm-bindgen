#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigationActivation",
        typescript_type = "NavigationActivation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationActivation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationActivation`*"]
    pub type NavigationActivation;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[wasm_bindgen(method, getter, js_class = "NavigationActivation", js_name = "from")]
    #[doc = "Getter for the `from` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/from)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationActivation`, `NavigationHistoryEntry`*"]
    pub fn from(this: &NavigationActivation) -> Option<NavigationHistoryEntry>;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[wasm_bindgen(method, getter, js_class = "NavigationActivation", js_name = "entry")]
    #[doc = "Getter for the `entry` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/entry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationActivation`, `NavigationHistoryEntry`*"]
    pub fn entry(this: &NavigationActivation) -> NavigationHistoryEntry;
    #[cfg(feature = "NavigationApiType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationActivation",
        js_name = "navigationType"
    )]
    #[doc = "Getter for the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/navigationType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationActivation`, `NavigationApiType`*"]
    pub fn navigation_type(this: &NavigationActivation) -> NavigationApiType;
}
