#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NamedConstructor , typescript_type = "NamedConstructor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NamedConstructor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedConstructor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedConstructor`*"]
    pub type NamedConstructor;
    # [wasm_bindgen (structural , method , getter , js_class = "NamedConstructor" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedConstructor/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedConstructor`*"]
    pub fn value(this: &NamedConstructor) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "NamedConstructorBar")]
    #[doc = "The `new NamedConstructor(..)` constructor, creating a new instance of `NamedConstructor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedConstructor/NamedConstructor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedConstructor`*"]
    pub fn new(value: f64) -> Result<NamedConstructor, JsValue>;
}
