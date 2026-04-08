# Handling Aborts

When built with `panic=unwind`, wasm-bindgen can catch Rust panics at the
JavaScript boundary (see [Catching Panics](./catch-unwind.md)). However, there
will still always be non-recoverable *hard aborts* — `unreachable` instructions,
stack overflow, or out-of-memory — that cannot be caught by `catch_unwind`. When a
hard abort occurs the Wasm instance is permanently poisoned: any subsequent
export call will throw `"Module terminated"`.

The `wasm_bindgen::handler` module provides hooks for responding to these events
and optionally recovering by reinitializing the module.

> **Note:** Hard abort detection and the abort handler API currently require
> `panic=unwind`. Support for `panic=abort` builds may be added in a future
> release.

## Termination States

When using `panic=unwind`, the `(export "__instance_terminated" (global i32))` flag
points to a boolean in linear memory:

| Value | Meaning | Behavior on next export call |
|-------|---------|------------------------------|
| `0` | Live | Normal execution |
| `1` | Terminated | Abort hook fires (if not already called), then throws `"Module terminated"` |

All aborts will set this `__instance_terminated` flag.

When the terminated state is set, reentrancy guards call the abort hook and
ensure that the WebAssembly instance cannot again execute, immediately
throwing if any new call is attempted instead.

## Abort Handling

`handler::set_on_abort` registers a callback that fires when the instance has been
aborted.

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

The abort handler is also invoked if the host terminates the instance by
writing directly to the `__instance_terminated` flag from JavaScript. This
means the handler fires on the next export call, giving it a chance to respond
(including calling `schedule_reinit()` to trigger automatic recovery).

## Reinitialization

`handler::schedule_reinit()` can be used to reinitialize the WebAssembly instance, while
keeping the JS wrapper bindings in place, performing a transparent reinitialization
of the library when state loss is acceptable.

`schedule_reinit` may be called at any time, including from within the abort hook
to create a self-recovering abort handler.

When called, `schedule_reinit()` the module for reinitialization. The next call to
any export detects this, creates a fresh `WebAssembly.Instance` from the same
module, while keeping the same JS wrapper bindings in place but updated to
the new instance.

When called during normal execution, the current call completes normally and
the reset happens on the next export call — no abort hook fires.

When called from within the abort handler, the abort hook has already fired and
the guard proceeds to reinitialize instead of throwing:

```rust
use wasm_bindgen::prelude::*;

fn on_abort() {
    // Schedule the module for reinitialization.
    wasm_bindgen::handler::schedule_reinit();
}

#[wasm_bindgen(start)]
pub fn start() {
    // Register the abort hook
    wasm_bindgen::handler::set_on_abort(on_abort);
}
```

With this setup, a hard abort (or host-initiated termination) transparently
reinitializes the module on the next export call. The caller sees a fresh
instance with all Rust statics reset to their initial values.

## See Also

- [Catching Panics](./catch-unwind.md) — catching recoverable Rust panics as
  JavaScript exceptions
