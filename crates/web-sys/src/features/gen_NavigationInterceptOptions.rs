#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigationInterceptOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigationInterceptOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`*"]
    pub type NavigationInterceptOptions;
    #[cfg(feature = "NavigationFocusReset")]
    #[doc = "Get the `focusReset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationFocusReset`, `NavigationInterceptOptions`*"]
    #[wasm_bindgen(method, getter = "focusReset")]
    pub fn get_focus_reset(this: &NavigationInterceptOptions) -> Option<NavigationFocusReset>;
    #[cfg(feature = "NavigationFocusReset")]
    #[doc = "Change the `focusReset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationFocusReset`, `NavigationInterceptOptions`*"]
    #[wasm_bindgen(method, setter = "focusReset")]
    pub fn set_focus_reset(this: &NavigationInterceptOptions, val: NavigationFocusReset);
    #[doc = "Get the `handler` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`*"]
    #[wasm_bindgen(method, getter = "handler")]
    pub fn get_handler(this: &NavigationInterceptOptions) -> Option<::js_sys::Function>;
    #[doc = "Change the `handler` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`*"]
    #[wasm_bindgen(method, setter = "handler")]
    pub fn set_handler(this: &NavigationInterceptOptions, val: &::js_sys::Function);
    #[cfg(feature = "NavigationScrollBehavior")]
    #[doc = "Get the `scroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`, `NavigationScrollBehavior`*"]
    #[wasm_bindgen(method, getter = "scroll")]
    pub fn get_scroll(this: &NavigationInterceptOptions) -> Option<NavigationScrollBehavior>;
    #[cfg(feature = "NavigationScrollBehavior")]
    #[doc = "Change the `scroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`, `NavigationScrollBehavior`*"]
    #[wasm_bindgen(method, setter = "scroll")]
    pub fn set_scroll(this: &NavigationInterceptOptions, val: NavigationScrollBehavior);
}
impl NavigationInterceptOptions {
    #[doc = "Construct a new `NavigationInterceptOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationInterceptOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "NavigationFocusReset")]
    #[deprecated = "Use `set_focus_reset()` instead."]
    pub fn focus_reset(&mut self, val: NavigationFocusReset) -> &mut Self {
        self.set_focus_reset(val);
        self
    }
    #[deprecated = "Use `set_handler()` instead."]
    pub fn handler(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_handler(val);
        self
    }
    #[cfg(feature = "NavigationScrollBehavior")]
    #[deprecated = "Use `set_scroll()` instead."]
    pub fn scroll(&mut self, val: NavigationScrollBehavior) -> &mut Self {
        self.set_scroll(val);
        self
    }
}
impl Default for NavigationInterceptOptions {
    fn default() -> Self {
        Self::new()
    }
}
