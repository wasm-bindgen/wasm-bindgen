// Wasm audio processors can be implemented in Rust without knowing
// about audio worklets.

use std::sync::atomic::{AtomicU8, Ordering};

// Let's implement a simple sine oscillator with variable frequency and volume.
pub struct Oscillator {
    params: &'static Params,
    accumulator: u32,
}

impl Oscillator {
    pub fn new(params: &'static Params) -> Self {
        Self {
            params,
            accumulator: 0,
        }
    }
}

impl Oscillator {
    pub fn process(&mut self, output: &mut [f32]) -> bool {
        // This method is called in the audio process thread.
        // All imports are set, so host functionality available in worklets
        // (for example, logging) can be used:
        // `web_sys::console::log_1(&JsValue::from(output.len()));`
        // Note that currently TextEncoder and TextDecoder are stubs, so passing
        // strings may not work in this thread.
        for a in output {
            let frequency = self.params.frequency.load(Ordering::Relaxed);
            let volume = self.params.volume.load(Ordering::Relaxed);
            self.accumulator += u32::from(frequency);
            *a = (self.accumulator as f32 / 512.).sin() * (volume as f32 / 100.);
        }
        true
    }
}

#[derive(Default)]
pub struct Params {
    // Use atomics for parameters so they can be set in the main thread and
    // fetched by the audio process thread without further synchronization.
    frequency: AtomicU8,
    volume: AtomicU8,
}

impl Params {
    pub fn set_frequency(&self, frequency: u8) {
        self.frequency.store(frequency, Ordering::Relaxed);
    }
    pub fn set_volume(&self, volume: u8) {
        self.volume.store(volume, Ordering::Relaxed);
    }
}
