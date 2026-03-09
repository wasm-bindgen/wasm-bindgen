#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = C)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `C` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    pub type C;
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &C) -> Option<i32>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &C, val: i32);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &C) -> Option<i32>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &C, val: i32);
    #[doc = "Get the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "g")]
    pub fn get_g(this: &C) -> Option<i32>;
    #[doc = "Change the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "g")]
    pub fn set_g(this: &C, val: i32);
    #[doc = "Get the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "h")]
    pub fn get_h(this: &C) -> Option<i32>;
    #[doc = "Change the `h` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "h")]
    pub fn set_h(this: &C, val: i32);
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &C) -> Option<i32>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &C, val: i32);
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "b")]
    pub fn get_b(this: &C) -> Option<i32>;
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "b")]
    pub fn set_b(this: &C, val: i32);
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "e")]
    pub fn get_e(this: &C) -> Option<i32>;
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "e")]
    pub fn set_e(this: &C, val: i32);
    #[doc = "Get the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, getter = "f")]
    pub fn get_f(this: &C) -> Option<i32>;
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
    #[wasm_bindgen(method, setter = "f")]
    pub fn set_f(this: &C, val: i32);
}
impl C {
    #[doc = "Construct a new `C`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `C`*"]
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
    #[deprecated = "Use `set_e()` instead."]
    pub fn e(&mut self, val: i32) -> &mut Self {
        self.set_e(val);
        self
    }
    #[deprecated = "Use `set_f()` instead."]
    pub fn f(&mut self, val: i32) -> &mut Self {
        self.set_f(val);
        self
    }
}
impl Default for C {
    fn default() -> Self {
        Self::new()
    }
}
