#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebGLLike , typescript_type = "WebGLLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlLike`*"]
    pub type WebGlLike;
    #[wasm_bindgen(catch, constructor, js_class = "WebGLLike")]
    #[doc = "The `new WebGlLike(..)` constructor, creating a new instance of `WebGlLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike/WebGLLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlLike`*"]
    pub fn new() -> Result<WebGlLike, JsValue>;
    #[cfg(feature = "TextureLike")]
    # [wasm_bindgen (catch , method , structural , js_class = "WebGLLike" , js_name = texUpload)]
    #[doc = "The `texUpload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike/texUpload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`, `WebGlLike`*"]
    pub fn tex_upload_with_x_and_y(
        this: &WebGlLike,
        texture: &TextureLike,
        x: i32,
        y: i32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextureLike")]
    # [wasm_bindgen (catch , method , structural , js_class = "WebGLLike" , js_name = texUpload)]
    #[doc = "The `texUpload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike/texUpload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`, `WebGlLike`*"]
    pub fn tex_upload_with_dx_and_dy(
        this: &WebGlLike,
        texture: &TextureLike,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextureLike")]
    # [wasm_bindgen (catch , method , structural , js_class = "WebGLLike" , js_name = texUpload)]
    #[doc = "The `texUpload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike/texUpload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`, `WebGlLike`*"]
    pub fn tex_upload_with_data(
        this: &WebGlLike,
        texture: &TextureLike,
        data: &str,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "TextureLike", feature = "UnstableFrame",))]
    # [wasm_bindgen (catch , method , structural , js_class = "WebGLLike" , js_name = texUpload)]
    #[doc = "The `texUpload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLLike/texUpload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextureLike`, `UnstableFrame`, `WebGlLike`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn tex_upload_with_frame(
        this: &WebGlLike,
        texture: &TextureLike,
        frame: &UnstableFrame,
    ) -> Result<(), JsValue>;
}
