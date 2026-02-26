#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GenericDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GenericDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    pub type GenericDict;
    #[doc = "Get the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &GenericDict) -> Option<::js_sys::Array<::js_sys::Number>>;
    #[doc = "Change the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &GenericDict, val: &[::js_sys::Number]);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &GenericDict) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &GenericDict, val: &str);
}
impl GenericDict {
    #[doc = "Construct a new `GenericDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GenericDict`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &[::js_sys::Number]) -> &mut Self {
        self.set_items(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
