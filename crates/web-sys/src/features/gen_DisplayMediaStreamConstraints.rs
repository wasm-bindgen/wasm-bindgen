#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayMediaStreamConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayMediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub type DisplayMediaStreamConstraints;
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[deprecated(note = "Use `set_audio_bool()` or `set_audio_media_track_constraints()` instead.")]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio_bool(this: &DisplayMediaStreamConstraints, val: bool);
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio_media_track_constraints(
        this: &DisplayMediaStreamConstraints,
        val: &MediaTrackConstraints,
    );
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[deprecated(note = "Use `set_video_bool()` or `set_video_media_track_constraints()` instead.")]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video_bool(this: &DisplayMediaStreamConstraints, val: bool);
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video_media_track_constraints(
        this: &DisplayMediaStreamConstraints,
        val: &MediaTrackConstraints,
    );
}
impl DisplayMediaStreamConstraints {
    #[doc = "Construct a new `DisplayMediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "MediaTrackConstraints")]
    #[deprecated = "Use `set_audio()` instead."]
    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_audio(val);
        self
    }
    #[cfg(feature = "MediaTrackConstraints")]
    #[deprecated = "Use `set_video()` instead."]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video(val);
        self
    }
}
impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
