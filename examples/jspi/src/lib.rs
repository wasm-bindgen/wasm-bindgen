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
//! ## Browser support
//! - Chrome 137+ (enabled by default; 119–136 behind a flag/origin trial)
//! - Firefox 150+ (`javascript.options.wasm_js_promise_integration = true`)
//! - Safari 18.4+ (Develop ▸ Feature Flags)
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
/// | 1 (default)          | 64 KiB      | **overflow → `RangeError` thrown by the guard band** |
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
    // Suspend: the shadow stack must hold both frames simultaneously. With
    // --jspi-stack-pages 1 (64 KiB) the SP has descended into the guard band by
    // now, so the suspending-import wrapper throws a RangeError here instead of
    // resuming a fiber that overran its slot.
    let promise = Promise::resolve(&JsValue::UNDEFINED);
    block_on_promise(&promise).unwrap_throw();
    // After resume (≥2 pages), sum the buffer to produce a verifiable value
    // (49152).
    buf.iter().copied().map(u32::from).sum()
}

/// Forces the caller to allocate its slice argument on the shadow stack.
/// Must be `#[inline(never)]` so LLVM cannot see through the call.
#[inline(never)]
fn read_first(s: &[u8]) -> u8 {
    s[0]
}

// ─────────────────────────────────────────────────────────────────────────────
// Example: deep recursion + post-resume heap allocation
// ─────────────────────────────────────────────────────────────────────────────

/// Stress-tests that `__stack_pointer` is correctly restored after suspension
/// for arbitrary call depth and for heap allocations that occur after resume.
///
/// The call tree recurses `depth` levels deep (each frame has a 1 KiB
/// shadow-stack local), suspends at the bottom, then allocates a `Vec` on
/// the way back up through every frame.
///
/// ## Why this validates the SP-restore invariant
///
/// `__stack_pointer` is a Wasm *global*; JSPI preserves Wasm locals across
/// fiber switches but **not** globals.  If another fiber runs while this one
/// is suspended it will overwrite the global.  Correctness therefore requires
/// that the correct SP is restored before any Wasm code in this fiber
/// continues.
///
/// wasm-bindgen's generated JS wraps every `#[wasm_bindgen(suspending)]`
/// import like this:
///
/// ```js
/// async function(...args) {
///     const __sp = wasm.__stack_pointer.value;          // save
///     try { return await __inner(...args); }             // fiber suspends
///     finally { wasm.__stack_pointer.value = __sp; }    // restore
/// }
/// ```
///
/// The `finally` block runs *before* the `WebAssembly.Suspending` mechanism
/// resumes the Wasm fiber, so by the time any Rust instruction executes after
/// `block_on_promise` returns, the SP is already correct — even after 20
/// nested frames and even when the very next operation (`release_id` →
/// `Vec::push` → `malloc`) allocates from the heap.
///
/// ## Expected return value
///
/// `deep_alloc(N)` returns `1000 + N*(N+1)/2`.
/// For `deep_alloc(20)` the expected value is **1210**.
#[wasm_bindgen(jspi)]
pub fn deep_alloc(depth: u32) -> u32 {
    deep_alloc_inner(depth)
}

/// Recursive helper: 1 KiB shadow-stack frame per level.
///
/// 20 levels × 1 KiB = 20 KiB, well within the default 1-page (64 KiB)
/// budget, so no `--jspi-stack-pages` increase is needed.
#[inline(never)]
fn deep_alloc_inner(depth: u32) -> u32 {
    // 1 KiB on the shadow stack — enough to accumulate meaningful depth while
    // staying within the default per-fiber budget.
    let buf = [depth as u8; 1024];
    // Pass a reference to a non-inlined callee so LLVM allocates `buf` on the
    // shadow stack rather than keeping it in wasm locals.
    let _ = read_first(&buf);

    if depth == 0 {
        // Deepest frame: suspend the fiber.
        let promise = Promise::resolve(&JsValue::UNDEFINED);
        block_on_promise(&promise).unwrap_throw();
        // First thing after resume: heap-allocate.  `block_on_promise` itself
        // calls `release_id` → `Vec::push` internally, so the very first post-
        // resume instruction is already an allocation.  This additional Vec
        // is a second, explicit heap allocation at the deepest call depth.
        let v: Vec<u32> = vec![1000];
        v[0] // base value
    } else {
        // Recurse first, then allocate on the way back up.
        let child = deep_alloc_inner(depth - 1);
        // Vec allocation at every level on the return path exercises malloc
        // with the restored SP at each call depth.
        let v: Vec<u32> = vec![depth];
        child + v[0]
    }
}
