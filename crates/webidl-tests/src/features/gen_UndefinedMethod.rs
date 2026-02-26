#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UndefinedMethod , typescript_type = "UndefinedMethod")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UndefinedMethod` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UndefinedMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UndefinedMethod`*"]
    pub type UndefinedMethod;
    #[wasm_bindgen(catch, constructor, js_class = "UndefinedMethod")]
    #[doc = "The `new UndefinedMethod(..)` constructor, creating a new instance of `UndefinedMethod`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UndefinedMethod/UndefinedMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UndefinedMethod`*"]
    pub fn new() -> Result<UndefinedMethod, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "UndefinedMethod" , js_name = ok_method)]
    #[doc = "The `ok_method()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UndefinedMethod/ok_method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UndefinedMethod`*"]
    pub fn ok_method(this: &UndefinedMethod) -> bool;
}
