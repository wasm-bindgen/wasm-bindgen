#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LockManager , typescript_type = "LockManager")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LockManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LockManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type LockManager;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "LockManager" , js_name = query)]
    #[doc = "The `query()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query(this: &LockManager) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "LockManager" , js_name = request)]
    #[doc = "The `request()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_with_callback(
        this: &LockManager,
        name: &str,
        callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockOptions")]
    # [wasm_bindgen (method , structural , js_class = "LockManager" , js_name = request)]
    #[doc = "The `request()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`, `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_with_options_and_callback(
        this: &LockManager,
        name: &str,
        options: &LockOptions,
        callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
}
