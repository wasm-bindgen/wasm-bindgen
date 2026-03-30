# Catching Panics

By default, when a Rust function exported to JavaScript panics, Rust will abort
and any allocated resources will be leaked. If you build with with
`-Cpanic=unwind` and the `std` feature, Rust panics will be caught at the
JavaScript boundary and converted into JavaScript exceptions, allowing proper
cleanup and error handling.

When enabled, panics in exported Rust functions are caught and thrown as
`PanicError` exceptions in JavaScript. For async functions, the returned promise
is rejected with the `PanicError`.

## Requirements

- **`panic=unwind`** - The panic strategy must be set to `unwind` (not `abort`)
- **`-Zbuild-std`** - Required to rebuild the standard library with the `unwind`
  panic strategy
- **Rust nightly compiler** - `-Zbuild-std` is only available on nightly
- **`std` feature** - `std` support is required to use
  `std::panic::catch_unwind`. to catch panics.

## Building

Build your project with the required flags:

```bash
RUSTFLAGS="-Cpanic=unwind" cargo +nightly build --target wasm32-unknown-unknown -Zbuild-std=std,panic_unwind
```

Or set these in `.cargo/config.toml`:

```toml
[unstable]
build-std = ["std", "panic_unwind"]

[build]
target = "wasm32-unknown-unknown"
rustflags = ["-C", "panic=unwind"]

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

## Closures

When built with `panic=unwind`, certain `ScopedClosure` and `ImmediateClosure` constructors
catch panics and convert them to JavaScript `PanicError` exceptions. The panic-catching
constructors are:

- `Closure::new`, `Closure::own`, `Closure::once`, `Closure::once_into_js`
- `Closure::wrap`, `Closure::wrap_assert_unwind_safe`
- `Closure::own_assert_unwind_safe`, `Closure::once_assert_unwind_safe`
- `Closure::borrow` (Fn), `Closure::borrow_mut` (FnMut)
- `Closure::borrow_assert_unwind_safe`, `Closure::borrow_mut_assert_unwind_safe`
- `ImmediateClosure::new` (Fn), `ImmediateClosure::new_mut` (FnMut)
- `ImmediateClosure::new_assert_unwind_safe` (Fn), `ImmediateClosure::new_mut_assert_unwind_safe` (FnMut)

The `*_aborting` variants (`own_aborting`, `once_aborting`, `wrap_aborting`, `borrow_aborting`,
`borrow_mut_aborting`, `ImmediateClosure::new_aborting`, `ImmediateClosure::new_mut_aborting`)
do NOT catch panics and will abort if the closure panics. These variants also don't require
the `MaybeUnwindSafe` bound.

Note: `Closure::new` is an alias for `Closure::own` and catches panics (requires `MaybeUnwindSafe`).

Catching panics in closures requires the closure to satisfy the `UnwindSafe` trait,
or you can use the `*_assert_unwind_safe` variants which skip this check.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setCallback(f: &Closure<dyn FnMut()>);
}

let closure = Closure::new(|| {
    panic!("closure panicked!");
});
setCallback(&closure);
```

```javascript
try {
    triggerCallback();
} catch (e) {
    console.log(e.name);    // "PanicError"
    console.log(e.message); // "closure panicked!"
}
```

`ImmediateClosure` also catches panics for immediate/synchronous callbacks:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn forEach(cb: ImmediateClosure<dyn FnMut(u32)>);
}

