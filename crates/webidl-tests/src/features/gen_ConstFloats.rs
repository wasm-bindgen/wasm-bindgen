#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstFloats , typescript_type = "ConstFloats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstFloats` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstFloats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstFloats`*"]
    pub type ConstFloats;
}
impl ConstFloats {
    #[doc = "The `ConstFloats.f` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstFloats`*"]
    pub const F: f32 = 0f64 as f32;
    #[doc = "The `ConstFloats.neg_inf` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstFloats`*"]
    pub const NEG_INF: f32 = -1.0 / 0.0 as f32;
    #[doc = "The `ConstFloats.inf` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstFloats`*"]
    pub const INF: f32 = 1.0 / 0.0 as f32;
    #[doc = "The `ConstFloats.nan` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstFloats`*"]
    pub const NAN: f32 = 0.0 / 0.0 as f32;
}
