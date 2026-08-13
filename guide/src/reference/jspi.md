# JSPI — JS Promise Integration

[WebAssembly JS Promise Integration (JSPI)][jspi-spec] lets Rust functions
suspend the WASM fiber while a JS `Promise` resolves, then resume — without
blocking the event loop.  The result: you can call fully Promise-based
browser APIs from ordinary Rust code, with no `async` call chain required.

[jspi-spec]: https://github.com/WebAssembly/js-promise-integration

> **Experimental.** JSPI support in wasm-bindgen is experimental and subject to
> change. Using `#[wasm_bindgen(jspi)]` or `#[wasm_bindgen(suspending)]` emits
> a compiler warning noting this status (silence it with `#[allow(deprecated)]`
> once acknowledged). The `js_sys::futures::jspi` runtime API is gated behind
> the `experimental-jspi` Cargo feature on `js-sys` — and on
> `wasm-bindgen-futures` for `#[wasm_bindgen(jspi)] async fn` exports, which
> expand to `wasm_bindgen_futures::jspi::future_to_promise`. These features
> are exempt from semver guarantees.

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

Because the settled value is returned directly, a suspending import is always
declared as a plain `fn` — combining `suspending` with `async` is a compile
error.

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

## Concurrency and Rust futures — no async machinery required

`block_on_promise` is the complete programming model, because promises are
*eager*: the JS work starts when the call is made, not when you wait on it.
Start several calls, then suspend:

```rust
#[wasm_bindgen(jspi)]
pub fn build_page() -> String {
    let a = fetch_text("/header.html");   // both requests already in flight
    let b = fetch_text("/body.html");
    let header = block_on_promise(&a).unwrap_throw();
    let body = block_on_promise(&b).unwrap_throw();   // often settled already
    // ...
}
```

Combinator semantics come from the promise level, where JS already has them:
suspend once on `Promise::all`, `Promise::race`, `Promise::any`, or
`Promise::all_settled`.

To await a Rust `Future` from sync jspi code, schedule it on the ordinary
microtask executor and suspend on its completion promise:

```rust
let value = block_on_promise(&wasm_bindgen_futures::future_to_promise(fut));
```

The future is polled by the normal executor on the event loop while the fiber
waits — no separate runtime is involved.

### Suspending under the async executor: `jspi::spawn_local`

`async` and JSPI compose: a suspending call in sync code is, from an
executor's perspective, just a poll that hasn't returned yet. What decides
whether it works is how the poll is *entered*. The plain microtask executor
polls tasks on a plain activation with no fiber underneath, so calling
`block_on_promise` from code being polled (a plain `async fn` export,
`spawn_local`, or a future passed to `future_to_promise`) fails with a
`SuspendError` at runtime.

`js_sys::futures::jspi::spawn_local` is the executor entry that grants the
capability: it runs a future like `spawn_local`, but each poll is entered
through a `WebAssembly.promising` boundary, so the task's whole call tree —
including sync callees — may suspend:

```rust
js_sys::futures::jspi::spawn_local(async move {
    let x = JsFuture::from(fetch_thing()).await;  // ordinary await
    let y = sync_helper_that_suspends();          // block_on_promise inside
    // ...
});
```

A suspension parks only that task's poll: the microtask queue, other tasks,
and the event loop all continue, and a wake arriving while the poll is
suspended re-polls after it returns. The stall unit is the task — futures
*joined inside* the task cannot be polled while a sync callee has it
suspended, which is the usual "don't block in async" trade made explicit.

If a poll unwinds — a panic under `panic=unwind`, or the rethrown rejection
of a non-`catch` suspending import — destructors run, only that task is
abandoned, and the failure surfaces as an unhandled promise rejection
carrying the original reason (like an exception from a plain `spawn_local`
task). Other tasks, fibers, and the executor are unaffected.

### `#[wasm_bindgen(jspi)]` on `async fn` — suspendable async exports

The same capability for exports. A jspi async export has the identical JS
contract to a plain async export — the caller receives a `Promise` — but the
body is scheduled via `jspi::spawn_local` (through
`jspi::future_to_promise`), so its whole call tree may suspend:

```rust
#[wasm_bindgen(jspi)]
pub async fn process() -> u32 {
    let x = JsFuture::from(fetch_thing()).await;  // ordinary await
    sync_helper_that_suspends(x)                  // block_on_promise inside
}
```

Note the export itself is *not* wrapped with `WebAssembly.promising` — the
task carries the capability and the promise is an ordinary one. Prefer plain
`async fn` unless the body actually reaches suspending sync code.

## Reentrancy

wasm-bindgen permits reentrancy — JS called through an import may
synchronously call back into wasm exports — and this can break Rust
invariants (e.g. a `RefCell` borrow or `static mut` access held across the
import call). JSPI is no different in this respect, and fully supports
reentrancy: every suspension point is additionally a reentrancy point, since
other exports, fibers, and tasks run while the fiber is suspended.

As always, when holding lifetimes over a reentrancy point — a borrow live
across `block_on_promise` or a suspending import call — care must be taken
that reentrant code cannot observe or contend with the borrowed state.

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

JSPI is not supported together with **threads/atomics** (shared memories):
JSPI itself is a single-threaded proposal, and the fiber state the
instrumentation maintains is per-instance. Building with both enabled is
rejected by the CLI.

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
