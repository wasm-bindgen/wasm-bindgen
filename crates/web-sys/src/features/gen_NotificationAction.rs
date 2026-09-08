#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NotificationAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationAction` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub type NotificationAction;
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &NotificationAction) -> ::alloc::string::String;
    #[doc = "Like `get_action()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action_js_string(this: &NotificationAction) -> ::js_sys::JsString;
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &NotificationAction, val: &str);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &NotificationAction) -> Option<::alloc::string::String>;
    #[doc = "Like `get_icon()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon_js_string(this: &NotificationAction) -> Option<::js_sys::JsString>;
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &NotificationAction, val: &str);
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &NotificationAction) -> ::alloc::string::String;
    #[doc = "Like `get_title()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title_js_string(this: &NotificationAction) -> ::js_sys::JsString;
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &NotificationAction, val: &str);
}
impl NotificationAction {
    #[doc = "Construct a new `NotificationAction`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn new(action: &str, title: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_action(action);
        ret.set_title(title);
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: &str) -> &mut Self {
        self.set_action(val);
        self
    }
    #[deprecated = "Use `set_icon()` instead."]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title(val);
        self
    }
}
