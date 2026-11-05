# `catch`

The `catch` attribute allows catching a JavaScript exception. This can be
attached to any imported function or method, and the function must return a
`Result` where the `Err` payload is a `JsValue`:

```rust
#[wasm_bindgen]
extern "C" {
    // `catch` on a standalone function.
    #[wasm_bindgen(catch)]
    fn foo() -> Result<(), JsValue>;

    // `catch` on a method.
    type Zoidberg;
    #[wasm_bindgen(catch, method)]
    fn woop_woop_woop(this: &Zoidberg) -> Result<u32, JsValue>;
}
```

If calling the imported function throws an exception, then `Err` will be
returned with the exception that was raised. Otherwise, `Ok` is returned with
the result of the function.

An import returning a `js_sys::Promise<T>` may instead keep its `Promise`
return. A synchronous throw is then returned as a rejected promise, so it
surfaces where the promise is awaited, as it would for an `async` JS function:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    fn fetch_text(url: &str) -> Promise<String>;
}

// `Err` here whether `fetchText` threw or its promise rejected.
let result = fetch_text("/").await;
```

The return type converts the exception through the
`wasm_bindgen::convert::FromCaughtException` trait, which `js-sys` implements
for `Promise<T, E>`.

> By default `wasm-bindgen` will take no action when Wasm calls a JS function
> which ends up throwing an exception. The Wasm spec right now doesn't support
> stack unwinding and as a result Rust code **will not execute destructors**.
> This can unfortunately cause memory leaks in Rust right now.
>
> [This limitation is entirely avoided when building with `-Cpanic=unwind` and
> the `std` feature enabled.](../catch-unwind.md) Unexpected JS exceptions that
> would otherwise cause issues will result in a proper unwind, with the JS
> exception propagated to the caller and destructors running correctly.
