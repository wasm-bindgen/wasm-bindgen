#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Credential",
        typescript_type = "Credential"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Credential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub type Credential;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn id(this: &Credential) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "id")]
    #[doc = "Like `id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn id_js_string(this: &Credential) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn type_(this: &Credential) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "type")]
    #[doc = "Like `type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn type_js_string(this: &Credential) -> ::js_sys::JsString;
}
