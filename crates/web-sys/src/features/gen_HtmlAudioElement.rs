#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlMediaElement , extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAudioElement , typescript_type = "HTMLAudioElement")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `HtmlAudioElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub type HtmlAudioElement;
    #[wasm_bindgen(catch, constructor, js_class = "Audio")]
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub fn new() -> Result<HtmlAudioElement, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Audio")]
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub fn new_with_src(src: &str) -> Result<HtmlAudioElement, JsValue>;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<HtmlMediaElement> for HtmlAudioElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<HtmlElement> for HtmlAudioElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Element> for HtmlAudioElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Node> for HtmlAudioElement {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for HtmlAudioElement {}
