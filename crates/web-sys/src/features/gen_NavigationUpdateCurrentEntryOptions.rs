#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigationUpdateCurrentEntryOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationUpdateCurrentEntryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationUpdateCurrentEntryOptions`*"]
    pub type NavigationUpdateCurrentEntryOptions;
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationUpdateCurrentEntryOptions`*"]
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &NavigationUpdateCurrentEntryOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationUpdateCurrentEntryOptions`*"]
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &NavigationUpdateCurrentEntryOptions, val: &::wasm_bindgen::JsValue);
}
impl NavigationUpdateCurrentEntryOptions {
    #[doc = "Construct a new `NavigationUpdateCurrentEntryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationUpdateCurrentEntryOptions`*"]
    pub fn new(state: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_state(state);
        ret
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_state(val);
        self
    }
}
