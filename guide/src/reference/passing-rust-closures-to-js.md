# Passing Rust Closures to Imported JavaScript Functions

The `Closure` type is the way to pass Rust closures to JavaScript. It is defined
in the `wasm_bindgen` crate and exported in `wasm_bindgen::prelude`.

All `Closure` variants are **unwind safe**: when built with `panic=unwind`,
panics inside closures are caught and converted to JavaScript `PanicError`
exceptions. See [Catching Panics](./catch-unwind.md) for details.

## Choosing a `Closure` API

| Use case | Recommended API |
|----------|----------------|
| Immediate/synchronous callbacks (forEach, map, etc.) | `ClosureBorrow::new` / `ClosureBorrow::new_mut` |
| Event listeners, timers, retained callbacks | `Closure::new` |
| One-shot callbacks (e.g., Promise handlers) | `Closure::once` or `Closure::once_into_js` |

## Immediate Closures with `ClosureBorrow`

Use `ClosureBorrow::new` (for `Fn`) or `ClosureBorrow::new_mut` (for `FnMut`) when
JavaScript will call the closure immediately and not retain it. This is the
recommended approach for synchronous callbacks like array iteration, Promise
executors, and similar APIs.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // A JS function that calls the callback immediately with a value
    fn call_with_value(cb: &Closure<dyn FnMut(u32)>, value: u32);
}

let mut result = 0;

// ClosureBorrow::new_mut allows capturing &mut result without 'static
{
    let mut func = |value: u32| {
        result = value * 2;
    };
    let closure = ClosureBorrow::new_mut(&mut func);
    call_with_value(closure.as_ref(), 21);
}

assert_eq!(result, 42);
```

Benefits of borrowed closures:

- **Non-`'static` captures**: Unlike `Closure::new`, you can capture references
  to local variables
- **Automatic cleanup**: The closure is invalidated when the `ClosureBorrow` is dropped
- **No heap allocation**: The closure data lives on the stack
- **Lifetime safety**: Rust's borrow checker ensures `ClosureBorrow` cannot
  outlive the closure's captured data

**Important**: The JavaScript function is only valid while the `ClosureBorrow`
exists. The `ClosureBorrow<'a, _>` has lifetime `'a` from the closure reference,
which transitively includes any data the closure captures. This means Rust
prevents you from holding the `ClosureBorrow` longer than its captured data lives.

If JavaScript retains a reference to the closure and calls it after the
`ClosureBorrow` is dropped, it will throw: "closure invoked recursively or
after being dropped".

## Long-Lived Closures with `Closure::new`

Use `Closure::new` when JavaScript needs to retain the closure and call it
later, such as for event listeners, timers, or callbacks that outlive the
current function call.

The validity of the JavaScript function is tied to the lifetime of the `Closure`
in Rust. **Once a `Closure` is dropped, it will deallocate its internal memory
and invalidate the corresponding JavaScript function so that any further
attempts to invoke it raise an exception.**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, clear its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.token);
    }
}

// Keep logging "hello" every second until the resulting `Interval` is dropped.
#[wasm_bindgen]
pub fn hello() -> Interval {
    Interval::new(1_000, || log("hello"))
}
```

`Closure` supports both `Fn` and `FnMut` closures, as well as arguments and
return values.

## One-Shot Closures with `Closure::once`

Use `Closure::once` for closures that should only be called once, such as
Promise handlers. This allows using `FnOnce` closures that consume captured
values.

```rust
use wasm_bindgen::prelude::*;

// Create a closure that consumes a String
let message = String::from("Hello!");
let closure: Closure<dyn FnMut()> = Closure::once(move || {
    // message is moved and consumed here
    web_sys::console::log_1(&message.into());
});
```

If you don't need to cancel the closure early, use `Closure::once_into_js` to
convert directly to a `JsValue`. Note that if the JavaScript function is never
called, the `FnOnce` and everything it closes over will leak.

```rust
use wasm_bindgen::prelude::*;

