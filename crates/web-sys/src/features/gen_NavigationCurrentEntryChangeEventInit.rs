#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigationCurrentEntryChangeEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationCurrentEntryChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    pub type NavigationCurrentEntryChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &NavigationCurrentEntryChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &NavigationCurrentEntryChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &NavigationCurrentEntryChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &NavigationCurrentEntryChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &NavigationCurrentEntryChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &NavigationCurrentEntryChangeEventInit, val: bool);
    #[cfg(feature = "NavigationHistoryEntry")]
    #[doc = "Get the `from` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`, `NavigationHistoryEntry`*"]
    #[wasm_bindgen(method, getter = "from")]
    pub fn get_from(this: &NavigationCurrentEntryChangeEventInit) -> NavigationHistoryEntry;
    #[cfg(feature = "NavigationHistoryEntry")]
    #[doc = "Change the `from` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`, `NavigationHistoryEntry`*"]
    #[wasm_bindgen(method, setter = "from")]
    pub fn set_from(this: &NavigationCurrentEntryChangeEventInit, val: &NavigationHistoryEntry);
    #[cfg(feature = "NavigationApiType")]
    #[doc = "Get the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationApiType`, `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "navigationType")]
    pub fn get_navigation_type(
        this: &NavigationCurrentEntryChangeEventInit,
    ) -> Option<NavigationApiType>;
    #[cfg(feature = "NavigationApiType")]
    #[doc = "Change the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationApiType`, `NavigationCurrentEntryChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "navigationType")]
    pub fn set_navigation_type(
        this: &NavigationCurrentEntryChangeEventInit,
        val: Option<NavigationApiType>,
    );
}
impl NavigationCurrentEntryChangeEventInit {
    #[cfg(feature = "NavigationHistoryEntry")]
    #[doc = "Construct a new `NavigationCurrentEntryChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationCurrentEntryChangeEventInit`, `NavigationHistoryEntry`*"]
    pub fn new(from: &NavigationHistoryEntry) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_from(from);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "NavigationHistoryEntry")]
    #[deprecated = "Use `set_from()` instead."]
    pub fn from(&mut self, val: &NavigationHistoryEntry) -> &mut Self {
        self.set_from(val);
        self
    }
    #[cfg(feature = "NavigationApiType")]
    #[deprecated = "Use `set_navigation_type()` instead."]
    pub fn navigation_type(&mut self, val: Option<NavigationApiType>) -> &mut Self {
        self.set_navigation_type(val);
        self
    }
}
