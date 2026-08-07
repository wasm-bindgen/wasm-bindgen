# JSPI — JS Promise Integration

[WebAssembly JS Promise Integration (JSPI)][jspi-spec] lets plain (non-`async`)
Rust functions suspend the WASM fiber while a JS `Promise` resolves, then resume
— without blocking the event loop.  The result: you can call fully Promise-based
browser APIs from ordinary Rust code, with no `async` call chain required.

[jspi-spec]: https://github.com/WebAssembly/js-promise-integration

> **Experimental.** JSPI support in wasm-bindgen is experimental and subject to
> change. Using `#[wasm_bindgen(jspi)]` or `#[wasm_bindgen(suspending)]` emits
> a compiler warning noting this status (silence it with `#[allow(deprecated)]`
> once acknowledged), and the `js_sys::futures::jspi` runtime API is gated
> behind the `js_sys_unstable_apis` cfg.

## Runtime support

| Runtime           | Enabled by default | Behind a flag |
|-------------------|--------------------|---------------|
| Chrome / Chromium | 137                | 119–136 (`#enable-experimental-webassembly-jspi`, or origin trial) |
| Firefox           | 153                | 150–152 (`javascript.options.wasm_js_promise_integration`) |
| Safari            | Technology Preview 238 | — |
| Node.js           | 25                 | 24 (`--experimental-wasm-jspi`) |

JSPI shipped *enabled by default* in **Chrome 137** (119–136 required the
experimental-WebAssembly-JSPI flag or an origin trial), **Firefox 153**, and
**Node.js 25**. Safari support has landed in Safari Technology Preview 238 but
has not yet reached a stable Safari release. JSPI also requires a **secure
context** (HTTPS or `localhost`).

## Attributes

### `#[wasm_bindgen(jspi)]` on exports

Marks a Rust export so that wasm-bindgen wraps it with `WebAssembly.promising`.
From JavaScript the function returns a `Promise`, and its TypeScript signature
reflects the `Promise<T>` return type.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(jspi)]
pub fn compute() -> u32 {
    // May call block_on_promise() internally
    42
}
```

JavaScript caller:

```js
import { compute } from './my_module.js';

const result = await compute();   // Promise<number>
```

### `#[wasm_bindgen(suspending)]` on imports

Marks an imported JS function as suspending: wasm-bindgen wraps the import shim
with `new WebAssembly.Suspending(...)` so that calling it from within a
`#[wasm_bindgen(jspi)]` export suspends the fiber while the returned `Promise`
is pending. The declared return type is the type the promise *resolves to* —
the suspended call returns the settled value directly:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // JS: async function fetch_data() { ...; return "text"; }
    #[wasm_bindgen(suspending)]
    fn fetch_data() -> String;
}
```

Return values are marshalled *after* the fiber resumes (the settled value
arrives as a raw JS value and is converted with the same ABI semantics as any
other import return), so all `FromWasmAbi` return types work — strings,
numbers, `Option<T>`, `Vec<T>`, imported JS types, etc.

`catch` composes with `suspending` exactly like it does elsewhere: a rejected
promise (or a synchronous throw from the JS function) surfaces as `Err` with
the rejection reason:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, suspending)]
    fn fallible_fetch() -> Result<String, JsValue>;
}
```

Without `catch`, a rejection is an uncaught exception at the suspend point
(and terminates the instance when the abort handler is enabled), mirroring
non-`catch` synchronous imports.

## `block_on_promise` — await a single `Promise`

`js_sys::futures::jspi::block_on_promise` is the low-level primitive.  It
suspends the fiber until `promise` settles and returns the resolved value, or
propagates a rejection as `Err`.

```rust
use js_sys::futures::jspi::block_on_promise;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(jspi)]
pub fn fetch_and_return() -> String {
    let promise: Promise = some_async_js_api();
    let value = block_on_promise(&promise).expect_throw("fetch failed");
    value.as_string().unwrap_or_default()
}
```

`block_on_promise` can be called multiple times inside the same
`#[wasm_bindgen(jspi)]` export — each call suspends once and resumes when the
corresponding `Promise` resolves.

## `block_on` — drive a Rust `Future`

`js_sys::futures::jspi::block_on<F: Future>` runs a Rust `Future` to
completion inside a JSPI-suspended export.  It spins a minimal executor backed
by JSPI — each time the future returns `Poll::Pending` the fiber suspends, and
each time the internal waker fires the fiber resumes and polls again.

