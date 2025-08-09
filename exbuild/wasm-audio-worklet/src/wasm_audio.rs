use crate::dependent_module;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, AudioWorkletNode, AudioWorkletNodeOptions};

#[wasm_bindgen]
pub struct WasmAudioProcessor(Box<dyn FnMut(&mut [f32]) -> bool>);

#[wasm_bindgen]
impl WasmAudioProcessor {
    pub fn process(&mut self, buf: &mut [f32]) -> bool {
        self.0(buf)
    }
    pub fn pack(self) -> usize {
        Box::into_raw(Box::new(self)) as usize
    }
    pub unsafe fn unpack(val: usize) -> Self {
        *Box::from_raw(val as *mut _)
    }
}

// Use wasm_audio if you have a single Wasm audio processor in your application
// whose samples should be played directly. Ideally, call wasm_audio based on
// user interaction. Otherwise, resume the context on user interaction, so
// playback starts reliably on all browsers.
pub async fn wasm_audio(
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> Result<AudioContext, JsValue> {
    let ctx = AudioContext::new()?;
    prepare_wasm_audio(&ctx).await?;
    let node = wasm_audio_node(&ctx, process)?;
    node.connect_with_audio_node(&ctx.destination())?;
    Ok(ctx)
}

// wasm_audio_node creates an AudioWorkletNode running a Wasm audio processor.
// Remember to call prepare_wasm_audio once on your context before calling
// this function.
pub fn wasm_audio_node(
    ctx: &AudioContext,
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> Result<AudioWorkletNode, JsValue> {
    let options = AudioWorkletNodeOptions::new();
    options.set_processor_options(Some(&js_sys::Array::of3(
        &wasm_bindgen::module(),
        &wasm_bindgen::memory(),
        &WasmAudioProcessor(process).pack().into(),
    )));
    AudioWorkletNode::new_with_options(ctx, "WasmProcessor", &options)
}

pub async fn prepare_wasm_audio(ctx: &AudioContext) -> Result<(), JsValue> {
    let mod_url = dependent_module!("worklet.js")?;
    JsFuture::from(ctx.audio_worklet()?.add_module(&mod_url)?).await?;
    Ok(())
}
