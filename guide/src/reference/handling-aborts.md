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

Any abort will set this `__instance_terminated` flag. When this happens, reentrancy guards
ensure that the WebAssembly instance cannot again execute, immediately throwing if any
new call is attempted instead.

## `set_on_abort`

`set_on_abort` registers a callback that fires when the instance has been
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

## `schedule_reinit` and `set_on_reinit`

`schedule_reinit()` schedules the module for reinitialization. The next call to
any export detects this, creates a fresh `WebAssembly.Instance` from the same
module, and then invokes the registered `set_on_reinit` callback on the new
instance, while keeping the same JS wrapper bindings in place but updated to
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

fn on_reinit() {
    // Called on every fresh instance after reinitialization.
    // Use this to restore application state.
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_bindgen::handler::set_on_abort(on_abort);
    wasm_bindgen::handler::set_on_reinit(on_reinit);
}
```

With this setup, a hard abort (or host-initiated termination) transparently
reinitializes the module on the next export call. The caller sees a fresh
instance with all Rust statics reset to their initial values.

## Host-Initiated Termination

The host (JavaScript) can terminate the instance by writing `1` to the
`__instance_terminated` flag in linear memory:

```javascript
const memory = new Int32Array(wasmExports.memory.buffer);
const addr = wasmExports.__instance_terminated.value;
memory[addr / 4] = 1;
```

On the next export call, the abort hook fires (if registered). If the hook
calls `schedule_reinit()`, the module reinitializes automatically. Otherwise,
the call throws `"Module terminated"`.

## See Also

- [Catching Panics](./catch-unwind.md) — catching recoverable Rust panics as
  JavaScript exceptions
