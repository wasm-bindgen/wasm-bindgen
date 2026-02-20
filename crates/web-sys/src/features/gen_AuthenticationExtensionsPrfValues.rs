#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsPRFValues)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsPrfValues` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AuthenticationExtensionsPrfValues;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "first")]
    pub fn get_first(this: &AuthenticationExtensionsPrfValues) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[deprecated(
        note = "Use `set_first_buffer_source()` or `set_first_u8_slice()` or `set_first_u8_array()` instead."
    )]
    #[wasm_bindgen(method, setter = "first")]
    pub fn set_first(this: &AuthenticationExtensionsPrfValues, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "first")]
    pub fn set_first_buffer_source(
        this: &AuthenticationExtensionsPrfValues,
        val: &::js_sys::Object,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "first")]
    pub fn set_first_u8_slice(this: &AuthenticationExtensionsPrfValues, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "first")]
    pub fn set_first_u8_array(this: &AuthenticationExtensionsPrfValues, val: &::js_sys::Uint8Array);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "second")]
    pub fn get_second(this: &AuthenticationExtensionsPrfValues) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[deprecated(
        note = "Use `set_second_buffer_source()` or `set_second_u8_slice()` or `set_second_u8_array()` instead."
    )]
    #[wasm_bindgen(method, setter = "second")]
    pub fn set_second(this: &AuthenticationExtensionsPrfValues, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "second")]
    pub fn set_second_buffer_source(
        this: &AuthenticationExtensionsPrfValues,
        val: &::js_sys::Object,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "second")]
    pub fn set_second_u8_slice(this: &AuthenticationExtensionsPrfValues, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "second")]
    pub fn set_second_u8_array(
        this: &AuthenticationExtensionsPrfValues,
        val: &::js_sys::Uint8Array,
    );
}
#[cfg(web_sys_unstable_apis)]
impl AuthenticationExtensionsPrfValues {
    #[doc = "Construct a new `AuthenticationExtensionsPrfValues`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(first: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_first(first);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_first()` instead."]
    pub fn first(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_first(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_second()` instead."]
    pub fn second(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_second(val);
        self
    }
}
