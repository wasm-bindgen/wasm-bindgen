# JSPI — JS Promise Integration

[WebAssembly JS Promise Integration (JSPI)][jspi-spec] lets Rust functions
suspend the WASM fiber while a JS `Promise` resolves, then resume — without
blocking the event loop. The result: you can call fully Promise-based
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

## Attributes

### `#[wasm_bindgen(jspi)]` on exports

Marks a Rust export as suspendable: JS callers receive a `Promise`, and the
TypeScript signature reflects `Promise<T>`. Anywhere in the export's call
tree, a `#[wasm_bindgen(suspending)]` import call or `block_on_promise` can
suspend to the JS event loop.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(jspi)]
pub fn compute() -> u32 {
    // May call suspending imports / block_on_promise() internally
    42
}
```

```js
const result = await compute();   // Promise<number>
```

The attribute also composes with `async fn`: the JS contract is identical to
a plain async export, but the body's sync callees may suspend. Prefer plain
`async fn` unless the body actually reaches suspending sync code.

Returning `Result` rejects the returned promise with the `Err` value, for
both forms.

### `#[wasm_bindgen(suspending)]` on imports

Marks an imported JS function as suspending: calling it from within a
`#[wasm_bindgen(jspi)]` export suspends the fiber while the returned
`Promise` is pending. The declared return type is the type the promise
*resolves to* — the call returns the settled value directly, so a suspending
import is always a plain `fn` (`async` + `suspending` is a compile error):

```rust
#[wasm_bindgen]
extern "C" {
    // JS: async function fetch_data() { ...; return "text"; }
    #[wasm_bindgen(suspending)]
    fn fetch_data() -> String;
}
```

All `FromWasmAbi` return types work — strings, numbers, `Option<T>`,
`Vec<T>`, imported JS types, etc.

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

Without `catch`, a rejection unwinds — running destructors under
`panic=unwind` — and rejects the export's promise with the original reason
(or terminates the instance when the abort handler is enabled), mirroring
non-`catch` synchronous imports.

## `block_on_promise` — await a single `Promise`

`js_sys::futures::jspi::block_on_promise` suspends the fiber until the given
`Promise` settles, returning the resolved value as `Ok` or the rejection
reason as `Err`. It can be called any number of times within a `jspi` export.

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

Concurrency composes at the promise level, because promises are *eager* —
the JS work starts when the call is made, not when you suspend on it. Start
several calls and suspend on each, or suspend once on `Promise::all`,
`Promise::race`, `Promise::any`, or `Promise::all_settled`. A Rust `Future`
is awaited by scheduling it on the ordinary executor and suspending on its
completion promise:

```rust
let value = block_on_promise(&wasm_bindgen_futures::future_to_promise(fut));
```

## `jspi::spawn_local` — suspending under the async executor

Calling `block_on_promise` from code polled by the plain microtask executor
(a plain `async fn` export, `spawn_local`, or `future_to_promise`) fails with
a `SuspendError`: those polls have no fiber underneath.
`js_sys::futures::jspi::spawn_local` runs a future like `spawn_local`, but
grants the whole call tree — including sync callees — the ability to
suspend:

```rust
js_sys::futures::jspi::spawn_local(async move {
    let x = JsFuture::from(fetch_thing()).await;  // ordinary await
    let y = sync_helper_that_suspends();          // block_on_promise inside
    // ...
});
```

A suspension parks only that task's poll: the microtask queue, other tasks,
and the event loop all continue. If a poll unwinds — a panic under
`panic=unwind`, or the rethrown rejection of a non-`catch` suspending import
— destructors run, only that task is abandoned, and the failure surfaces as
an unhandled promise rejection carrying the original reason.

`#[wasm_bindgen(jspi)] async fn` exports are scheduled through this same
mechanism.

## Reentrancy

