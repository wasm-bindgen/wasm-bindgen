#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PublicKeyCredentialRpEntity")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRpEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub type PublicKeyCredentialRpEntity;
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[deprecated(note = "Removed from the WebAuthn specification")]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &PublicKeyCredentialRpEntity) -> Option<::alloc::string::String>;
    #[doc = "Like `get_icon()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[deprecated(note = "Removed from the WebAuthn specification")]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon_js_string(this: &PublicKeyCredentialRpEntity) -> Option<::js_sys::JsString>;
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[deprecated(note = "Removed from the WebAuthn specification")]
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &PublicKeyCredentialRpEntity, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PublicKeyCredentialRpEntity) -> ::alloc::string::String;
    #[doc = "Like `get_name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name_js_string(this: &PublicKeyCredentialRpEntity) -> ::js_sys::JsString;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &PublicKeyCredentialRpEntity, val: &str);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialRpEntity) -> Option<::alloc::string::String>;
    #[doc = "Like `get_id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id_js_string(this: &PublicKeyCredentialRpEntity) -> Option<::js_sys::JsString>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &PublicKeyCredentialRpEntity, val: &str);
}
impl PublicKeyCredentialRpEntity {
    #[doc = "Construct a new `PublicKeyCredentialRpEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated(note = "Removed from the WebAuthn specification")]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
}
