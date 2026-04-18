# Working with a JS `Promise` and a Rust `Future`

Many APIs on the web work with a `Promise`, such as an `async` function in JS.
Naturally you'll probably want to interoperate with them from Rust! To do that
you can use [`js_sys::Promise`] together with Rust `async` functions.

[`js_sys::Promise`]: https://docs.rs/js-sys/*/js_sys/struct.Promise.html

## Awaiting a `Promise` directly

`js_sys::Promise` implements [`IntoFuture`], so you can `.await` it directly
in any `async` function. This is the recommended approach:

```rust
use js_sys::Promise;
use wasm_bindgen::prelude::*;

async fn get_from_js() -> Result<JsValue, JsValue> {
    let promise = Promise::resolve(&42.into());
    let result = promise.await?;
    Ok(result)
}
```

This works out of the box when `wasm-bindgen-futures` is in your dependency
tree, which activates the `futures` feature on `js-sys`. You can also enable
it explicitly without `wasm-bindgen-futures`:

```toml
[dependencies]
js-sys = { version = "0.3", features = ["futures"] }
```

For typed promises the awaited value matches the type parameter directly:

```rust
use js_sys::{Promise, Number};

async fn get_number() -> Result<Number, JsValue> {
    let promise: Promise<Number> = fetch_number();
    let number = promise.await?;  // number: Number
    Ok(number)
}
```

[`IntoFuture`]: https://doc.rust-lang.org/std/future/trait.IntoFuture.html

## Using `JsFuture` explicitly

If you need a named `Future` type — for example to store it in a struct or
pass it around — use [`JsFuture`] from `js_sys::futures` or
`wasm_bindgen_futures`:

```rust
use js_sys::futures::JsFuture;
use wasm_bindgen::JsValue;

async fn get_from_js() -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::resolve(&42.into());
    let result = JsFuture::from(promise).await?;
    Ok(result)
}
```

`JsFuture` is also re-exported from `wasm_bindgen_futures` for backwards
compatibility:

```rust
use wasm_bindgen_futures::JsFuture;
```

A successful promise becomes `Ok` and a rejected promise becomes `Err`,
corresponding to JS `.then` and `.catch`.

[`JsFuture`]: https://docs.rs/js-sys/*/js_sys/futures/struct.JsFuture.html

## Importing JS `async` functions

You can import a JS async function directly with an `extern "C"` block, and
the promise will be converted to a future automatically. The return type can be
`JsValue`, no return at all, or `Result` and `Option` types to primitives or
types supporting [JsCast] conversions:

```rust
#[wasm_bindgen]
extern "C" {
    async fn async_func_1_ret_number() -> JsValue;
    async fn async_func_2();
    async fn async_func_3_ret_string() -> JsString;
    async fn async_func_4_ret_array() -> Uint8Array;
}

async fn get_from_js() -> f64 {
    async_func_1_ret_number().await.as_f64().unwrap_or(0.0)
}
```

[JsCast]: https://docs.rs/wasm-bindgen/*/wasm_bindgen/trait.JsCast.html

The `async` attribute can be combined with `catch` to manage errors from the
JS promise:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn async_func_3() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn async_func_4() -> Result<(), JsValue>;
}
```

## Exporting Rust `async fn` to JS

Use an `async` function with `#[wasm_bindgen]` to export a function that
returns a `Promise` to JavaScript:

```rust
#[wasm_bindgen]
pub async fn foo() {
    // ...
}
```

When invoked from JS the `foo` function here will return a `Promise`, so you
can use it as:

```js
import { foo } from "my-module";

async function shim() {
    const result = await foo();
    // ...
}
```

## Return values of `async fn`

When using an `async fn` in Rust and exporting it to JS there are some
restrictions on the return type. The return value of an exported Rust function
will eventually become `Result<JsValue, JsValue>` where `Ok` turns into a
successfully resolved promise and `Err` is equivalent to throwing an exception.

The following types are supported as return types from an `async fn`:

* `()` — turns into a successful `undefined` in JS
* `T: Into<JsValue>` — turns into a successful JS value
* `Result<(), E: Into<JsValue>>` — `Ok(())` resolves to `undefined`; `Err`
  rejects with `E` converted to a JS value
* `Result<T: Into<JsValue>, E: Into<JsValue>>` — both payloads are converted
  into a `JsValue`

Note that many types implement being converted into a `JsValue`, such as all
imported types via `#[wasm_bindgen]` (aka those in `js-sys` or `web-sys`),
primitives like `u32`, and all exported `#[wasm_bindgen]` types. In general,
you should be able to write code without having too many explicit conversions,
and the macro should take care of the rest!

## `spawn_local` and `future_to_promise`

Use [`spawn_local`] to run a `Future<Output = ()>` on the current thread
without returning a `Promise` to JavaScript:

```rust
use js_sys::futures::spawn_local;

spawn_local(async {
    // drive a future to completion on the JS microtask queue
});
```

Use [`future_to_promise`] to convert a Rust `Future` into a JavaScript
`Promise` explicitly:

