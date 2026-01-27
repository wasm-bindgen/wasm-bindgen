// FLAGS: --target=bundler
// FLAGS: --target=web
// FLAGS: --target=web --experimental-reset-state-function
// FLAGS: --target=no-modules
// FLAGS: --target=nodejs
// FLAGS: --target=nodejs --experimental-reset-state-function
// FLAGS: --target=deno
// FLAGS: --target=module
// FLAGS: --target=module --experimental-reset-state-function
// FLAGS: --target=experimental-nodejs-module

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
pub fn add_that_might_fail(a: u32, b: u32) -> u32 {
    assert!(random() > 0.5);
    a + b
}
