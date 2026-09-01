#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "MediaStreamError" , typescript_type = "MediaStreamError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub type MediaStreamError;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn name(this: &MediaStreamError) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "name")]
    #[doc = "Like `name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn name_js_string(this: &MediaStreamError) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn message(this: &MediaStreamError) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "message")]
    #[doc = "Like `message()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn message_js_string(this: &MediaStreamError) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "constraint")]
    #[doc = "Getter for the `constraint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/constraint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn constraint(this: &MediaStreamError) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamError", js_name = "constraint")]
    #[doc = "Like `constraint()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/constraint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn constraint_js_string(this: &MediaStreamError) -> Option<::js_sys::JsString>;
}
