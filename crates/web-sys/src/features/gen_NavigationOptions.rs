#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigationOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationOptions`*"]
    pub type NavigationOptions;
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationOptions`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &NavigationOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationOptions`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &NavigationOptions, val: &::wasm_bindgen::JsValue);
}
impl NavigationOptions {
    #[doc = "Construct a new `NavigationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationOptions`*"]
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
}
impl Default for NavigationOptions {
    fn default() -> Self {
        Self::new()
    }
}
