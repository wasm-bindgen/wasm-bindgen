#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "RTCDTMFToneChangeEvent",
        typescript_type = "RTCDTMFToneChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfToneChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub type RtcdtmfToneChangeEvent;
    #[wasm_bindgen(method, getter, js_class = "RTCDTMFToneChangeEvent", js_name = "tone")]
    #[doc = "Getter for the `tone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub fn tone(this: &RtcdtmfToneChangeEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "RTCDTMFToneChangeEvent", js_name = "tone")]
    #[doc = "Like `tone()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub fn tone_js_string(this: &RtcdtmfToneChangeEvent) -> ::js_sys::JsString;
    #[wasm_bindgen(catch, constructor, js_class = "RTCDTMFToneChangeEvent")]
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub fn new(type_: &str) -> Result<RtcdtmfToneChangeEvent, JsValue>;
    #[cfg(feature = "RtcdtmfToneChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCDTMFToneChangeEvent")]
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`, `RtcdtmfToneChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcdtmfToneChangeEventInit,
    ) -> Result<RtcdtmfToneChangeEvent, JsValue>;
}
