#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Worklet , extends = :: js_sys :: Object , js_name = AudioWorklet , typescript_type = "AudioWorklet")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `AudioWorklet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorklet`*"]
    pub type AudioWorklet;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<Worklet> for AudioWorklet {}
