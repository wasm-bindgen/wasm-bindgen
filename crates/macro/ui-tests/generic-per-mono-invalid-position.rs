use wasm_bindgen::prelude::*;

// `generic_per_mono` only means something on an imported function, or on an
// `extern "C"` block whose functions inherit it. Anywhere else it would silently
// change nothing while looking like it changed the ABI, so it is rejected
// outright rather than warned about.

// Not an import at all: exported functions are never generic.
#[wasm_bindgen(generic_per_mono)]
pub fn exported_fn(x: u32) -> u32 {
    x
}

// Exported types have no imported-function ABI to opt in to.
#[wasm_bindgen(generic_per_mono)]
pub struct Exported {
    pub field: u32,
}

#[wasm_bindgen]
extern "C" {
    // An imported `type` is bound by its class, not by a per-call shim.
    #[wasm_bindgen(generic_per_mono)]
    pub type ImportedType;

    // An imported `static` is read through a single getter shim, and cannot be
    // generic.
    #[wasm_bindgen(generic_per_mono, js_name = someGlobal, thread_local_v2)]
    static SOME_GLOBAL: JsValue;
}

fn main() {}