let callback = Closure::once_into_js(move || {
    // This runs when called from JS
});
// callback is a JsValue containing a JS function
```

## Panic Handling

When built with `panic=unwind`, all `Closure` variants catch panics and convert
them to JavaScript `PanicError` exceptions. This requires the closure to satisfy
Rust's `UnwindSafe` trait.

For more information on enabling panic catching, see [Catching
Panics](./catch-unwind.md).

### UnwindSafe Requirement

The default constructors (`Closure::new`, `Closure::wrap`, `Closure::once`,
`ClosureBorrow::new`, `ClosureBorrow::new_mut`) require that closures be
`UnwindSafe`. This is a marker trait that indicates a type is safe to use
across panic boundaries.

Common "not unwind safe" compiler errors are caused by capturing types with
interior mutability:

- `Rc<Cell<_>>`, `Rc<RefCell<_>>`
- Other interior mutability types

The compiler error will indicate which captured type is problematic.

**Fix 1: Use aborting variants**

If you don't need panic catching, use the `*_aborting` variants (`new_aborting`,
`wrap_aborting`, `once_aborting`, `once_into_js_aborting`,
`ClosureBorrow::new_aborting`, `ClosureBorrow::new_mut_aborting`) which do not
require `UnwindSafe`:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

let data = Rc::new(RefCell::new(0));

// No UnwindSafe requirement — aborts on panic instead of catching
let closure = Closure::new_aborting(move || {
    *data.borrow_mut() += 1;
});
```

**Fix 2: Assert unwind safety**

If you need panic catching and are confident your closure is safe to use across
panic boundaries, you can use `AssertUnwindSafe`:

```rust
use std::panic::AssertUnwindSafe;
use wasm_bindgen::prelude::*;

let closure = Closure::new(AssertUnwindSafe(move || {
    // you're asserting this is safe across panics
}));
```

### Type Inference with Box

`Closure::wrap` checks `UnwindSafe` on the concrete closure type before it is
erased to a trait object. Casting to a trait object too early defeats this
check:

```rust
// ❌ Wrong — cast erases concrete type, UnwindSafe can't be checked
Closure::wrap(Box::new(|| {}) as Box<dyn FnMut()>)
```

Instead, use one of these patterns:

```rust
// ✅ Correct — use turbofish, let concrete type flow through
Closure::<dyn FnMut()>::wrap(Box::new(|| {}))
```

```rust
// ✅ Correct — use type annotation on binding
let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {}));
```

---

## Legacy `&dyn Fn` and `&mut dyn FnMut`

> ⚠️ Note: This pattern will be **deprecated** going forward, to instead use the unwind-safe `ClosureBorrow` for immediate callbacks.

The `#[wasm_bindgen]` attribute also supports passing closures as `&dyn Fn` or
`&mut dyn FnMut` trait object references. However, **this pattern is not unwind
safe** — if a panic occurs inside such a closure, it may corrupt program state
rather than being converted to a JavaScript exception.

```rust
// Legacy `&dyn Fn` and `&mut dyn FnMut` bindings (soon to be deprecated).
#[wasm_bindgen]
extern "C" {
    fn takes_closure(f: &dyn Fn());
    fn takes_mut_closure(f: &mut dyn FnMut());
}
```

### Migrating to `ClosureBorrow`

Replace `&dyn Fn` / `&mut dyn FnMut` parameters with `&Closure<dyn Fn(...)>` and
use `ClosureBorrow::new` (for `Fn`) or `ClosureBorrow::new_mut` (for `FnMut`):

```rust
#[wasm_bindgen]
extern "C" {
    fn forEach(f: &mut dyn FnMut(JsValue));
}
forEach(&mut |value| { /* ... */ });

// ✅ NEW: Unwind safe
#[wasm_bindgen]
extern "C" {
    fn forEach(f: &Closure<dyn FnMut(JsValue)>);
}
{
    let mut func = |value| { /* ... */ };
    let closure = ClosureBorrow::new_mut(&mut func);
    forEach(closure.as_ref());
}
```

Note that `js-sys` currently still uses the `&dyn Fn` pattern for its callback
APIs (such as `Array::for_each`). These will be migrated to `Closure` in a
future release.
