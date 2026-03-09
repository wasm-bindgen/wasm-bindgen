#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ChildType , extends = BaseType , extends = :: js_sys :: Object , js_name = GrandChildType , typescript_type = "GrandChildType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GrandChildType` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GrandChildType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GrandChildType`*"]
    pub type GrandChildType;
    # [wasm_bindgen (structural , method , getter , js_class = "GrandChildType" , js_name = grandChildValue)]
    #[doc = "Getter for the `grandChildValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GrandChildType/grandChildValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GrandChildType`*"]
    pub fn grand_child_value(this: &GrandChildType) -> bool;
    # [wasm_bindgen (structural , method , setter , js_class = "GrandChildType" , js_name = grandChildValue)]
    #[doc = "Setter for the `grandChildValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GrandChildType/grandChildValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GrandChildType`*"]
    pub fn set_grand_child_value(this: &GrandChildType, value: bool);
    #[wasm_bindgen(catch, constructor, js_class = "GrandChildType")]
    #[doc = "The `new GrandChildType(..)` constructor, creating a new instance of `GrandChildType`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GrandChildType/GrandChildType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GrandChildType`*"]
    pub fn new() -> Result<GrandChildType, JsValue>;
}
