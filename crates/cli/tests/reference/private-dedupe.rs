// DEPENDENCY: private-dedupe-a = { path = '{root}/crates/cli/tests/reference/crates/private-dedupe-a' }
// DEPENDENCY: private-dedupe-b = { path = '{root}/crates/cli/tests/reference/crates/private-dedupe-b' }

// Two crates each exporting a `#[wasm_bindgen(private)]` struct named
// `Status`: the wasm shim symbols are mangled per crate so the link
// succeeds, and each crate gets its own internal `Status_<hash>` class in
// the generated bindings.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn statuses() -> Vec<JsValue> {
    vec![
        private_dedupe_a::make().into(),
        private_dedupe_b::make().into(),
    ]
}

// Vectors of another crate's private items resolve to that crate's renamed
// identity, not the current crate's.

#[wasm_bindgen]
pub fn b_statuses() -> Vec<private_dedupe_b::Status> {
    vec![private_dedupe_b::make()]
}

#[wasm_bindgen]
pub fn a_levels() -> Vec<private_dedupe_a::Level> {
    private_dedupe_a::levels()
}

#[wasm_bindgen]
pub fn b_levels() -> Vec<private_dedupe_b::Level> {
    private_dedupe_b::levels()
}
