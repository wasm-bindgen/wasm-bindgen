// FLAGS: --target=web
// DEPENDENCY: wasm-bindgen-test = { path = '{root}/crates/test' }

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

// This test ensures that identifiers containing colons (like `__wbgt__reference_test::colon_test`)
// are properly quoted in the generated .d.ts file, since colons are not valid in TypeScript identifiers.

#[wasm_bindgen_test]
fn colon_test() {
    assert!(true)
}