```rust
use js_sys::futures::jspi::block_on;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(jspi)]
pub fn run() {
    block_on(async {
        // Use .await as normal
        let result = some_rust_future().await;
        // ...
    });
}
```

Use `block_on_promise` when you hold a bare `js_sys::Promise`; use `block_on`
when you have a `Future` (e.g. returned from a `wasm_bindgen_futures`-based
helper or assembled with Rust async combinators).

## Shadow-stack management

JSPI preserves wasm locals across a suspension, but not globals — and the
LLVM shadow stack (where address-taken locals live) sits in linear memory
behind the `__stack_pointer` global. wasm-bindgen makes suspension safe with
an *evacuate-on-suspend* scheme instrumented directly into the wasm module:

- Fibers run on the ordinary main shadow stack, at its full size (typically
  1 MiB). There is no separate per-fiber stack, no fixed size limit to tune,
  and no special overflow behavior — deep recursion behaves exactly as in
  non-JSPI code.
- Just before a fiber suspends, its live shadow-stack region is copied out to
  a heap allocation and the stack pointer is reset, leaving the shadow stack
  free for anything that runs while the fiber is suspended (other fibers,
  synchronous calls, `spawn_local` tasks).
- As the very first instructions after the fiber resumes, the region is copied
  back to its original address and the stack pointer is restored — so all
  interior pointers into the stack are valid again before any user code runs.

Memory cost is proportional to the *live stack depth at each suspension*
(usually small), is paid only when a suspension actually happens, and is
returned to the Rust allocator on resume. A `#[wasm_bindgen(jspi)]` export
that never suspends costs nothing beyond the `WebAssembly.promising` wrapper.

This scheme is target independent: on `--target emscripten` the same
instrumentation operates against emscripten's stack pointer, with no
interaction with (or requirement for) emscripten's own JSPI/Asyncify support.

## Requirements

JSPI support requires **reference types** (enabled by default since Rust
1.82) and a runtime with **exception handling** support — the instrumented
module uses `try_table` and `WebAssembly.JSTag` to restore the shadow stack
on unwinding and to surface promise rejections as `Result` data. Every
JSPI-capable engine ships both. Note that post-processing tools need EH
enabled too (e.g. `wasm-opt --enable-exceptions`, or disable `wasm-opt` in
`wasm-pack` builds).

## Full example — OPFS file system

The `jspi-opfs` example demonstrates all four patterns: `#[wasm_bindgen(jspi)]`
exports, multiple sequential `block_on_promise` calls, cross-context
`navigator.storage`, and testing with Playwright.

[View the jspi-opfs example](../examples/jspi-opfs.md)

## Testing

JSPI exports require a JSPI-capable runtime. Chrome has JSPI enabled by default
since **v137** and Node.js since **v25** (v24 needs `--experimental-wasm-jspi`),
and CI runs
all three JSPI examples automatically via the Playwright test suite using a
Chrome channel new enough to have it on by default.

### Building the examples

```sh
cargo build -p wasm-bindgen-cli

cd examples/jspi-opfs
PATH="$(git rev-parse --show-toplevel)/target/debug:$PATH" npm run build
```

This produces a ready-to-serve `examples/dist/jspi-opfs/` directory.

### Running headless Playwright tests

```sh
cd examples
pnpm install
PREBUILT_EXAMPLES=1 pnpm exec playwright test -g "jspi"
```

Runs all three JSPI examples (`jspi`, `jspi-opfs`, `jspi-fetch-streams`) under
Chrome via Playwright.

### Manual testing

Serve the built output from any static HTTP server (e.g. `npx serve`) over
`localhost` or HTTPS and open `index.html`.

```sh
cd examples/dist/jspi-opfs
npx serve .
# then open http://localhost:3000/index.html in Chrome 137+
```

## Calling a suspending import outside a `jspi` export

A `#[wasm_bindgen(suspending)]` import may only be called while a
`WebAssembly.promising` frame is on the stack — i.e. transitively from a
`#[wasm_bindgen(jspi)]` export. Calling one from a plain export (or from the
module's start path) throws a `SuspendError` at the import boundary at runtime.

This cannot easily be a compile error: "is this function only ever reached from
a `jspi` export?" is a whole-program reachability property, not a local one. So
the failure surfaces at runtime as an opaque JS error type that does not point
at the offending Rust call. If you see a `SuspendError`, check that every path
reaching the suspending import originates in a `jspi` export.
