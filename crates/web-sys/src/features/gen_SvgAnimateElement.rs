#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgAnimationElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGAnimateElement , typescript_type = "SVGAnimateElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgAnimateElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimateElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimateElement`*"]
    pub type SvgAnimateElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgAnimationElement> for SvgAnimateElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgAnimateElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgAnimateElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgAnimateElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgAnimateElement {}
