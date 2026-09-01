#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigationReloadOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationReloadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
    pub type NavigationReloadOptions;
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &NavigationReloadOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &NavigationReloadOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &NavigationReloadOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &NavigationReloadOptions, val: &::wasm_bindgen::JsValue);
}
impl NavigationReloadOptions {
    #[doc = "Construct a new `NavigationReloadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationReloadOptions`*"]
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
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_state(val);
        self
    }
}
impl Default for NavigationReloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
