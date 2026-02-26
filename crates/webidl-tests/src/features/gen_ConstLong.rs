#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstLong , typescript_type = "ConstLong")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstLong` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstLong)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLong`*"]
    pub type ConstLong;
}
impl ConstLong {
    #[doc = "The `ConstLong.imin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLong`*"]
    pub const IMIN: i32 = -2147483648i64 as i32;
    #[doc = "The `ConstLong.imax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLong`*"]
    pub const IMAX: i32 = 2147483647u64 as i32;
    #[doc = "The `ConstLong.umin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLong`*"]
    pub const UMIN: u32 = 0i64 as u32;
    #[doc = "The `ConstLong.umax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLong`*"]
    pub const UMAX: u32 = 4294967295u64 as u32;
}
