#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SpeechSynthesisVoice",
        typescript_type = "SpeechSynthesisVoice"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisVoice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub type SpeechSynthesisVoice;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisVoice",
        js_name = "voiceURI"
    )]
    #[doc = "Getter for the `voiceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn voice_uri(this: &SpeechSynthesisVoice) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisVoice",
        js_name = "voiceURI"
    )]
    #[doc = "Like `voice_uri()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn voice_uri_js_string(this: &SpeechSynthesisVoice) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisVoice", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn name(this: &SpeechSynthesisVoice) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisVoice", js_name = "name")]
    #[doc = "Like `name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn name_js_string(this: &SpeechSynthesisVoice) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisVoice", js_name = "lang")]
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn lang(this: &SpeechSynthesisVoice) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisVoice", js_name = "lang")]
    #[doc = "Like `lang()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn lang_js_string(this: &SpeechSynthesisVoice) -> ::js_sys::JsString;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisVoice",
        js_name = "localService"
    )]
    #[doc = "Getter for the `localService` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn local_service(this: &SpeechSynthesisVoice) -> bool;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisVoice", js_name = "default")]
    #[doc = "Getter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn default(this: &SpeechSynthesisVoice) -> bool;
}
