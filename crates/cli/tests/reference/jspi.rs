// FLAGS: --target=bundler
// FLAGS: --target=web
use wasm_bindgen::prelude::*;

// ── Suspending imports ───────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    /// A plain suspending import: the wasm fiber suspends until the JS
    /// function's returned Promise settles.  The generated glue wraps it with
    /// `new WebAssembly.Suspending(...)`; shadow-stack save/restore is
    /// instrumented into the wasm module itself.
    #[wasm_bindgen(suspending)]
    fn sleep(ms: u32);

    /// Suspending import with a return value: the settled value arrives as
    /// the raw externref return and is converted post-resume in Rust.
    #[wasm_bindgen(suspending)]
    fn fetch_number() -> u32;

    /// Non-externref return: marshalled to `String` post-resume via a
    /// `__wbindgen_cast_*` adapter shim.
    #[wasm_bindgen(suspending)]
    fn fetch_text() -> String;

    /// `catch` + `suspending`: a rejection is caught in-wasm at the resume
    /// point and surfaced as `Err` data.
    #[wasm_bindgen(catch, suspending)]
    fn try_fetch() -> Result<u32, JsValue>;
}

// ── JSPI exports ─────────────────────────────────────────────────────────────

/// Export returning void: wrapped with `WebAssembly.promising` in JS.
/// TypeScript signature becomes `(): Promise<void>`.
#[wasm_bindgen(jspi)]
pub fn do_work() {
    sleep(100);
}

/// Export returning a primitive: TypeScript becomes `(): Promise<number>`.
#[wasm_bindgen(jspi)]
pub fn compute() -> u32 {
    fetch_number() + fetch_text().len() as u32 + try_fetch().unwrap_or(13)
}
