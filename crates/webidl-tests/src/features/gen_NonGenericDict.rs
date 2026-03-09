#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NonGenericDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NonGenericDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    pub type NonGenericDict;
    #[doc = "Get the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &NonGenericDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &NonGenericDict, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &NonGenericDict) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &NonGenericDict, val: &str);
}
impl NonGenericDict {
    #[doc = "Construct a new `NonGenericDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NonGenericDict`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_items(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
