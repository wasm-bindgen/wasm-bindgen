#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = JsOptionMethod , typescript_type = "JsOptionMethod")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `JsOptionMethod` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/JsOptionMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsOptionMethod`*"]
    pub type JsOptionMethod;
    #[wasm_bindgen(catch, constructor, js_class = "JsOptionMethod")]
    #[doc = "The `new JsOptionMethod(..)` constructor, creating a new instance of `JsOptionMethod`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/JsOptionMethod/JsOptionMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsOptionMethod`*"]
    pub fn new() -> Result<JsOptionMethod, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "JsOptionMethod" , js_name = opt)]
    #[doc = "The `opt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/JsOptionMethod/opt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsOptionMethod`*"]
    pub fn opt(this: &JsOptionMethod, a: Option<i16>) -> Option<u8>;
}
