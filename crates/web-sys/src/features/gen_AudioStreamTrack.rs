#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = MediaStreamTrack , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioStreamTrack , typescript_type = "AudioStreamTrack")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `AudioStreamTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioStreamTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioStreamTrack`*"]
    pub type AudioStreamTrack;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<MediaStreamTrack> for AudioStreamTrack {}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<EventTarget> for AudioStreamTrack {}
