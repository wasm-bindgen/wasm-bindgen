mod gui;
mod oscillator;
mod wasm_audio;

use gui::create_gui;
use oscillator::{Oscillator, Params};
use wasm_audio::wasm_audio;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn web_main() {
    console_error_panic_hook::set_once();
    // On the application level, audio worklet internals are abstracted by wasm_audio:
    let params: &'static Params = Box::leak(Box::default());
    let mut osc = Oscillator::new(params);
    let ctx = wasm_audio(Box::new(move |buf| osc.process(buf)))
        .await
        .unwrap();
    create_gui(params, ctx);
}
