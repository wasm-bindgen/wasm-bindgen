#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLParagraphElement , typescript_type = "HTMLParagraphElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `HtmlParagraphElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub type HtmlParagraphElement;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLParagraphElement" , js_name = align)]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub fn align(this: &HtmlParagraphElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLParagraphElement" , js_name = align)]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub fn set_align(this: &HtmlParagraphElement, value: &str);
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<HtmlElement> for HtmlParagraphElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for HtmlParagraphElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for HtmlParagraphElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for HtmlParagraphElement {}
