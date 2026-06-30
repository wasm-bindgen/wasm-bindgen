// FLAGS: --target=bundler
// FLAGS: --target=web
// FLAGS: --target=bundler --jspi-stack-pages 4
use wasm_bindgen::prelude::*;

// ── Suspending imports ───────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    /// A plain suspending import: the wasm fiber suspends until the JS
    /// function's returned Promise settles.  The generated glue wraps it with
    /// `new WebAssembly.Suspending(...)` and saves/restores `__stack_pointer`
    /// around the suspension.
    #[wasm_bindgen(suspending)]
    fn sleep(ms: u32);

    /// Suspending import with a return value.
    #[wasm_bindgen(suspending)]
    fn fetch_number() -> u32;
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
    fetch_number()
}
