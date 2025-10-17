#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSwitchElement , typescript_type = "SVGSwitchElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgSwitchElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSwitchElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSwitchElement`*"]
    pub type SvgSwitchElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgGraphicsElement> for SvgSwitchElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgSwitchElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgSwitchElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgSwitchElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgSwitchElement {}
