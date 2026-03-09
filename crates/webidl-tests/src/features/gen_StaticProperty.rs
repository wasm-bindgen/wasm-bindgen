#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StaticProperty , typescript_type = "StaticProperty")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StaticProperty` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticProperty`*"]
    pub type StaticProperty;
    # [wasm_bindgen (structural , static_method_of = StaticProperty , getter , js_class = "StaticProperty" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticProperty/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticProperty`*"]
    pub fn value() -> f64;
    # [wasm_bindgen (structural , static_method_of = StaticProperty , setter , js_class = "StaticProperty" , js_name = value)]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticProperty/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticProperty`*"]
    pub fn set_value(value: f64);
}
