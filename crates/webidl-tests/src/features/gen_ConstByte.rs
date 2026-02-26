#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstByte , typescript_type = "ConstByte")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstByte` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstByte)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstByte`*"]
    pub type ConstByte;
}
impl ConstByte {
    #[doc = "The `ConstByte.imin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstByte`*"]
    pub const IMIN: i8 = -128i64 as i8;
    #[doc = "The `ConstByte.imax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstByte`*"]
    pub const IMAX: i8 = 127u64 as i8;
    #[doc = "The `ConstByte.umin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstByte`*"]
    pub const UMIN: u8 = 0i64 as u8;
    #[doc = "The `ConstByte.umax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstByte`*"]
    pub const UMAX: u8 = 255u64 as u8;
}
