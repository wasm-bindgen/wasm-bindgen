#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageDataLike , typescript_type = "ImageDataLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDataLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageDataLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDataLike`*"]
    pub type ImageDataLike;
    #[wasm_bindgen(catch, constructor, js_class = "ImageDataLike")]
    #[doc = "The `new ImageDataLike(..)` constructor, creating a new instance of `ImageDataLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageDataLike/ImageDataLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDataLike`*"]
    pub fn new() -> Result<ImageDataLike, JsValue>;
}
