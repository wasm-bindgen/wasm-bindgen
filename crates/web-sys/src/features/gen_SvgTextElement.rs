#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgTextPositioningElement , extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextElement , typescript_type = "SVGTextElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgTextElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextElement`*"]
    pub type SvgTextElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgTextPositioningElement> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgTextContentElement> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgGraphicsElement> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgTextElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgTextElement {}
