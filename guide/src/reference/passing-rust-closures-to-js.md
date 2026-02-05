# Passing Rust Closures to Imported JavaScript Functions

The `Closure` type is the way to pass Rust closures to JavaScript. It is defined
in the `wasm_bindgen` crate and exported in `wasm_bindgen::prelude`.

All `Closure` variants are **unwind safe**: when built with `panic=unwind`,
panics inside closures are caught and converted to JavaScript `PanicError`
exceptions. See [Catching Panics](./catch-unwind.md) for details.

## Choosing a `Closure` API

| Use case | Recommended API |
|----------|----------------|
| Immediate/synchronous callbacks (forEach, map, etc.) | `Closure::with` |
| Event listeners, timers, retained callbacks | `Closure::new` |
| One-shot callbacks (e.g., Promise handlers) | `Closure::once` or `Closure::once_into_js` |

## Immediate Closures with `Closure::with`

Use `Closure::with` when JavaScript will call the closure immediately and not
retain it. This is the recommended approach for synchronous callbacks like
`Array.forEach`, `Array.map`, and similar APIs.

```rust
use wasm_bindgen::prelude::*;
use js_sys::Array;

let array = Array::of3(&1.into(), &2.into(), &3.into());
let mut sum = 0;

// Closure::with allows capturing &mut sum without 'static
Closure::with(&mut |value: JsValue, _index, _array| {
    sum += value.as_f64().unwrap() as i32;
}, |closure| {
    array.for_each(closure.as_ref().unchecked_ref());
});

assert_eq!(sum, 6);
```

Benefits of `Closure::with`:

- **Non-`'static` captures**: Unlike `Closure::new`, you can capture references
  to local variables
- **Automatic cleanup**: The closure is invalidated when `with` returns
- **No heap allocation**: The closure data lives on the stack

**Important**: The closure is only valid during the callback. Once `with`
returns, the JavaScript function is invalidated. If JavaScript retains the
closure and calls it later, it will throw: "closure invoked recursively or
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

The default constructors (`new`, `wrap`, `once`, `with`) require that closures
be `UnwindSafe`. This is a marker trait that indicates a type is safe to use
across panic boundaries.

Common "not unwind safe" compiler errors are caused by capturing types with
interior mutability:

- `Rc<Cell<_>>`, `Rc<RefCell<_>>`
- Other interior mutability types

The compiler error will indicate which captured type is problematic.

**Fix 1: Use aborting variants**

If you don't need panic catching, use the `*_aborting` variants (`new_aborting`,
`wrap_aborting`, `once_aborting`, `once_into_js_aborting`, `with_aborting`)
which do not require `UnwindSafe`:

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

## Deprecated: `&dyn Fn` and `&mut dyn FnMut`

> **Warning**: This pattern is deprecated and should not be used in new code.
> Use `Closure::with` instead for immediate callbacks.

The `#[wasm_bindgen]` attribute also supports passing closures as `&dyn Fn` or
`&mut dyn FnMut` trait object references. However, **this pattern is not unwind
safe** — if a panic occurs inside such a closure, it may corrupt program state
rather than being converted to a JavaScript exception.

```rust
// ⚠️ DEPRECATED: Not unwind safe
#[wasm_bindgen]
extern "C" {
    fn takes_closure(f: &dyn Fn());
    fn takes_mut_closure(f: &mut dyn FnMut());
}
```

### Migrating to `Closure::with`

Replace `&dyn Fn` / `&mut dyn FnMut` parameters with `&Closure<dyn Fn(...)>` and
use `Closure::with`:

```rust
// ❌ OLD: Not unwind safe
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
Closure::with(&mut |value| { /* ... */ }, |closure| {
    forEach(closure);
});
```

Note that `js-sys` currently still uses the `&dyn Fn` pattern for its callback
APIs (such as `Array::for_each`). These will be migrated to `Closure` in a
future release.
