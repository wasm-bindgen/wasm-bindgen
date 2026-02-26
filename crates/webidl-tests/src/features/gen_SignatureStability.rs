#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SignatureStability , typescript_type = "SignatureStability")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SignatureStability` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SignatureStability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStability`*"]
    pub type SignatureStability;
    #[wasm_bindgen(catch, constructor, js_class = "SignatureStability")]
    #[doc = "The `new SignatureStability(..)` constructor, creating a new instance of `SignatureStability`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SignatureStability/SignatureStability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStability`*"]
    pub fn new() -> Result<SignatureStability, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    # [wasm_bindgen (method , structural , js_class = "SignatureStability" , js_name = process)]
    #[doc = "The `process()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SignatureStability/process)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStability`*"]
    pub fn process(this: &SignatureStability) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SignatureStabilityOptions")]
    # [wasm_bindgen (method , structural , js_class = "SignatureStability" , js_name = process)]
    #[doc = "The `process()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SignatureStability/process)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignatureStability`, `SignatureStabilityOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn process(
        this: &SignatureStability,
        options: &SignatureStabilityOptions,
    ) -> ::alloc::string::String;
}
