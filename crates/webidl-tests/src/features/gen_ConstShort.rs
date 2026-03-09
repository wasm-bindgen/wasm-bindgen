#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstShort , typescript_type = "ConstShort")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstShort` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstShort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstShort`*"]
    pub type ConstShort;
}
impl ConstShort {
    #[doc = "The `ConstShort.imin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstShort`*"]
    pub const IMIN: i16 = -32768i64 as i16;
    #[doc = "The `ConstShort.imax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstShort`*"]
    pub const IMAX: i16 = 32767u64 as i16;
    #[doc = "The `ConstShort.umin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstShort`*"]
    pub const UMIN: u16 = 0i64 as u16;
    #[doc = "The `ConstShort.umax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstShort`*"]
    pub const UMAX: u16 = 65535u64 as u16;
}
