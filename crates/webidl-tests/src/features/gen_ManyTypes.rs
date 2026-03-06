#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ManyTypes)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ManyTypes` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    pub type ManyTypes;
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &ManyTypes) -> Option<::alloc::string::String>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &ManyTypes, val: &str);
    #[doc = "Get the `n1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n1")]
    pub fn get_n1(this: &ManyTypes) -> Option<u8>;
    #[doc = "Change the `n1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n1")]
    pub fn set_n1(this: &ManyTypes, val: u8);
    #[doc = "Get the `n2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n2")]
    pub fn get_n2(this: &ManyTypes) -> Option<i8>;
    #[doc = "Change the `n2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n2")]
    pub fn set_n2(this: &ManyTypes, val: i8);
    #[doc = "Get the `n3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n3")]
    pub fn get_n3(this: &ManyTypes) -> Option<u16>;
    #[doc = "Change the `n3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n3")]
    pub fn set_n3(this: &ManyTypes, val: u16);
    #[doc = "Get the `n4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n4")]
    pub fn get_n4(this: &ManyTypes) -> Option<i16>;
    #[doc = "Change the `n4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n4")]
    pub fn set_n4(this: &ManyTypes, val: i16);
    #[doc = "Get the `n5` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n5")]
    pub fn get_n5(this: &ManyTypes) -> Option<u32>;
    #[doc = "Change the `n5` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n5")]
    pub fn set_n5(this: &ManyTypes, val: u32);
    #[doc = "Get the `n6` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, getter = "n6")]
    pub fn get_n6(this: &ManyTypes) -> Option<i32>;
    #[doc = "Change the `n6` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    #[wasm_bindgen(method, setter = "n6")]
    pub fn set_n6(this: &ManyTypes, val: i32);
}
impl ManyTypes {
    #[doc = "Construct a new `ManyTypes`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ManyTypes`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_a()` instead."]
    pub fn a(&mut self, val: &str) -> &mut Self {
        self.set_a(val);
        self
    }
    #[deprecated = "Use `set_n1()` instead."]
    pub fn n1(&mut self, val: u8) -> &mut Self {
        self.set_n1(val);
        self
    }
    #[deprecated = "Use `set_n2()` instead."]
    pub fn n2(&mut self, val: i8) -> &mut Self {
        self.set_n2(val);
        self
    }
    #[deprecated = "Use `set_n3()` instead."]
    pub fn n3(&mut self, val: u16) -> &mut Self {
        self.set_n3(val);
        self
    }
    #[deprecated = "Use `set_n4()` instead."]
    pub fn n4(&mut self, val: i16) -> &mut Self {
        self.set_n4(val);
        self
    }
    #[deprecated = "Use `set_n5()` instead."]
    pub fn n5(&mut self, val: u32) -> &mut Self {
        self.set_n5(val);
        self
    }
    #[deprecated = "Use `set_n6()` instead."]
    pub fn n6(&mut self, val: i32) -> &mut Self {
        self.set_n6(val);
        self
    }
}
impl Default for ManyTypes {
    fn default() -> Self {
        Self::new()
    }
}
