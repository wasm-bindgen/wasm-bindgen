#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CallbackInterface2)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CallbackInterface2` dictionary.\n\n*This API requires the following crate features to be activated: `CallbackInterface2`*"]
    pub type CallbackInterface2;
    #[doc = "Get the `foo` field of this object.\n\n*This API requires the following crate features to be activated: `CallbackInterface2`*"]
    #[wasm_bindgen(method, getter = "foo")]
    pub fn get_foo(this: &CallbackInterface2) -> Option<::js_sys::Function>;
    #[doc = "Change the `foo` field of this object.\n\n*This API requires the following crate features to be activated: `CallbackInterface2`*"]
    #[wasm_bindgen(method, setter = "foo")]
    pub fn set_foo(this: &CallbackInterface2, val: &::js_sys::Function);
}
impl CallbackInterface2 {
    #[doc = "Construct a new `CallbackInterface2`.\n\n*This API requires the following crate features to be activated: `CallbackInterface2`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_foo()` instead."]
    pub fn foo(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_foo(val);
        self
    }
}
impl Default for CallbackInterface2 {
    fn default() -> Self {
        Self::new()
    }
}
