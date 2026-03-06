#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SignatureStabilityOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SignatureStabilityOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStabilityOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SignatureStabilityOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SignatureStabilityMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStabilityMode`, `SignatureStabilityOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &SignatureStabilityOptions) -> Option<SignatureStabilityMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SignatureStabilityMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStabilityMode`, `SignatureStabilityOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &SignatureStabilityOptions, val: SignatureStabilityMode);
}
#[cfg(web_sys_unstable_apis)]
impl SignatureStabilityOptions {
    #[doc = "Construct a new `SignatureStabilityOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStabilityOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SignatureStabilityMode")]
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: SignatureStabilityMode) -> &mut Self {
        self.set_mode(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SignatureStabilityOptions {
    fn default() -> Self {
        Self::new()
    }
}
