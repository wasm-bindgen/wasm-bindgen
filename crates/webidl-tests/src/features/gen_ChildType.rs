#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = BaseType , extends = :: js_sys :: Object , js_name = ChildType , typescript_type = "ChildType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChildType` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChildType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChildType`*"]
    pub type ChildType;
    # [wasm_bindgen (structural , method , getter , js_class = "ChildType" , js_name = childValue)]
    #[doc = "Getter for the `childValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChildType/childValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChildType`*"]
    pub fn child_value(this: &ChildType) -> i32;
    # [wasm_bindgen (structural , method , setter , js_class = "ChildType" , js_name = childValue)]
    #[doc = "Setter for the `childValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChildType/childValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChildType`*"]
    pub fn set_child_value(this: &ChildType, value: i32);
    #[wasm_bindgen(catch, constructor, js_class = "ChildType")]
    #[doc = "The `new ChildType(..)` constructor, creating a new instance of `ChildType`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChildType/ChildType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChildType`*"]
    pub fn new() -> Result<ChildType, JsValue>;
}
