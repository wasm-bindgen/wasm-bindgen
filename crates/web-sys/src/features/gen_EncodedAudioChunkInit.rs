#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EncodedAudioChunkInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedAudioChunkInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedAudioChunkInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &EncodedAudioChunkInit) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &EncodedAudioChunkInit, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &EncodedAudioChunkInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &EncodedAudioChunkInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &EncodedAudioChunkInit) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &EncodedAudioChunkInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`, `EncodedAudioChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &EncodedAudioChunkInit) -> EncodedAudioChunkType;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`, `EncodedAudioChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &EncodedAudioChunkInit, val: EncodedAudioChunkType);
}
#[cfg(web_sys_unstable_apis)]
impl EncodedAudioChunkInit {
    #[cfg(feature = "EncodedAudioChunkType")]
    #[doc = "Construct a new `EncodedAudioChunkInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkInit`, `EncodedAudioChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(data: &::js_sys::Object, timestamp: f64, type_: EncodedAudioChunkType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_data(data);
        ret.set_timestamp(timestamp);
        ret.set_type(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_data(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: EncodedAudioChunkType) -> &mut Self {
        self.set_type(val);
        self
    }
}
