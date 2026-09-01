#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigationResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    pub type NavigationResult;
    #[doc = "Get the `committed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    #[wasm_bindgen(method, getter = "committed")]
    pub fn get_committed(this: &NavigationResult) -> Option<::js_sys::Promise>;
    #[doc = "Change the `committed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    #[wasm_bindgen(method, setter = "committed")]
    pub fn set_committed(this: &NavigationResult, val: &::js_sys::Promise);
    #[doc = "Get the `finished` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    #[wasm_bindgen(method, getter = "finished")]
    pub fn get_finished(this: &NavigationResult) -> Option<::js_sys::Promise>;
    #[doc = "Change the `finished` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    #[wasm_bindgen(method, setter = "finished")]
    pub fn set_finished(this: &NavigationResult, val: &::js_sys::Promise);
}
impl NavigationResult {
    #[doc = "Construct a new `NavigationResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_committed()` instead."]
    pub fn committed(&mut self, val: &::js_sys::Promise) -> &mut Self {
        self.set_committed(val);
        self
    }
    #[deprecated = "Use `set_finished()` instead."]
    pub fn finished(&mut self, val: &::js_sys::Promise) -> &mut Self {
        self.set_finished(val);
        self
    }
}
impl Default for NavigationResult {
    fn default() -> Self {
        Self::new()
    }
}
