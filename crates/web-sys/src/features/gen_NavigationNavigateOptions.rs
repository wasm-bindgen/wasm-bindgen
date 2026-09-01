#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigationNavigateOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationNavigateOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    pub type NavigationNavigateOptions;
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &NavigationNavigateOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &NavigationNavigateOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "NavigationHistoryBehavior")]
    #[doc = "Get the `history` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryBehavior`, `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, getter = "history")]
    pub fn get_history(this: &NavigationNavigateOptions) -> Option<NavigationHistoryBehavior>;
    #[cfg(feature = "NavigationHistoryBehavior")]
    #[doc = "Change the `history` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationHistoryBehavior`, `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, setter = "history")]
    pub fn set_history(this: &NavigationNavigateOptions, val: NavigationHistoryBehavior);
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &NavigationNavigateOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &NavigationNavigateOptions, val: &::wasm_bindgen::JsValue);
}
impl NavigationNavigateOptions {
    #[doc = "Construct a new `NavigationNavigateOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationNavigateOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_info()` instead."]
    pub fn info(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_info(val);
        self
    }
    #[cfg(feature = "NavigationHistoryBehavior")]
    #[deprecated = "Use `set_history()` instead."]
    pub fn history(&mut self, val: NavigationHistoryBehavior) -> &mut Self {
        self.set_history(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_state(val);
        self
    }
}
impl Default for NavigationNavigateOptions {
    fn default() -> Self {
        Self::new()
    }
}
