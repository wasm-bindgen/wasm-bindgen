#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TaskSignalAnyInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskSignalAnyInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TaskSignalAnyInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &TaskSignalAnyInit) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &TaskSignalAnyInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl TaskSignalAnyInit {
    #[doc = "Construct a new `TaskSignalAnyInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_priority(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for TaskSignalAnyInit {
    fn default() -> Self {
        Self::new()
    }
}
