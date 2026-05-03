#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StartViewTransitionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StartViewTransitionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type StartViewTransitionOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "types")]
    pub fn get_types(
        this: &StartViewTransitionOptions,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "types")]
    pub fn set_types(this: &StartViewTransitionOptions, val: Option<&[::js_sys::JsString]>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `update` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "update")]
    pub fn get_update(this: &StartViewTransitionOptions) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `update` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "update")]
    pub fn set_update(this: &StartViewTransitionOptions, val: Option<&::js_sys::Function>);
}
#[cfg(web_sys_unstable_apis)]
impl StartViewTransitionOptions {
    #[doc = "Construct a new `StartViewTransitionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StartViewTransitionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_types()` instead."]
    pub fn types(&mut self, val: Option<&[::js_sys::JsString]>) -> &mut Self {
        self.set_types(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_update()` instead."]
    pub fn update(&mut self, val: Option<&::js_sys::Function>) -> &mut Self {
        self.set_update(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for StartViewTransitionOptions {
    fn default() -> Self {
        Self::new()
    }
}
