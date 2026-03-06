#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstLongLong , typescript_type = "ConstLongLong")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstLongLong` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstLongLong)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLongLong`*"]
    pub type ConstLongLong;
}
impl ConstLongLong {
    #[doc = "The `ConstLongLong.imin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLongLong`*"]
    pub const IMIN: f64 = -9223372036854775808i64 as f64;
    #[doc = "The `ConstLongLong.imax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLongLong`*"]
    pub const IMAX: f64 = 9223372036854775807u64 as f64;
    #[doc = "The `ConstLongLong.umin` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLongLong`*"]
    pub const UMIN: f64 = 0i64 as f64;
    #[doc = "The `ConstLongLong.umax` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstLongLong`*"]
    pub const UMAX: f64 = 18446744073709551615u64 as f64;
}
