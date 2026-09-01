#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredentialDescriptorJSON"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialDescriptorJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PublicKeyCredentialDescriptorJson;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialDescriptorJson) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Like `get_id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id_js_string(this: &PublicKeyCredentialDescriptorJson) -> ::js_sys::JsString;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id<S0: ::wasm_bindgen::JsStringLike>(
        this: &PublicKeyCredentialDescriptorJson,
        val: S0,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "transports")]
    pub fn get_transports(
        this: &PublicKeyCredentialDescriptorJson,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "transports")]
    pub fn set_transports(this: &PublicKeyCredentialDescriptorJson, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &PublicKeyCredentialDescriptorJson) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Like `get_type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type_js_string(this: &PublicKeyCredentialDescriptorJson) -> ::js_sys::JsString;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type<S0: ::wasm_bindgen::JsStringLike>(
        this: &PublicKeyCredentialDescriptorJson,
        val: S0,
    );
}
#[cfg(web_sys_unstable_apis)]
impl PublicKeyCredentialDescriptorJson {
    #[doc = "Construct a new `PublicKeyCredentialDescriptorJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new<S0: ::wasm_bindgen::JsStringLike, S1: ::wasm_bindgen::JsStringLike>(
        id: S0,
        type_: S1,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_id(id);
        ret.set_type(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_id()` instead."]
    pub fn id<S0: ::wasm_bindgen::JsStringLike>(&mut self, val: S0) -> &mut Self {
        self.set_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_transports()` instead."]
    pub fn transports(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_transports(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_<S0: ::wasm_bindgen::JsStringLike>(&mut self, val: S0) -> &mut Self {
        self.set_type(val);
        self
    }
}
