#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = A)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `A` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    pub type A;
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &A) -> Option<i32>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &A, val: i32);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &A) -> Option<i32>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &A, val: i32);
    #[doc = "Get the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, getter = "g")]
    pub fn get_g(this: &A) -> Option<i32>;
    #[doc = "Change the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, setter = "g")]
    pub fn set_g(this: &A, val: i32);
    #[doc = "Get the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, getter = "h")]
    pub fn get_h(this: &A) -> Option<i32>;
    #[doc = "Change the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    #[wasm_bindgen(method, setter = "h")]
    pub fn set_h(this: &A, val: i32);
}
impl A {
    #[doc = "Construct a new `A`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `A`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_c()` instead."]
    pub fn c(&mut self, val: i32) -> &mut Self {
        self.set_c(val);
        self
    }
    #[deprecated = "Use `set_d()` instead."]
    pub fn d(&mut self, val: i32) -> &mut Self {
        self.set_d(val);
        self
    }
    #[deprecated = "Use `set_g()` instead."]
    pub fn g(&mut self, val: i32) -> &mut Self {
        self.set_g(val);
        self
    }
    #[deprecated = "Use `set_h()` instead."]
    pub fn h(&mut self, val: i32) -> &mut Self {
        self.set_h(val);
        self
    }
}
impl Default for A {
    fn default() -> Self {
        Self::new()
    }
}
