#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PartialInterface , typescript_type = "PartialInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PartialInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub type PartialInterface;
    # [wasm_bindgen (structural , method , getter , js_class = "PartialInterface" , js_name = un)]
    #[doc = "Getter for the `un` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface/un)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub fn un(this: &PartialInterface) -> i16;
    # [wasm_bindgen (structural , method , getter , js_class = "PartialInterface" , js_name = trois)]
    #[doc = "Getter for the `trois` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface/trois)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub fn trois(this: &PartialInterface) -> i16;
    #[wasm_bindgen(catch, constructor, js_class = "PartialInterface")]
    #[doc = "The `new PartialInterface(..)` constructor, creating a new instance of `PartialInterface`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface/PartialInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub fn new() -> Result<PartialInterface, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "PartialInterface" , js_name = deux)]
    #[doc = "The `deux()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface/deux)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub fn deux(this: &PartialInterface) -> i16;
    # [wasm_bindgen (method , structural , js_class = "PartialInterface" , js_name = quatre)]
    #[doc = "The `quatre()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PartialInterface/quatre)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PartialInterface`*"]
    pub fn quatre(this: &PartialInterface) -> i16;
}
