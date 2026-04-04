# Writing Asynchronous Tests

Not all tests can execute immediately and some may need to do "blocking" work
like fetching resources and/or other bits and pieces. To accommodate this
asynchronous tests are also supported through the `futures` and
`wasm-bindgen-futures` crates.

Writing an asynchronous test is pretty simple, just use an `async` function!
`js_sys::Promise` implements `IntoFuture`, so you can `.await` it directly:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen_test]
async fn my_async_test() {
    // Create a promise that is ready on the next tick of the micro task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Await the promise directly — no conversion wrapper needed.
    let x = promise.await.unwrap();
    assert_eq!(x, 42);
}
```

If you need a named `Future` value, `JsFuture` is available from
`js_sys::futures` or from `wasm_bindgen_futures`:

```rust
use wasm_bindgen_futures::JsFuture;

let x = JsFuture::from(promise).await.unwrap();
```

## Rust compiler compatibility

Note that `async` functions are only supported in stable from Rust 1.39.0 and
beyond.

If you're using the `futures` crate from crates.io in its 0.1 version then
you'll want to use the `0.3.*` version of `wasm-bindgen-futures` and the `0.2.8`
version of `wasm-bindgen-test`. In those modes you'll also need to use
`#[wasm_bindgen_test(async)]` instead of using an `async` function. In general
we'd recommend using the nightly version with `async` since the user experience
is much improved!
