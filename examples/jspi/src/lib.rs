//! # JSPI (JS Promise Integration) Example
//!
//! WebAssembly JSPI lets **non-async** Rust code suspend a WASM fiber while
//! awaiting a JavaScript `Promise`, without blocking the browser's event loop.
//!
//! ## Key difference from `wasm-bindgen-futures`
//!
//! | | `wasm-bindgen-futures` | JSPI |
//! |---|---|---|
//! | Rust call style | `async fn` | ordinary `fn` |
//! | Call chain | fully async | can be sync |
//! | Event loop blocked? | no | no |
//!
//! ## How it works (overview)
//!
//! 1. Imports marked `#[wasm_bindgen(suspending)]` are automatically wrapped
//!    with `new WebAssembly.Suspending(...)` by the generated glue.
//! 2. Exports marked `#[wasm_bindgen(jspi)]` are automatically wrapped with
//!    `WebAssembly.promising` via a lazy cache in the generated glue.
//! 3. Inside Rust, [`block_on_promise`] / [`block_on`] from `js_sys::futures::jspi`
//!    store the pending `Promise`, call `jspi_do_suspend` (which suspends the fiber),
//!    then read the resolved value after the fiber resumes.
//!
//! ## Browser support (2025)
//! - Chrome 117+
//! - Firefox 150+ (`javascript.options.wasm_js_promise_integration = true`)
//! - Safari 18.4+
//!
//! See [`index.html`](../index.html) for the full setup and live demo.

use js_sys::Promise;
use wasm_bindgen::prelude::*;

use js_sys::futures::jspi::block_on_promise;

// ─────────────────────────────────────────────────────────────────────────────
// Example: non-async sleep
// ─────────────────────────────────────────────────────────────────────────────

/// Sleep for `ms` milliseconds.
///
/// This is a plain (non-`async`) Rust function, yet it awaits a `setTimeout`
/// promise via JSPI without blocking the browser's event loop.
///
/// The `#[wasm_bindgen(jspi)]` attribute causes the generated JS glue to wrap
/// this export with `WebAssembly.promising`, so callers receive a `Promise`.
#[wasm_bindgen(jspi)]
pub fn do_sleep(ms: u32) {
    let promise = Promise::new(&mut |resolve, _| {
        web_sys::window()
            .expect_throw("no window")
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32)
            .expect_throw("setTimeout failed");
    });
    block_on_promise(&promise).unwrap_throw();
}
