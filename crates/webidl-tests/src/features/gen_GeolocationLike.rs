#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GeolocationLike , typescript_type = "GeolocationLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationLike`*"]
    pub type GeolocationLike;
    #[wasm_bindgen(catch, constructor, js_class = "GeolocationLike")]
    #[doc = "The `new GeolocationLike(..)` constructor, creating a new instance of `GeolocationLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationLike/GeolocationLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationLike`*"]
    pub fn new() -> Result<GeolocationLike, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    # [wasm_bindgen (catch , method , structural , js_class = "GeolocationLike" , js_name = getCurrentPosition)]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationLike/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationLike`*"]
    pub fn get_current_position(
        this: &GeolocationLike,
        success_callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Position")]
    # [wasm_bindgen (method , structural , js_class = "GeolocationLike" , js_name = getCurrentPosition)]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationLike/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationLike`, `Position`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_position(
        this: &GeolocationLike,
        success_callback: &::js_sys::Function<fn(Position) -> ::js_sys::Undefined>,
    );
}
