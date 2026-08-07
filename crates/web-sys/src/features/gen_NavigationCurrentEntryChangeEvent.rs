#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "NavigationCurrentEntryChangeEvent",
        typescript_type = "NavigationCurrentEntryChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationCurrentEntryChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEvent`*"]
    pub type NavigationCurrentEntryChangeEvent;
    #[cfg(feature = "NavigationApiType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationCurrentEntryChangeEvent",
        js_name = "navigationType"
    )]
    #[doc = "Getter for the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/navigationType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationApiType`, `NavigationCurrentEntryChangeEvent`*"]
    pub fn navigation_type(this: &NavigationCurrentEntryChangeEvent) -> Option<NavigationApiType>;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigationCurrentEntryChangeEvent",
        js_name = "from"
    )]
    #[doc = "Getter for the `from` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/from)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEvent`, `NavigationHistoryEntry`*"]
    pub fn from(this: &NavigationCurrentEntryChangeEvent) -> NavigationHistoryEntry;
    #[cfg(feature = "NavigationCurrentEntryChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "NavigationCurrentEntryChangeEvent")]
    #[doc = "The `new NavigationCurrentEntryChangeEvent(..)` constructor, creating a new instance of `NavigationCurrentEntryChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/NavigationCurrentEntryChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEvent`, `NavigationCurrentEntryChangeEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &NavigationCurrentEntryChangeEventInit,
    ) -> Result<NavigationCurrentEntryChangeEvent, JsValue>;
    #[cfg(feature = "NavigationCurrentEntryChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "NavigationCurrentEntryChangeEvent")]
    #[doc = "The `new NavigationCurrentEntryChangeEvent(..)` constructor, creating a new instance of `NavigationCurrentEntryChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigationCurrentEntryChangeEvent/NavigationCurrentEntryChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEvent`, `NavigationCurrentEntryChangeEventInit`*"]
    pub fn new_with_js_string(
        type_: &::js_sys::JsString,
        event_init_dict: &NavigationCurrentEntryChangeEventInit,
    ) -> Result<NavigationCurrentEntryChangeEvent, JsValue>;
}
