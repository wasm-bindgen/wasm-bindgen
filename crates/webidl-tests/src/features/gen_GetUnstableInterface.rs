#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GetUnstableInterface , typescript_type = "GetUnstableInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetUnstableInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUnstableInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUnstableInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GetUnstableInterface;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableInterface")]
    # [wasm_bindgen (static_method_of = GetUnstableInterface , js_class = "GetUnstableInterface" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUnstableInterface/get_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUnstableInterface`, `UnstableInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get() -> UnstableInterface;
}
