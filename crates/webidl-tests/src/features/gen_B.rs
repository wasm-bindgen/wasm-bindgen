#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = B)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `B` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    pub type B;
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &B) -> Option<i32>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &B, val: i32);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &B) -> Option<i32>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &B, val: i32);
    #[doc = "Get the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "g")]
    pub fn get_g(this: &B) -> Option<i32>;
    #[doc = "Change the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "g")]
    pub fn set_g(this: &B, val: i32);
    #[doc = "Get the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "h")]
    pub fn get_h(this: &B) -> Option<i32>;
    #[doc = "Change the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "h")]
    pub fn set_h(this: &B, val: i32);
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &B) -> Option<i32>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &B, val: i32);
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, getter = "b")]
    pub fn get_b(this: &B) -> Option<i32>;
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
    #[wasm_bindgen(method, setter = "b")]
    pub fn set_b(this: &B, val: i32);
}
impl B {
    #[doc = "Construct a new `B`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `B`*"]
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
    #[deprecated = "Use `set_a()` instead."]
    pub fn a(&mut self, val: i32) -> &mut Self {
        self.set_a(val);
        self
    }
    #[deprecated = "Use `set_b()` instead."]
    pub fn b(&mut self, val: i32) -> &mut Self {
        self.set_b(val);
        self
    }
}
impl Default for B {
    fn default() -> Self {
        Self::new()
    }
}
