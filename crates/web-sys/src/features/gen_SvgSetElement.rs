#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgAnimationElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSetElement , typescript_type = "SVGSetElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgSetElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSetElement`*"]
    pub type SvgSetElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgAnimationElement> for SvgSetElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgSetElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgSetElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgSetElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgSetElement {}