forEach(ImmediateClosure::new_mut(&mut |x| {
    if x == 0 {
        panic!("zero not allowed!");
    }
}));
```

For closures that should not catch panics (and abort the program instead), use
the `*_aborting` variants: `Closure::own_aborting`, `Closure::wrap_aborting`,
`Closure::once_aborting`,
`ScopedClosure::borrow_aborting`, `ScopedClosure::borrow_mut_aborting`,
`ImmediateClosure::new_aborting`, and `ImmediateClosure::new_mut_aborting`.
These do not require `UnwindSafe`.

> **Note**: `&dyn Fn` and `&mut dyn FnMut` arguments are unwind safe when
> `panic=unwind` is active. The `#[wasm_bindgen]` macro auto-injects a
> `MaybeUnwindSafe` bound, so the compiler will require callers to wrap
> non-unwind-safe captured values (e.g. `Cell<T>`, `&mut T`) in
> `std::panic::AssertUnwindSafe`. See
> [Passing Rust Closures to JavaScript](./passing-rust-closures-to-js.md#unwind-safety)
> for details and examples.

See [Passing Rust Closures to JavaScript](./passing-rust-closures-to-js.md) for
more details on closure APIs and the `UnwindSafe` requirement.

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

Exported function arguments and closure captures must satisfy Rust's `UnwindSafe`
trait. For `&dyn Fn` and `&mut dyn FnMut` import arguments the macro enforces
this via an auto-injected `MaybeUnwindSafe` bound. For captured values that are
not `UnwindSafe` (such as `&mut T`, `Cell<T>`, or `RefCell<T>`), wrap them in
`std::panic::AssertUnwindSafe` before the closure captures them:

```rust
let cell = std::cell::Cell::new(0u32);
let cell_ref = std::panic::AssertUnwindSafe(&cell);
takes_mut_closure(&mut move || { cell_ref.set(cell_ref.get() + 1); });
```

### Mutable Slice Arguments

Functions with `&mut [T]` slice arguments cannot be used because mutable slices
are not `UnwindSafe`. Consider using owned types like `Vec<T>` instead.

## Hard Abort Handlers

> **Note**: This feature is experimental and subject to change.

When built with `panic=unwind`, wasm-bindgen exposes hooks for responding to
*hard aborts* — non-recoverable errors such as `unreachable`, stack overflow,
or out-of-memory that cannot be caught by `catch_unwind`.  When a hard abort
occurs the Wasm instance is permanently poisoned and no further exports can be
called.

`set_on_abort` is available with `panic=unwind`.  `reinit` and `set_on_reinit`
additionally require passing `--experimental-reset-state-function` to
wasm-bindgen.

Because `--experimental-reset-state-function` creates a completely fresh
`WebAssembly.Instance` on reinit (resetting all Rust statics including the
registered handlers), both callbacks should be registered in a
`#[wasm_bindgen(start)]` function so they are re-registered automatically on
every instantiation.

### `wasm_bindgen::handler::set_on_abort`

Registers a callback that fires immediately after the instance is poisoned, but
before the original error propagates to JavaScript.  The terminated flag is
already set when the callback runs, so any re-entrant export call from within
the handler is immediately blocked.  A throwing or panicking handler cannot
suppress the original error.

`set_on_abort` returns the previously registered handler (`None` if none was
set), mirroring the `std::panic::set_hook` convention.

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

static ABORTED: AtomicBool = AtomicBool::new(false);

fn on_abort() {
    ABORTED.store(true, Ordering::SeqCst);
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_bindgen::handler::set_on_abort(on_abort);
}
```

### `wasm_bindgen::handler::reinit` and `set_on_reinit`

`reinit()` writes a sentinel value into the termination flag.  The next call to
any export detects this, creates a fresh `WebAssembly.Instance` from the same
module, and then invokes the registered `set_on_reinit` callback on the new
instance.

```rust
use wasm_bindgen::prelude::*;

fn on_abort() {
    // called on hard abort before the error propagates
}

fn on_reinit() {
    // called on every fresh instance after reinit()
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_bindgen::handler::set_on_abort(on_abort);
    wasm_bindgen::handler::set_on_reinit(on_reinit);
}

#[wasm_bindgen]
pub fn request_reinit() {
    wasm_bindgen::handler::reinit();
}
```

## See Also

- [`catch` attribute](./attributes/on-js-imports/catch.md) - For catching
  JavaScript exceptions in Rust
- [`Result<T, E>` type](./types/result.md) - For explicit error handling between
  Rust and JavaScript
