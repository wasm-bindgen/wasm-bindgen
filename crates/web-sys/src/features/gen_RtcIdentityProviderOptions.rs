#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityProviderOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub type RtcIdentityProviderOptions;
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "peerIdentity")]
    pub fn get_peer_identity(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String>;
    #[doc = "Like `get_peer_identity()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "peerIdentity")]
    pub fn get_peer_identity_js_string(
        this: &RtcIdentityProviderOptions,
    ) -> Option<::js_sys::JsString>;
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, setter = "peerIdentity")]
    pub fn set_peer_identity(this: &RtcIdentityProviderOptions, val: &str);
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String>;
    #[doc = "Like `get_protocol()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol_js_string(this: &RtcIdentityProviderOptions) -> Option<::js_sys::JsString>;
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, setter = "protocol")]
    pub fn set_protocol(this: &RtcIdentityProviderOptions, val: &str);
    #[doc = "Get the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "usernameHint")]
    pub fn get_username_hint(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String>;
    #[doc = "Like `get_username_hint()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, getter = "usernameHint")]
    pub fn get_username_hint_js_string(
        this: &RtcIdentityProviderOptions,
    ) -> Option<::js_sys::JsString>;
    #[doc = "Change the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    #[wasm_bindgen(method, setter = "usernameHint")]
    pub fn set_username_hint(this: &RtcIdentityProviderOptions, val: &str);
}
impl RtcIdentityProviderOptions {
    #[doc = "Construct a new `RtcIdentityProviderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
    pub fn peer_identity(&mut self, val: &str) -> &mut Self {
        self.set_peer_identity(val);
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol(val);
        self
    }
    #[deprecated = "Use `set_username_hint()` instead."]
    pub fn username_hint(&mut self, val: &str) -> &mut Self {
        self.set_username_hint(val);
        self
    }
}
impl Default for RtcIdentityProviderOptions {
    fn default() -> Self {
        Self::new()
    }
}
