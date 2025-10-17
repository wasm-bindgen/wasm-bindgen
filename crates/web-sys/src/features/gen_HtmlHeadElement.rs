#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLHeadElement , typescript_type = "HTMLHeadElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `HtmlHeadElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHeadElement`*"]
    pub type HtmlHeadElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<HtmlElement> for HtmlHeadElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for HtmlHeadElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for HtmlHeadElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for HtmlHeadElement {}
