#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstBool , typescript_type = "ConstBool")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstBool` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstBool)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstBool`*"]
    pub type ConstBool;
}
impl ConstBool {
    #[doc = "The `ConstBool.not_true` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstBool`*"]
    pub const NOT_TRUE: bool = false as bool;
    #[doc = "The `ConstBool.not_false` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstBool`*"]
    pub const NOT_FALSE: bool = true as bool;
}
