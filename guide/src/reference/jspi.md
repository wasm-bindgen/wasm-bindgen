# JSPI — JS Promise Integration

[WebAssembly JS Promise Integration (JSPI)][jspi-spec] lets plain (non-`async`)
Rust functions suspend the WASM fiber while a JS `Promise` resolves, then resume
— without blocking the event loop.  The result: you can call fully Promise-based
browser APIs from ordinary Rust code, with no `async` call chain required.

[jspi-spec]: https://github.com/WebAssembly/js-promise-integration

## Browser support

| Browser           | Minimum version |
|-------------------|-----------------|
| Chrome / Chromium | 117             |
| Firefox           | 150             |
| Safari            | 18.4            |

JSPI requires a **secure context** (HTTPS or `localhost`).

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
is pending.

```rust
use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(suspending)]
    fn fetch_data() -> Promise;
}
```

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

## Shadow-stack size and `--jspi-stack-pages`

Each `#[wasm_bindgen(jspi)]` export allocates a fresh region of Wasm linear
memory for its fiber's shadow stack. The size is:

```
stack_size = N × 64 KiB    (N = --jspi-stack-pages, default 1)
```

Pass the flag to the `wasm-bindgen` CLI:

```sh
wasm-bindgen target/wasm32-unknown-unknown/release/my_module.wasm \
    --out-dir pkg --target web \
    --jspi-stack-pages 2
```

> **Warning — no guard page.** If a fiber's shadow stack overflows, writes go
> into adjacent linear memory and corrupt data silently. There is no trap, no
> error, and no diagnostic output. Symptoms include garbled return values,
> random assertion failures, and `unwrap_throw` panics at unexpected sites.

> **Note — no reclamation.** Freed fiber stacks return to a JS pool; Wasm
> linear memory never shrinks. If your application creates many concurrent
> fibers, the pool amortises re-use, but initial `memory.grow` calls are
> permanent.

### Choosing N

| Situation | Recommended N |
|-----------|---------------|
| Shallow call trees, small locals (typical) | 1 (default) |
| Moderate stack depth or medium-sized locals | 2–4 |
| Deep recursion or large stack-allocated buffers | 8–16 |

The default Wasm shadow stack is considerably larger than 64 KiB (typically
1 MiB). Code migrated from non-JSPI Rust may silently overflow a 1-page fiber
stack. If you suspect overflow, double `N` and see whether the problem
disappears.

### Demonstrating overflow: the `deep_stack` example

The `jspi` example ships a `deep_stack` export that allocates ~48 KiB per
frame across two live frames (~96 KiB total) while suspended.

| `--jspi-stack-pages` | Budget  | Outcome |
|----------------------|---------|---------|
| 1 (default)          | 64 KiB  | overflow → corrupted return value |
| 2                    | 128 KiB | returns `49152` (correct) |

Build and serve the example, then open `index.html`. Demo 3 reports the actual
vs expected return value, so you can see the corruption without reading assembly.

## Full example — OPFS file system

The `jspi-opfs` example demonstrates all four patterns: `#[wasm_bindgen(jspi)]`
exports, multiple sequential `block_on_promise` calls, cross-context
`navigator.storage`, and testing with Playwright.

[View the jspi-opfs example](../examples/jspi-opfs.md)

## Testing

JSPI exports require a real browser; Node.js does not support
`WebAssembly.Suspending` / `WebAssembly.promising`. Chrome has JSPI enabled by
default since v123, and CI runs all three JSPI examples automatically via the
Playwright test suite.

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
# then open http://localhost:3000/index.html in Chrome 123+
```
