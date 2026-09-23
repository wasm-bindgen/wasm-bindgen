#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_polygon_offset_clamp" , typescript_type = "EXT_polygon_offset_clamp")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtPolygonOffsetClamp` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_polygon_offset_clamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtPolygonOffsetClamp`*"]
    pub type ExtPolygonOffsetClamp;
    #[wasm_bindgen(
        method,
        js_class = "EXT_polygon_offset_clamp",
        js_name = "polygonOffsetClampEXT"
    )]
    #[doc = "The `polygonOffsetClampEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_polygon_offset_clamp/polygonOffsetClampEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtPolygonOffsetClamp`*"]
    pub fn polygon_offset_clamp_ext(
        this: &ExtPolygonOffsetClamp,
        factor: f32,
        units: f32,
        clamp: f32,
    );
}
impl ExtPolygonOffsetClamp {
    #[doc = "The `EXT_polygon_offset_clamp.POLYGON_OFFSET_CLAMP_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtPolygonOffsetClamp`*"]
    pub const POLYGON_OFFSET_CLAMP_EXT: u32 = 36379u64 as u32;
}
