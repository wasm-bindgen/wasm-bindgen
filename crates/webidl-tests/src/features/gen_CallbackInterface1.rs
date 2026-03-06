#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CallbackInterface1)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CallbackInterface1` dictionary.\n\n*This API requires the following crate features to be activated: `CallbackInterface1`*"]
    pub type CallbackInterface1;
    #[doc = "Get the `foo` field of this object.\n\n*This API requires the following crate features to be activated: `CallbackInterface1`*"]
    #[wasm_bindgen(method, getter = "foo")]
    pub fn get_foo(this: &CallbackInterface1) -> Option<::js_sys::Function>;
    #[doc = "Change the `foo` field of this object.\n\n*This API requires the following crate features to be activated: `CallbackInterface1`*"]
    #[wasm_bindgen(method, setter = "foo")]
    pub fn set_foo(this: &CallbackInterface1, val: &::js_sys::Function);
}
impl CallbackInterface1 {
    #[doc = "Construct a new `CallbackInterface1`.\n\n*This API requires the following crate features to be activated: `CallbackInterface1`*"]
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
impl Default for CallbackInterface1 {
    fn default() -> Self {
        Self::new()
    }
}
