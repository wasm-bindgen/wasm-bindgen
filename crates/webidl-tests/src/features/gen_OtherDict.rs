#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OtherDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OtherDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OtherDict`*"]
    pub type OtherDict;
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OtherDict`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &OtherDict) -> Option<i32>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OtherDict`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &OtherDict, val: i32);
}
impl OtherDict {
    #[doc = "Construct a new `OtherDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OtherDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_a()` instead."]
    pub fn a(&mut self, val: i32) -> &mut Self {
        self.set_a(val);
        self
    }
}
impl Default for OtherDict {
    fn default() -> Self {
        Self::new()
    }
}
