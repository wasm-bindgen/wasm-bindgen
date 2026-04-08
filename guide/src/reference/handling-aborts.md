# Handling Aborts

When built with `panic=unwind`, wasm-bindgen can catch Rust panics at the
JavaScript boundary (see [Catching Panics](./catch-unwind.md)). However, some
errors are non-recoverable *hard aborts* — `unreachable` instructions, stack
overflow, or out-of-memory — that cannot be caught by `catch_unwind`. When a
hard abort occurs the Wasm instance is permanently poisoned: any subsequent
export call will throw `"Module terminated"`.

The `wasm_bindgen::handler` module provides hooks for responding to these events
and optionally recovering by reinitializing the module.

> **Note:** Hard abort detection and the abort handler API currently require
> `panic=unwind`. Support for `panic=abort` builds may be added in a future
> release.

## Requirements

- **`panic=unwind`** — the abort handler relies on the exception-handling
  machinery emitted by `panic=unwind` builds.

The reinit machinery is automatically emitted when `schedule_reinit()` is used
in the Rust code — no CLI flag is required. `--experimental-reset-state-function`
is only needed if you want the public `__wbg_reset_state()` export.

Both handlers should be registered in a `#[wasm_bindgen(start)]` function so
they are re-registered automatically whenever the module is (re)instantiated,
since reinitialization creates a completely fresh `WebAssembly.Instance`,
resetting all Rust statics including the registered handlers.

## `set_on_abort`

Registers a callback that fires immediately after the instance is poisoned, but
before the original error propagates to JavaScript. The terminated flag is
already set when the callback runs, so any re-entrant export call from within
the handler is immediately blocked. A throwing or panicking handler cannot
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

The abort handler is also invoked when the host terminates the instance by
writing directly to the `__instance_terminated` flag from JavaScript. This
means the handler fires on the next export call, giving it a chance to respond
(including calling `schedule_reinit()` to trigger automatic recovery).

## `schedule_reinit` and `set_on_reinit`

`schedule_reinit()` schedules the module for reinitialization. The next call to
any export detects this, creates a fresh `WebAssembly.Instance` from the same
module, and then invokes the registered `set_on_reinit` callback on the new
instance.

When called outside of an abort (i.e. during normal execution), the current
call completes normally and the reset happens on the next export call — no
abort hook fires.

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

## Termination States

The `__instance_terminated` flag in linear memory is a simple boolean:

| Value | Meaning | Behavior on next export call |
|-------|---------|------------------------------|
| `0` | Live | Normal execution |
| `1` | Terminated | Abort hook fires (if not already called), then throws `"Module terminated"` |

Reinitialization state is tracked separately and is not visible through the
`__instance_terminated` address. When the abort hook calls `schedule_reinit()`,
the guard proceeds to reinitialize instead of throwing. `schedule_reinit()` can
also be called during normal execution (without any termination), in which case
the reset happens on the next export call without the abort hook firing.

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

## Manual Reset

With `--experimental-reset-state-function`, you can also reset the module
explicitly from JavaScript without going through the abort/reinit lifecycle:

```javascript
import { __wbg_reset_state } from './my_module.js';

// Explicitly reinitialize — creates a fresh WebAssembly.Instance
__wbg_reset_state();
```

## See Also

- [Catching Panics](./catch-unwind.md) — catching recoverable Rust panics as
  JavaScript exceptions
- [Command Line Interface](./cli.md#--experimental-reset-state-function) — the
  `--experimental-reset-state-function` flag
