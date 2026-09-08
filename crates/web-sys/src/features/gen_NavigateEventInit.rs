#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NavigateEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigateEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    pub type NavigateEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &NavigateEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &NavigateEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &NavigateEventInit, val: bool);
    #[doc = "Get the `canIntercept` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "canIntercept")]
    pub fn get_can_intercept(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `canIntercept` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "canIntercept")]
    pub fn set_can_intercept(this: &NavigateEventInit, val: bool);
    #[cfg(feature = "NavigationDestination")]
    #[doc = "Get the `destination` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`, `NavigationDestination`*"]
    #[wasm_bindgen(method, getter = "destination")]
    pub fn get_destination(this: &NavigateEventInit) -> NavigationDestination;
    #[cfg(feature = "NavigationDestination")]
    #[doc = "Change the `destination` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`, `NavigationDestination`*"]
    #[wasm_bindgen(method, setter = "destination")]
    pub fn set_destination(this: &NavigateEventInit, val: &NavigationDestination);
    #[doc = "Get the `downloadRequest` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "downloadRequest")]
    pub fn get_download_request(this: &NavigateEventInit) -> Option<::alloc::string::String>;
    #[doc = "Like `get_download_request()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "downloadRequest")]
    pub fn get_download_request_js_string(this: &NavigateEventInit) -> Option<::js_sys::JsString>;
    #[doc = "Change the `downloadRequest` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "downloadRequest")]
    pub fn set_download_request(this: &NavigateEventInit, val: Option<&str>);
    #[cfg(feature = "FormData")]
    #[doc = "Get the `formData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "formData")]
    pub fn get_form_data(this: &NavigateEventInit) -> Option<FormData>;
    #[cfg(feature = "FormData")]
    #[doc = "Change the `formData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "formData")]
    pub fn set_form_data(this: &NavigateEventInit, val: Option<&FormData>);
    #[doc = "Get the `hasUAVisualTransition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "hasUAVisualTransition")]
    pub fn get_has_ua_visual_transition(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `hasUAVisualTransition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "hasUAVisualTransition")]
    pub fn set_has_ua_visual_transition(this: &NavigateEventInit, val: bool);
    #[doc = "Get the `hashChange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "hashChange")]
    pub fn get_hash_change(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `hashChange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "hashChange")]
    pub fn set_hash_change(this: &NavigateEventInit, val: bool);
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &NavigateEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &NavigateEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "NavigationApiType")]
    #[doc = "Get the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`, `NavigationApiType`*"]
    #[wasm_bindgen(method, getter = "navigationType")]
    pub fn get_navigation_type(this: &NavigateEventInit) -> Option<NavigationApiType>;
    #[cfg(feature = "NavigationApiType")]
    #[doc = "Change the `navigationType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`, `NavigationApiType`*"]
    #[wasm_bindgen(method, setter = "navigationType")]
    pub fn set_navigation_type(this: &NavigateEventInit, val: NavigationApiType);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "signal")]
    pub fn get_signal(this: &NavigateEventInit) -> AbortSignal;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "signal")]
    pub fn set_signal(this: &NavigateEventInit, val: &AbortSignal);
    #[cfg(feature = "Element")]
    #[doc = "Get the `sourceElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "sourceElement")]
    pub fn get_source_element(this: &NavigateEventInit) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[doc = "Change the `sourceElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "sourceElement")]
    pub fn set_source_element(this: &NavigateEventInit, val: Option<&Element>);
    #[doc = "Get the `userInitiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, getter = "userInitiated")]
    pub fn get_user_initiated(this: &NavigateEventInit) -> Option<bool>;
    #[doc = "Change the `userInitiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigateEventInit`*"]
    #[wasm_bindgen(method, setter = "userInitiated")]
    pub fn set_user_initiated(this: &NavigateEventInit, val: bool);
}
impl NavigateEventInit {
    #[cfg(all(feature = "AbortSignal", feature = "NavigationDestination",))]
    #[doc = "Construct a new `NavigateEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `NavigateEventInit`, `NavigationDestination`*"]
    pub fn new(destination: &NavigationDestination, signal: &AbortSignal) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_destination(destination);
        ret.set_signal(signal);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_can_intercept()` instead."]
    pub fn can_intercept(&mut self, val: bool) -> &mut Self {
        self.set_can_intercept(val);
        self
    }
    #[cfg(feature = "NavigationDestination")]
    #[deprecated = "Use `set_destination()` instead."]
    pub fn destination(&mut self, val: &NavigationDestination) -> &mut Self {
        self.set_destination(val);
        self
    }
    #[deprecated = "Use `set_download_request()` instead."]
    pub fn download_request(&mut self, val: Option<&str>) -> &mut Self {
        self.set_download_request(val);
        self
    }
    #[cfg(feature = "FormData")]
    #[deprecated = "Use `set_form_data()` instead."]
    pub fn form_data(&mut self, val: Option<&FormData>) -> &mut Self {
        self.set_form_data(val);
        self
    }
    #[deprecated = "Use `set_has_ua_visual_transition()` instead."]
    pub fn has_ua_visual_transition(&mut self, val: bool) -> &mut Self {
        self.set_has_ua_visual_transition(val);
        self
    }
    #[deprecated = "Use `set_hash_change()` instead."]
    pub fn hash_change(&mut self, val: bool) -> &mut Self {
        self.set_hash_change(val);
        self
    }
    #[deprecated = "Use `set_info()` instead."]
    pub fn info(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_info(val);
        self
    }
    #[cfg(feature = "NavigationApiType")]
    #[deprecated = "Use `set_navigation_type()` instead."]
    pub fn navigation_type(&mut self, val: NavigationApiType) -> &mut Self {
        self.set_navigation_type(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal(val);
        self
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_source_element()` instead."]
    pub fn source_element(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_source_element(val);
        self
    }
    #[deprecated = "Use `set_user_initiated()` instead."]
    pub fn user_initiated(&mut self, val: bool) -> &mut Self {
        self.set_user_initiated(val);
        self
    }
}
