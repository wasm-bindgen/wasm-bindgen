use wasm_bindgen::prelude::*;

// `generic_per_mono` is accepted on imported functions, imported generic
// types, or an `extern "C"` block whose functions inherit it. Other positions
// are rejected rather than silently changing nothing.

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
    // Imported generic classes opt in separately from their methods.
    #[wasm_bindgen(generic_per_mono)]
    pub type ImportedType<T>;

    // An imported `static` is read through a single getter shim, and cannot be
    // generic.
    #[wasm_bindgen(generic_per_mono, js_name = someGlobal, thread_local_v2)]
    static SOME_GLOBAL: JsValue;
}

#[wasm_bindgen]
extern "C" {
    // There is nothing to monomorphise on a non-generic imported type.
    #[wasm_bindgen(generic_per_mono)]
    pub type NonGeneric;
}

fn main() {}
