#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GlobalMethod , typescript_type = "GlobalMethod")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GlobalMethod` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GlobalMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GlobalMethod`*"]
    pub type GlobalMethod;
    #[wasm_bindgen(catch, constructor, js_class = "GlobalMethod")]
    #[doc = "The `new GlobalMethod(..)` constructor, creating a new instance of `GlobalMethod`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GlobalMethod/GlobalMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GlobalMethod`*"]
    pub fn new() -> Result<GlobalMethod, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "GlobalMethod" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GlobalMethod/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GlobalMethod`*"]
    pub fn m(this: &GlobalMethod) -> u8;
}
