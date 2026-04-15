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

This works out of the box — the futures support is built into `js-sys` and
is always available.

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

## Combining JS Promises Concurrently

Rust's async ecosystem and JavaScript's promise ecosystem interoperate
cleanly: awaiting a [`Promise<T>`][`js_sys::Promise`] yields to the JS
event loop, so in-flight JS-side work (fetches, timers, I/O) continues in
parallel while Rust futures are pending. `futures_util`'s `join_all` /
`select` / `try_join_all` work correctly over `JsFuture`s and deliver
proper parallelism.

When your inputs are already JS `Promise<T>` values, the native
`Promise.all` / `Promise.allSettled` / `Promise.race` bindings on
[`js_sys::Promise`] are the idiomatic choice — they match JS semantics
exactly and avoid the Rust executor polling each child on every wake.

The canonical recipe collects the promise-producing iterator straight into a
typed [`Array<Promise<T>>`] and hands it to the combinator — no turbofish
on the elements, no intermediate `Vec`:

### `Promise.all` — all must succeed

```rust
use js_sys::{Array, Promise};

// responses: Array<Response>
let responses = Promise::all_iterable(
    &(0..10)
        .map(|_| worker.fetch_with_str_and_init(&url, &init))
        .collect::<Array<_>>(),
)
.await?;
```

Rejects with the first rejection, matching `Promise.all` semantics.

### `Promise.allSettled` — wait for all, never reject early

```rust
use js_sys::{Array, Promise};

// results: Array<PromiseState<Response>>
let results = Promise::all_settled_iterable(
    &(0..10)
        .map(|_| worker.fetch_with_str_and_init(&url, &init))
        .collect::<Array<_>>(),
)
.await?;
for state in results.iter() {
    if state.is_fulfilled() {
        let value = state.get_value().unwrap();
    } else {
        let reason = state.get_reason().unwrap();
    }
}
```

### `Promise.race` — first to settle

```rust
use js_sys::{Array, Promise};

// first: Response
let first = Promise::race_iterable(
    &(0..10)
        .map(|_| worker.fetch_with_str_and_init(&url, &init))
        .collect::<Array<_>>(),
)
.await?;
```

If you already have a `Vec<Promise<T>>`, use [`Array::of`] to lift it:
`Promise::all_iterable(&Array::of(&promises))`.

### Heterogeneous `Promise.all` over a tuple

When the promises resolve to *different* types, collect them into a Rust
tuple and pass to [`Promise::all_tuple`]. The result is a typed
[`ArrayTuple<(T1, T2, ..., Tn)>`] you can destructure via `.into_tuple()`
back into a native Rust tuple. Implemented for arity 1..=8.

```rust
use js_sys::Promise;

// fetch_promise:  Promise<Response>
// buffer_promise: Promise<ArrayBuffer>
let (response, buffer) = Promise::all_tuple((fetch_promise, buffer_promise))
    .await?
    .into_tuple();
```

Rejects with the first rejection, matching `Promise.all` semantics.

For the `Promise.allSettled` analogue use [`Promise::all_settled_tuple`]; it
resolves to an `ArrayTuple<(PromiseState<T1>, ..., PromiseState<Tn>)>` and
never rejects early:

```rust
use js_sys::Promise;

let results = Promise::all_settled_tuple((fetch_promise, buffer_promise)).await?;
let (response_state, buffer_state) = results.into_tuple();
if response_state.is_fulfilled() {
    let response = response_state.get_value().unwrap();
    // ...
}
```

There is no `race_tuple` equivalent — `Promise.race` returns whichever input
settles first, whose type would be a union `T1 | T2 | ... | Tn` that Rust
can't express. For heterogeneous race, collect into an `Array<JsValue>` and
use [`Promise::race_iterable`] explicitly, then narrow with `dyn_into` at
the inspection site.

### Mixing Rust `Future`s with JS `Promise`s

If some of your inputs are Rust `Future<Output = Result<T, JsValue>>` values
rather than JS `Promise<T>`, lift each one explicitly with
[`future_to_promise_typed`] at the call site:

```rust
use js_sys::{futures::future_to_promise_typed, Array, Promise};

// For homogeneous batches, mix both shapes into one `Array<Promise<T>>`:
let responses = Promise::all_iterable(
    &[
        fetch_promise,
        future_to_promise_typed(async { Ok(fetch_via_rust().await?) }),
    ]
    .into_iter()
    .collect::<Array<_>>(),
)
.await?;

// For heterogeneous tuples, lift each future before passing the tuple:
let (response, buffer) = Promise::all_tuple((
    fetch_promise,
    future_to_promise_typed(async { Ok(buffer_from_rust().await?) }),
))
.await?
.into_tuple();
```

`future_to_promise_typed` spawns the `Future` on the current thread and
returns a JS `Promise<T>` that settles with its result — from that point on
the two shapes are interchangeable.

[`js_sys::Promise`]: https://docs.rs/js-sys/*/js_sys/struct.Promise.html
[`Array<Promise<T>>`]: https://docs.rs/js-sys/*/js_sys/struct.Array.html
[`Array::of`]: https://docs.rs/js-sys/*/js_sys/struct.Array.html#method.of
[`future_to_promise_typed`]: https://docs.rs/js-sys/*/js_sys/futures/fn.future_to_promise_typed.html
[`Promise::all_tuple`]: https://docs.rs/js-sys/*/js_sys/struct.Promise.html#method.all_tuple
[`Promise::all_settled_tuple`]: https://docs.rs/js-sys/*/js_sys/struct.Promise.html#method.all_settled_tuple
[`Promise::race_iterable`]: https://docs.rs/js-sys/*/js_sys/struct.Promise.html#method.race_iterable
[`ArrayTuple<(T1, T2, ..., Tn)>`]: https://docs.rs/js-sys/*/js_sys/struct.ArrayTuple.html

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

## Using `js_sys` directly without `wasm-bindgen-futures`

By default the `#[wasm_bindgen]` macro generates async glue that references
`wasm_bindgen_futures`. If you want to depend only on `js-sys` and avoid
pulling in `wasm-bindgen-futures`, depend directly on `js-sys` and enable the
`js-sys` feature on `wasm-bindgen`:

```toml
[dependencies]
js-sys = "..."
wasm-bindgen = { version = "...", features = ["js-sys"] }
```

```rust
#[wasm_bindgen]
pub async fn foo() {
    // codegen uses js_sys::futures::future_to_promise / spawn_local
}

#[wasm_bindgen]
extern "C" {
    async fn fetch(url: &str) -> JsValue;
}
```

This feature is not enabled automatically by transitive `js-sys` dependencies:
the crate using `#[wasm_bindgen]` must be able to resolve the generated
`::js_sys` path. The existing `js_sys = my_crate::js_sys` attribute remains
available as a path override for re-export crates.

## Compatibility note

`wasm-bindgen-futures` is now a thin re-export shim. The futures implementation
lives in `js-sys` and is always available (no feature gate required). All
existing import paths (`wasm_bindgen_futures::JsFuture`,
`wasm_bindgen_futures::spawn_local`, etc.) continue to work without changes.

Learn more:

* [`js_sys::futures` API documentation][js-sys-docs]
* [`wasm_bindgen_futures` on crates.io][crate]

[js-sys-docs]: https://docs.rs/js-sys/*/js_sys/futures/index.html
[crate]: https://crates.io/crates/wasm-bindgen-futures
