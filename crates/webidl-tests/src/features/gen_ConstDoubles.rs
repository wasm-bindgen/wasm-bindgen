#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstDoubles , typescript_type = "ConstDoubles")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstDoubles` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstDoubles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub type ConstDoubles;
}
impl ConstDoubles {
    #[doc = "The `ConstDoubles.d` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub const D: f64 = 0f64 as f64;
    #[doc = "The `ConstDoubles.neg_inf` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub const NEG_INF: f64 = -1.0 / 0.0 as f64;
    #[doc = "The `ConstDoubles.inf` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub const INF: f64 = 1.0 / 0.0 as f64;
    #[doc = "The `ConstDoubles.nan` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub const NAN: f64 = 0.0 / 0.0 as f64;
    #[doc = "The `ConstDoubles.one` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstDoubles`*"]
    pub const ONE: f64 = 1f64 as f64;
}
