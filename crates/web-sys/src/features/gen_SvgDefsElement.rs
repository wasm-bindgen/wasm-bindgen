#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGDefsElement , typescript_type = "SVGDefsElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgDefsElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGDefsElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgDefsElement`*"]
    pub type SvgDefsElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgGraphicsElement> for SvgDefsElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgDefsElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgDefsElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgDefsElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgDefsElement {}
