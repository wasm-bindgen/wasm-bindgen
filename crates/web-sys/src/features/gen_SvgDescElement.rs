#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGDescElement , typescript_type = "SVGDescElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgDescElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGDescElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgDescElement`*"]
    pub type SvgDescElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgDescElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgDescElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgDescElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgDescElement {}
