#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Global , typescript_type = "Global")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Global` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Global)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Global`*"]
    pub type Global;
    # [wasm_bindgen (structural , method , getter , js_class = "Global" , js_name = global_attribute)]
    #[doc = "Getter for the `global_attribute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Global/global_attribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Global`*"]
    pub fn global_attribute(this: &Global) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "Global" , js_name = global_attribute)]
    #[doc = "Setter for the `global_attribute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Global/global_attribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Global`*"]
    pub fn set_global_attribute(this: &Global, value: &str);
    # [wasm_bindgen (method , structural , js_class = "Global" , js_name = global_no_args)]
    #[doc = "The `global_no_args()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Global/global_no_args)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Global`*"]
    pub fn global_no_args(this: &Global) -> u32;
    # [wasm_bindgen (method , structural , js_class = "Global" , js_name = global_with_args)]
    #[doc = "The `global_with_args()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Global/global_with_args)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Global`*"]
    pub fn global_with_args(this: &Global, a: &str, b: &str) -> ::alloc::string::String;
}