```rust
use js_sys::futures::future_to_promise;
use wasm_bindgen::JsValue;

let promise = future_to_promise(async {
    Ok(JsValue::from(42))
});
```

Both functions are also re-exported from `wasm_bindgen_futures` unchanged.

[`spawn_local`]: https://docs.rs/js-sys/*/js_sys/futures/fn.spawn_local.html
[`future_to_promise`]: https://docs.rs/js-sys/*/js_sys/futures/fn.future_to_promise.html

## Concurrent I/O with Promise Combinators

When running multiple JS-backed async operations concurrently (fetch, KV, D1,
R2, etc.), use the promise combinators in `js_sys::futures` instead of
`futures_util::future::join_all`. The Rust combinator polls futures
cooperatively within the WASM executor, which serializes JS promise resolution.
The `js_sys` combinators delegate to `Promise.all`, `Promise.race`, etc.,
letting V8's event loop drive true concurrent I/O.

### `join_all` — all must succeed (homogeneous)

```rust
use js_sys::futures::join_all;

let promises: Vec<Promise> = (0..10)
    .map(|_| worker.fetch_with_str_and_init(&url, &init))
    .collect();
let responses: Array = join_all(promises).await?;
```

Rejects with the first rejection, like `Promise.all`.

### `join!` — all must succeed (heterogeneous)

```rust
use js_sys::join;

let results = join!(
    fetch_promise,        // Promise<Response>
    array_buffer_promise, // Promise<ArrayBuffer>
).await?;
let (response, buffer) = results.into_parts();
```

Each argument can be a different `Promise<T>`, *or* a Rust
`Future<Output = Result<T, JsValue>>` — `IntoPromise` lifts both uniformly,
so you can mix them in any position:

```rust
let results = join!(
    fetch_promise,                              // Promise<Response>
    async { Ok(compute_buffer_locally()) },     // Future<...>
).await?;
let (response, buffer) = results.into_parts();
```

Returns a `Promise<ArrayTuple<(T1, T2, ...)>>` whose shape is pinned by the
`PromiseTuple` trait (implemented for arities 1..=8). Destructure via
`.into_parts()`.

### `all_settled` — wait for all, never reject early (homogeneous)

```rust
use js_sys::futures::all_settled;

let results = all_settled(promises).await?;
for state in results.iter() {
    if state.is_fulfilled() {
        let value = state.get_value().unwrap();
    } else {
        let reason = state.get_reason().unwrap();
    }
}
```

### `all_settled!` — wait for all, never reject early (heterogeneous)

```rust
use js_sys::all_settled;

let results = all_settled!(
    fetch_promise,        // Promise<Response>
    array_buffer_promise, // Promise<ArrayBuffer>
).await?;
let (response_state, buffer_state) = results.into_parts();
if response_state.is_fulfilled() {
    let response = response_state.get_value().unwrap();
    // ...
}
```

Like `join!`, accepts a mix of `Promise<T>` and Rust
`Future<Output = Result<T, JsValue>>` in any position. Returns a
`Promise<ArrayTuple<(PromiseState<T1>, PromiseState<T2>, ...)>>`.

### `race` — first to settle

```rust
use js_sys::futures::race;

let first = race(promises).await?;
```

### `any` — first to succeed

```rust
use js_sys::futures::any;

let first_success = any(promises).await?;
```

Rejects with an `AggregateError` only if every promise rejects.

### `IntoPromise` trait

All homogeneous combinators accept any iterator of items implementing
`IntoPromise`, which is implemented for:

- `Promise<T>` — identity conversion
- `Future<Output = Result<T, JsValue>>` — converted via `future_to_promise_typed`

This means you can mix promises from JS APIs with Rust futures in
`join_all`, `race`, etc.

## Using Generic Promise Types

Promises support [erasable generic type parameters](./types/js-sys.md). With
`IntoFuture`, the resolved type flows through directly without a cast:

```rust
use js_sys::{Promise, Number, Array};

#[wasm_bindgen]
extern "C" {
    fn fetchNumbers() -> Promise<Array<Number>>;
}

async fn process_numbers() -> Result<f64, JsValue> {
    // Await the typed promise — no cast needed
    let numbers: Array<Number> = fetchNumbers().await?;

    let mut sum = 0.0;
    for i in 0..numbers.length() {
        sum += numbers.get(i).value_of();
    }
    Ok(sum)
}
```

## Compatibility note

`wasm-bindgen-futures` is now a thin re-export shim. The implementation lives
in `js-sys` under the `futures` feature, which `wasm-bindgen-futures` activates
automatically when it is a dependency. All existing import paths
(`wasm_bindgen_futures::JsFuture`, `wasm_bindgen_futures::spawn_local`, etc.)
continue to work without any changes.

Learn more:

* [`js_sys::futures` API documentation][js-sys-docs]
* [`wasm_bindgen_futures` on crates.io][crate]

[js-sys-docs]: https://docs.rs/js-sys/*/js_sys/futures/index.html
[crate]: https://crates.io/crates/wasm-bindgen-futures
