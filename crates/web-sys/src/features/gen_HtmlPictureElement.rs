#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLPictureElement , typescript_type = "HTMLPictureElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `HtmlPictureElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPictureElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlPictureElement`*"]
    pub type HtmlPictureElement;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<HtmlElement> for HtmlPictureElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for HtmlPictureElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for HtmlPictureElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for HtmlPictureElement {}
