#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Required)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Required` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    pub type Required;
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &Required) -> i32;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &Required, val: i32);
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, getter = "b")]
    pub fn get_b(this: &Required) -> ::alloc::string::String;
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, setter = "b")]
    pub fn set_b(this: &Required, val: &str);
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &Required) -> Option<i32>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &Required, val: i32);
}
impl Required {
    #[doc = "Construct a new `Required`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Required`*"]
    pub fn new(a: i32, b: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_a(a);
        ret.set_b(b);
        ret
    }
    #[deprecated = "Use `set_a()` instead."]
    pub fn a(&mut self, val: i32) -> &mut Self {
        self.set_a(val);
        self
    }
    #[deprecated = "Use `set_b()` instead."]
    pub fn b(&mut self, val: &str) -> &mut Self {
        self.set_b(val);
        self
    }
    #[deprecated = "Use `set_c()` instead."]
    pub fn c(&mut self, val: i32) -> &mut Self {
        self.set_c(val);
        self
    }
}
