# Catching Panics

By default, when a Rust function exported to JavaScript panics, Rust will abort
and any allocated resources will be leaked. The `catch-unwind` feature provides
a way to catch Rust panics at the JavaScript boundary and convert them to
JavaScript exceptions, allowing proper cleanup and error handling.

When enabled, panics in exported Rust functions are caught and thrown as
`PanicError` exceptions in JavaScript. For async functions, the returned promise
is rejected with the `PanicError`.

## Requirements

The `catch-unwind` feature requires:

- **Rust nightly compiler** - The feature relies on `-Zbuild-std` which is only
  available on nightly
- **`panic=unwind`** - The panic strategy must be set to `unwind` (not `abort`)
- **`-Zbuild-std`** - Required to rebuild the standard library with the correct
  panic strategy
- **`std` feature** - The feature depends on `std` support

## Enabling the Feature

Add the `catch-unwind` feature to your `Cargo.toml`:

```toml
[dependencies]
wasm-bindgen = { version = "0.2", features = ["catch-unwind"] }

# If using async functions:
wasm-bindgen-futures = { version = "0.4", features = ["catch-unwind"] }
```

## Building

Build your project with the required flags:

```bash
cargo +nightly build --target wasm32-unknown-unknown -Zbuild-std
```

Or set these in `.cargo/config.toml`:

```toml
[unstable]
build-std = ["std", "panic_unwind"]

[build]
target = "wasm32-unknown-unknown"

[profile.release]
panic = "unwind"
```

Then build with:

```bash
cargo +nightly build
```

## How It Works

### Synchronous Functions

When a synchronous exported function panics, the panic is caught and a
`PanicError` is thrown to JavaScript:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```

```javascript
import { divide } from './my_wasm_module.js';

try {
    divide(10, 0);
} catch (e) {
    console.log(e.name);    // "PanicError"
    console.log(e.message); // "division by zero"
}
```

### Async Functions

For async functions, panics cause the returned promise to be rejected with a
`PanicError`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fetch_data(url: String) -> Result<JsValue, JsValue> {
    if url.is_empty() {
        panic!("URL cannot be empty");
    }
    // ... fetch implementation
    Ok(JsValue::NULL)
}
```

```javascript
import { fetch_data } from './my_wasm_module.js';

try {
    await fetch_data("");
} catch (e) {
    console.log(e.name);    // "PanicError"
    console.log(e.message); // "URL cannot be empty"
}
```

## The PanicError Class

When a panic occurs, a `PanicError` JavaScript exception is created with:

- `name` property set to `"PanicError"`
- `message` property containing the panic message (if the panic was created with
  a string message)

If the panic payload is not a `String` or `&str` (e.g., `panic_any(42)`), the message will
be `"No panic message available"`.

## Limitations

### Nightly Only

This feature requires a nightly Rust compiler and will not work on stable Rust.

### UnwindSafe Requirement

All function arguments must satisfy Rust's `UnwindSafe` trait. This is
automatically handled by wrapping arguments in `AssertUnwindSafe`, but be aware
that this assumes your code handles potential inconsistent state after a panic.

### Mutable Slice Arguments

Functions with `&mut [T]` slice arguments cannot be used with `catch-unwind`
because mutable slices are not `UnwindSafe`. Consider using owned types like
`Vec<T>` instead.

## See Also

- [`catch` attribute](./attributes/on-js-imports/catch.md) - For catching
  JavaScript exceptions in Rust
- [`Result<T, E>` type](./types/result.md) - For explicit error handling between
  Rust and JavaScript
