#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CanvasLike , typescript_type = "CanvasLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasLike`*"]
    pub type CanvasLike;
    #[wasm_bindgen(catch, constructor, js_class = "CanvasLike")]
    #[doc = "The `new CanvasLike(..)` constructor, creating a new instance of `CanvasLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasLike/CanvasLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasLike`*"]
    pub fn new() -> Result<CanvasLike, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageDataLike")]
    # [wasm_bindgen (method , structural , js_class = "CanvasLike" , js_name = putImageData)]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasLike/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasLike`, `ImageDataLike`*"]
    pub fn put_image_data(this: &CanvasLike, imagedata: &ImageDataLike, dx: f64, dy: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageDataLike")]
    # [wasm_bindgen (method , structural , js_class = "CanvasLike" , js_name = putImageData)]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasLike/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasLike`, `ImageDataLike`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn put_image_data(this: &CanvasLike, imagedata: &ImageDataLike, dx: i32, dy: i32);
}
