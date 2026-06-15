#![cfg(js_sys_unstable_apis)]
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
    let promise = Promise::new(&mut |resolve, _reject| {
        // `resolve` is a typed `Function<fn(T) -> Undefined>` when
        // js_sys_unstable_apis is active; coerce to plain &Function for
        // set_timeout_with_callback_and_timeout_and_arguments_0.
        let resolve_fn: &js_sys::Function = wasm_bindgen::JsCast::unchecked_ref(&resolve);
        web_sys::window()
            .expect_throw("no window")
            .set_timeout_with_callback_and_timeout_and_arguments_0(resolve_fn, ms as i32)
            .expect_throw("setTimeout failed");
    });
    block_on_promise(&promise).unwrap_throw();
}

// ─────────────────────────────────────────────────────────────────────────────
// Example: shadow-stack depth
// ─────────────────────────────────────────────────────────────────────────────

/// Demonstrates the per-fiber shadow-stack limit set by `--jspi-stack-pages`.
///
/// The call tree allocates ~96 KiB of shadow-stack space across two active
/// frames while suspended:
///
/// | `--jspi-stack-pages` | Stack budget | Result |
/// |----------------------|-------------|--------|
/// | 1 (default)          | 64 KiB      | **silent overflow → corrupted return value** |
/// | 2                    | 128 KiB     | 49152 (correct) |
///
/// Build with the flag you want to test, then call this export:
///
/// ```sh
/// # overflows (default):
/// RUSTFLAGS=--cfg=js_sys_unstable_apis wasm-pack build --target web
///
/// # works:
/// RUSTFLAGS=--cfg=js_sys_unstable_apis wasm-pack build --target web \
///     -- -Z wasm-bindgen-extra-args='--jspi-stack-pages 2'
/// ```
#[wasm_bindgen(jspi)]
pub fn deep_stack() -> u32 {
    outer_frame()
}

/// 48 KiB shadow-stack frame.  Calls `inner_frame` while this frame is live.
#[inline(never)]
fn outer_frame() -> u32 {
    let buf = [1u8; 48 * 1024];
    // Passing a reference to a non-inlined callee forces `buf` onto the wasm
    // shadow stack (LLVM allocates addressable locals there).
    let _ = read_first(&buf);
    inner_frame()
}

/// Another 48 KiB shadow-stack frame.  Suspends while `outer_frame` is live.
///
/// At suspension time, `outer_frame` (48 KiB) + `inner_frame` (48 KiB) +
/// overhead are all active on the shadow stack (~96 KiB total).
#[inline(never)]
fn inner_frame() -> u32 {
    let buf = [1u8; 48 * 1024];
    let _ = read_first(&buf);
    // Suspend: the shadow stack must hold both frames simultaneously.
    // With --jspi-stack-pages 1 (64 KiB), it already overflowed before here.
    let promise = Promise::resolve(&JsValue::UNDEFINED);
    block_on_promise(&promise).unwrap_throw();
    // After resume, sum the buffer to produce a verifiable value (49152).
    // If the shadow stack overflowed, this memory is corrupt and the sum is wrong.
    buf.iter().copied().map(u32::from).sum()
}

/// Forces the caller to allocate its slice argument on the shadow stack.
/// Must be `#[inline(never)]` so LLVM cannot see through the call.
#[inline(never)]
fn read_first(s: &[u8]) -> u8 {
    s[0]
}
