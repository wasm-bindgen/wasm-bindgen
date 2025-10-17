#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMPathElement , typescript_type = "SVGMPathElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `SvgmPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgmPathElement`*"]
    pub type SvgmPathElement;
    #[cfg(feature = "SvgAnimatedString")]
    # [wasm_bindgen (structural , method , getter , js_class = "SVGMPathElement" , js_name = href)]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgmPathElement`*"]
    pub fn href(this: &SvgmPathElement) -> SvgAnimatedString;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<SvgElement> for SvgmPathElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for SvgmPathElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for SvgmPathElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for SvgmPathElement {}
