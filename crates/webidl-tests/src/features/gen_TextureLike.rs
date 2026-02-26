#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TextureLike , typescript_type = "TextureLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextureLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextureLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`*"]
    pub type TextureLike;
    #[wasm_bindgen(catch, constructor, js_class = "TextureLike")]
    #[doc = "The `new TextureLike(..)` constructor, creating a new instance of `TextureLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextureLike/TextureLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`*"]
    pub fn new() -> Result<TextureLike, JsValue>;
}
