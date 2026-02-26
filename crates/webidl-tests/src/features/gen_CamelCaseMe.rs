#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = camel_case_me)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CamelCaseMe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CamelCaseMe`*"]
    pub type CamelCaseMe;
    #[doc = "Get the `snakeCaseMe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CamelCaseMe`*"]
    #[wasm_bindgen(method, getter = "snakeCaseMe")]
    pub fn get_snake_case_me(this: &CamelCaseMe) -> Option<i32>;
    #[doc = "Change the `snakeCaseMe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CamelCaseMe`*"]
    #[wasm_bindgen(method, setter = "snakeCaseMe")]
    pub fn set_snake_case_me(this: &CamelCaseMe, val: i32);
}
impl CamelCaseMe {
    #[doc = "Construct a new `CamelCaseMe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CamelCaseMe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_snake_case_me()` instead."]
    pub fn snake_case_me(&mut self, val: i32) -> &mut Self {
        self.set_snake_case_me(val);
        self
    }
}
impl Default for CamelCaseMe {
    fn default() -> Self {
        Self::new()
    }
}
