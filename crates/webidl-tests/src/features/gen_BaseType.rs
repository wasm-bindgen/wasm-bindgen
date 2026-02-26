#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BaseType , typescript_type = "BaseType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseType` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseType`*"]
    pub type BaseType;
    # [wasm_bindgen (structural , method , getter , js_class = "BaseType" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseType/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseType`*"]
    pub fn value(this: &BaseType) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "BaseType" , js_name = value)]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseType/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseType`*"]
    pub fn set_value(this: &BaseType, value: &str);
    #[wasm_bindgen(catch, constructor, js_class = "BaseType")]
    #[doc = "The `new BaseType(..)` constructor, creating a new instance of `BaseType`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseType/BaseType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseType`*"]
    pub fn new() -> Result<BaseType, JsValue>;
}
