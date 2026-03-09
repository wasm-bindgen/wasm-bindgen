#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Position , typescript_type = "Position")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Position` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Position`*"]
    pub type Position;
}
