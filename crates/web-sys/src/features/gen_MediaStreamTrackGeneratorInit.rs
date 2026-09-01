#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaStreamTrackGeneratorInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackGeneratorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGeneratorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaStreamTrackGeneratorInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `kind` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGeneratorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "kind")]
    pub fn get_kind(this: &MediaStreamTrackGeneratorInit) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `kind` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGeneratorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "kind")]
    pub fn set_kind<S0: ::wasm_bindgen::JsStringLike>(
        this: &MediaStreamTrackGeneratorInit,
        val: S0,
    );
}
#[cfg(web_sys_unstable_apis)]
impl MediaStreamTrackGeneratorInit {
    #[doc = "Construct a new `MediaStreamTrackGeneratorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGeneratorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new<S0: ::wasm_bindgen::JsStringLike>(kind: S0) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_kind(kind);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_kind()` instead."]
    pub fn kind<S0: ::wasm_bindgen::JsStringLike>(&mut self, val: S0) -> &mut Self {
        self.set_kind(val);
        self
    }
}
