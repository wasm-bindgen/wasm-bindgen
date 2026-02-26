#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Property , typescript_type = "Property")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Property` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Property)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Property`*"]
    pub type Property;
    # [wasm_bindgen (structural , method , getter , js_class = "Property" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Property/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Property`*"]
    pub fn value(this: &Property) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "Property" , js_name = value)]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Property/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Property`*"]
    pub fn set_value(this: &Property, value: f64);
    #[wasm_bindgen(catch, constructor, js_class = "Property")]
    #[doc = "The `new Property(..)` constructor, creating a new instance of `Property`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Property/Property)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Property`*"]
    pub fn new(value: f64) -> Result<Property, JsValue>;
}
