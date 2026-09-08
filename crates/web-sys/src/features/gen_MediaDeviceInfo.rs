#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaDeviceInfo",
        typescript_type = "MediaDeviceInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDeviceInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub type MediaDeviceInfo;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "deviceId")]
    #[doc = "Getter for the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn device_id(this: &MediaDeviceInfo) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "deviceId")]
    #[doc = "Like `device_id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn device_id_js_string(this: &MediaDeviceInfo) -> ::js_sys::JsString;
    #[cfg(feature = "MediaDeviceKind")]
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`, `MediaDeviceKind`*"]
    pub fn kind(this: &MediaDeviceInfo) -> MediaDeviceKind;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn label(this: &MediaDeviceInfo) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "label")]
    #[doc = "Like `label()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn label_js_string(this: &MediaDeviceInfo) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "groupId")]
    #[doc = "Getter for the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn group_id(this: &MediaDeviceInfo) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaDeviceInfo", js_name = "groupId")]
    #[doc = "Like `group_id()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn group_id_js_string(this: &MediaDeviceInfo) -> ::js_sys::JsString;
    #[wasm_bindgen(method, js_class = "MediaDeviceInfo", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn to_json(this: &MediaDeviceInfo) -> ::js_sys::Object;
}
