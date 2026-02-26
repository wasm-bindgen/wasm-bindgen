#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PreserveNames)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PreserveNames` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PreserveNames`*"]
    pub type PreserveNames;
    #[doc = "Get the `weird_fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PreserveNames`*"]
    #[wasm_bindgen(method, getter = "weird_fieldName")]
    pub fn get_weird_field_name(this: &PreserveNames) -> Option<i32>;
    #[doc = "Change the `weird_fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PreserveNames`*"]
    #[wasm_bindgen(method, setter = "weird_fieldName")]
    pub fn set_weird_field_name(this: &PreserveNames, val: i32);
}
impl PreserveNames {
    #[doc = "Construct a new `PreserveNames`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PreserveNames`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_weird_field_name()` instead."]
    pub fn weird_field_name(&mut self, val: i32) -> &mut Self {
        self.set_weird_field_name(val);
        self
    }
}
impl Default for PreserveNames {
    fn default() -> Self {
        Self::new()
    }
}
