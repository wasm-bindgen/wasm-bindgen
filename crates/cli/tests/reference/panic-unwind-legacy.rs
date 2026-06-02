// FLAGS: --target=bundler

// This test is built with `-C panic=unwind -Cllvm-args=-wasm-use-legacy-eh`
// and `-Zbuild-std=std,panic_unwind` (see `runtest_panic_unwind_legacy` in the
// reference test harness). Under those flags the wasm has *legacy* Wasm
// exception-handling instructions (`try`/`catch`) and an
// `__instance_terminated` global. Legacy EH does not require
// `WebAssembly.JSTag` to be present on the host — Node 18/20 support `try`/
// `catch` but not `JSTag` — so wasm-bindgen emits a JS-side polyfill: a custom
// `new WebAssembly.Tag(...)` bound as the `__wbindgen_jstag` import plus a
// per-import `__wbg_jstag_wrap` that re-throws caught JS exceptions through
// it. This snapshot locks in that polyfill shape.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn maybe_panic(x: u32) -> u32 {
    if x == 0 {
        panic!("zero");
    }
    x + 1
}
