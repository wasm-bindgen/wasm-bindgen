#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "Animation",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CSSAnimation",
        typescript_type = "CSSAnimation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssAnimation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssAnimation`*"]
    pub type CssAnimation;
    #[wasm_bindgen(method, getter, js_class = "CSSAnimation", js_name = "animationName")]
    #[doc = "Getter for the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssAnimation`*"]
    pub fn animation_name(this: &CssAnimation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "CSSAnimation", js_name = "animationName")]
    #[doc = "Like `animation_name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssAnimation`*"]
    pub fn animation_name_js_string(this: &CssAnimation) -> ::js_sys::JsString;
}
