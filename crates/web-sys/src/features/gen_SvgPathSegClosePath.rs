#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegClosePath , typescript_type = "SVGPathSegClosePath")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgPathSegClosePath` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegClosePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegClosePath`*"]
    pub type SvgPathSegClosePath;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgPathSeg> for SvgPathSegClosePath {}
