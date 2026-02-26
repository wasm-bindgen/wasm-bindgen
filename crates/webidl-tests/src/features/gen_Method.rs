#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Method , typescript_type = "Method")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Method` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Method`*"]
    pub type Method;
    #[wasm_bindgen(catch, constructor, js_class = "Method")]
    #[doc = "The `new Method(..)` constructor, creating a new instance of `Method`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Method/Method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Method`*"]
    pub fn new(value: f64) -> Result<Method, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "Method" , js_name = myCmp)]
    #[doc = "The `myCmp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Method/myCmp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Method`*"]
    pub fn my_cmp(this: &Method, bar: &Method) -> bool;
}
