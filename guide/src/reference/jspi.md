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

## Full example — OPFS file system

The `jspi-opfs` example demonstrates all four patterns: `#[wasm_bindgen(jspi)]`
exports, multiple sequential `block_on_promise` calls, cross-context
`navigator.storage`, and testing with QUnit + Playwright.

[View the jspi-opfs example](../examples/jspi-opfs.md)

## Testing

JSPI exports require a real browser with the flag enabled; Node.js does not
support `WebAssembly.Suspending` / `WebAssembly.promising`.

### Building the examples

```
just build-jspi-opfs
```

This runs `npm run build` inside `examples/jspi-opfs/` and produces a
ready-to-serve `examples/dist/jspi-opfs/` directory.

### Running headless Playwright tests

```
just test-jspi-examples
```

Builds both JSPI examples and runs the Playwright test suite against them.
Chrome 117+ or Firefox 150+ must be available to the Playwright runner.

### Manual testing

Serve the built output from any static HTTP server (e.g. `npx serve`) over
`localhost` or HTTPS and open `index.html` or `tests.html`.

```
cd examples/dist/jspi-opfs
npx serve .
# then open http://localhost:3000/tests.html in Chrome 117+ or Firefox 150+
```
