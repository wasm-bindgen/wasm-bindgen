#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StaticMethod , typescript_type = "StaticMethod")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StaticMethod` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticMethod`*"]
    pub type StaticMethod;
    # [wasm_bindgen (static_method_of = StaticMethod , js_class = "StaticMethod" , js_name = swap)]
    #[doc = "The `swap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticMethod/swap_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticMethod`*"]
    pub fn swap(value: f64) -> f64;
}