wasm-bindgen permits reentrancy — JS called through an import may
synchronously call back into wasm exports — and this can break Rust
invariants (e.g. a `RefCell` borrow or `static mut` access held across the
import call). JSPI is no different in this respect: reentrancy is fully
supported — multiple in-flight JSPI suspensions operate on separate stacks
without conflict — and every suspension point is additionally a reentrancy
point, since other exports, fibers, and tasks run while the fiber is
suspended.

As always, when holding lifetimes over a reentrancy point — a borrow live
across `block_on_promise` or a suspending import call — care must be taken
that reentrant code cannot observe or contend with the borrowed state.

## Requirements

JSPI support requires **reference types** (enabled by default since Rust
1.82) and a runtime with **exception handling** support; every JSPI-capable
engine ships both. Note that post-processing tools need EH enabled too
(e.g. `wasm-opt --enable-exceptions`, or disable `wasm-opt` in `wasm-pack`
builds).

JSPI is not supported together with **threads/atomics** (shared memories):
JSPI itself is a single-threaded proposal. Building with both enabled is
rejected by the CLI.

## Full example — OPFS file system

The `jspi-opfs` example demonstrates all four patterns: `#[wasm_bindgen(jspi)]`
exports, multiple sequential `block_on_promise` calls, cross-context
`navigator.storage`, and testing with Playwright.

[View the jspi-opfs example](../examples/jspi-opfs.md)

## Testing

JSPI exports require a JSPI-capable runtime (see the support table above).
CI runs all three JSPI examples (`jspi`, `jspi-opfs`, `jspi-fetch-streams`)
automatically via the Playwright test suite under Chrome.

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

### Manual testing

Serve the built output from any static HTTP server and open `index.html`
(the OPFS example needs a secure context — `localhost` or HTTPS — for
`navigator.storage`).

```sh
cd examples/dist/jspi-opfs
npx serve .
# then open http://localhost:3000/index.html in Chrome 137+
```

## Calling a suspending import outside a `jspi` export

A `#[wasm_bindgen(suspending)]` import may only be called while a
`WebAssembly.promising` frame is on the stack — i.e. transitively from a
`#[wasm_bindgen(jspi)]` export. Calling one from a plain export (or from the
module's start path) throws a `SuspendError` at the import boundary at
runtime.

This cannot easily be a compile error: "is this function only ever reached
from a `jspi` export?" is a whole-program reachability property, not a local
one. If you see a `SuspendError`, check that every path reaching the
suspending import originates in a `jspi` export.

## Shadow Stack Management

On suspension the shadow stack is saved into the heap, and restored back
onto the stack on resume. Each `#[wasm_bindgen(jspi)]` export records the
shadow-stack watermark at entry — its *base* — and each
`#[wasm_bindgen(suspending)]` call copies the live region `[SP, base)` out
to a heap allocation and resets SP to the base before suspending. The first
instructions after resume copy the region back to its original address and
restore SP from a wasm local (which JSPI preserves), so interior stack
pointers remain fully valid. This is correct by construction: a `Promise`
resumption is always dispatched from the JS event loop, with an empty wasm
stack, so the restored region has the stack to itself — the address range is
time-multiplexed, and the only live data in it at any moment belongs to the
currently executing stack.

Reentrancy adds one wrinkle: a promising export can be entered over live
frames (a sync export calls into JS, which calls the promising export),
giving it a stack offset — its base sits partway down the stack. The offset
is simply maintained; if the export then suspends, those leading frames
unwind while it is pending, and after resume the region above its base is
dead stack space, reclaimed when the promising call finally exits. There are
thus two kinds of exit: a promising exit that never suspended, which may
have a live parent stack region above it from reentrancy; and a promising
exit that did suspend, which — by definition of being resumed from the JS
event loop — has no live stack above it, only the dead stack space, which
must be bumped off on exit.

Since the unsuspended exit requires no stack shift while the resumed exit
does, the difference is tracked with an internal `__jspi_suspended` global —
zeroed at entry, set on every resume, consulted at exit — which is correct
under single-threading for the current promising execution. And "resumed"
implies "really suspended" exactly, because JSPI performs promise resolution
on every `Suspending` return — even a non-`Promise` return suspends for a
tick.
