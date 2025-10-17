use core::panic::AssertUnwindSafe;
use core::pin::Pin;
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

// This inline JS creates a blob URL for the AudioWorklet processor.
// It computes the main wasm-bindgen module URL by resolving relative to
// this inline module's URL (going up from snippets/.../inline0.js).
// This is necessary because AudioWorklet modules loaded via blob URLs
// cannot use relative imports - they need absolute URLs.
#[wasm_bindgen(inline_js = "
export function createWorkletModuleUrl() {
    // This inline module is at: snippets/<crate>-<hash>/inline0.js
    // Main module is at: wasm_audio_worklet.js (2 levels up)
    const bindgenUrl = new URL('../../wasm_audio_worklet.js', import.meta.url).href;
    return URL.createObjectURL(new Blob([`
        import * as bindgen from '${bindgenUrl}';

        registerProcessor('WasmProcessor', class WasmProcessor extends AudioWorkletProcessor {
            constructor(options) {
                super();
                let [module, memory, handle] = options.processorOptions;
                bindgen.initSync({ module, memory });
                this.processor = bindgen.WasmAudioProcessor.unpack(handle);
            }
            process(inputs, outputs) {
                return this.processor.process(outputs[0][0]);
            }
        });
    `], { type: 'text/javascript' }));
}
")]
extern "C" {
    fn createWorkletModuleUrl() -> String;
}

// Use wasm_audio if you have a single Wasm audio processor in your application
// whose samples should be played directly. Ideally, call wasm_audio based on
// user interaction. Otherwise, resume the context on user interaction, so
// playback starts reliably on all browsers.
pub fn wasm_audio(
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> AssertUnwindSafe<Pin<Box<dyn std::future::Future<Output = Result<AudioContext, JsValue>>>>> {
    let process = AssertUnwindSafe(process);
    AssertUnwindSafe(Box::pin(async {
        let ctx = AudioContext::new()?;
        prepare_wasm_audio(&ctx).await?;
        let node = wasm_audio_node(&ctx, process.0)?;
        node.connect_with_audio_node(&ctx.destination())?;
        Ok(ctx)
    }))
}

// wasm_audio_node creates an AudioWorkletNode running a Wasm audio processor.
// Remember to call prepare_wasm_audio once on your context before calling
// this function.
pub fn wasm_audio_node(
    ctx: &AudioContext,
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> Result<AudioWorkletNode, JsValue> {
    let options = AudioWorkletNodeOptions::new();
    options.set_processor_options(Some(&js_sys::Array::of(&[
        wasm_bindgen::module(),
        wasm_bindgen::memory(),
        WasmAudioProcessor(process).pack().into(),
    ])));
    AudioWorkletNode::new_with_options(ctx, "WasmProcessor", &options)
}

pub async fn prepare_wasm_audio(ctx: &AudioContext) -> Result<(), JsValue> {
    let mod_url = createWorkletModuleUrl();
    JsFuture::from(ctx.audio_worklet()?.add_module(&mod_url)?).await?;
    Ok(())
}
