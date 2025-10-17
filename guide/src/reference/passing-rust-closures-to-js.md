# Passing Rust Closures to Imported JavaScript Functions

The `#[wasm_bindgen]` attribute supports Rust closures being passed to
JavaScript in two variants:

1. Stack-lifetime closures that should not be invoked by JavaScript again after
   the imported JavaScript function that the closure was passed to returns.

2. Heap-allocated closures that can be invoked any number of times, but must be
   explicitly deallocated when finished.

## Stack-Lifetime Closures

Closures with a stack lifetime are passed to JavaScript as either `&dyn Fn` or `&mut
dyn FnMut` trait objects:

```rust
// Import JS functions that take closures

#[wasm_bindgen]
extern "C" {
    fn takes_immutable_closure(f: &dyn Fn());

    fn takes_mutable_closure(f: &mut dyn FnMut());
}

// Usage

takes_immutable_closure(&|| {
    // ...
});

let mut times_called = 0;
takes_mutable_closure(&mut || {
    times_called += 1;
});
```

**Once these imported functions return, the closures that were given to them
will become invalidated, and any future attempts to call those closures from
JavaScript will raise an exception.**

Closures also support arguments and return values like exports do, for example:

```rust
#[wasm_bindgen]
extern "C" {
    fn takes_closure_that_takes_int_and_returns_string(x: &dyn Fn(u32) -> String);
}

takes_closure_that_takes_int_and_returns_string(&|x: u32| -> String {
    format!("x is {}", x)
});
```

## Heap-Allocated Closures

Sometimes the discipline of stack-lifetime closures is not desired. For example,
you'd like to schedule a closure to be run on the next turn of the event loop in
JavaScript through `setTimeout`. For this, you want the imported function to
return but the JavaScript closure still needs to be valid!

For this scenario, you need the `Closure` type, which is defined in the
`wasm_bindgen` crate, exported in `wasm_bindgen::prelude`, and represents a
"long lived" closure.

The validity of the JavaScript closure is tied to the lifetime of the `Closure`
in Rust. **Once a `Closure` is dropped, it will deallocate its internal memory
and invalidate the corresponding JavaScript function so that any further
attempts to invoke it raise an exception.**

Like stack closures a `Closure` supports both `Fn` and `FnMut` closures, as well
as arguments and returns.

```rust
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

### Panic Handling in Closures

When built with `panic=unwind`, the default `Closure` constructors (`new`,
`wrap`, and `once`) catch panics that occur when the closure is invoked from
JavaScript. Panics are converted to JavaScript `PanicError` exceptions, allowing
JavaScript code to catch and handle them.

These constructors require the closure to satisfy Rust's `UnwindSafe` trait.

For more information on enabling panic catching, see [Catching
Panics](./catch-unwind.md).

### UnwindSafe Requirement

The default constructors (`new`, `wrap`, `once`) require that closures be
`UnwindSafe`. This is a marker trait that indicates a type is safe to use across
panic boundaries.

Common "not unwind safe" compiler errors are caused by capturing types with
interior mutability:

- `Rc<Cell<_>>`, `Rc<RefCell<_>>`
- Other interior mutability types

The compiler error will indicate which captured type is problematic.

**Fix 1: Use abort variants**

If you don't need panic catching, use the `*_aborting` variants (`new_aborting`,
`wrap_aborting`, `once_aborting`, `once_into_js_aborting`) which do not require
`UnwindSafe`:

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

## Converting Closures to Typed Functions

The `js_sys::Function::from_closure()` method provides type-safe conversion from `Closure` to typed `Function` with comprehensive covariance support:

```rust
use js_sys::{Function, Number, JsString};
use wasm_bindgen::prelude::*;

// Rust primitives automatically convert to JS types (casting covariance)
let closure: Closure<dyn Fn() -> u32> = Closure::new(|| 42);
let func: Function<Number> = Function::from_closure(closure);

// String types convert to JsString
let str_closure: Closure<dyn Fn() -> String> = Closure::new(|| "hello".to_string());
let str_func: Function<JsString> = Function::from_closure(str_closure);

// Generic covariance also applies
let js_closure: Closure<dyn Fn() -> Number> = Closure::new(|| Number::from(42));
let typed_func: Function<Number> = Function::from_closure(js_closure);
let general_func: Function<JsValue> = typed_func.upcast();
```

This supports two forms of covariance:

1. **Casting covariance**: Rust primitives → JS types
   - Numeric primitives (`u32`, `i32`, `f64`, etc.) → `Number`
   - String types (`String`, `&str`, `char`) → `JsString`

2. **Generic covariance**: Typed JS values → wider JS types
   - `Function<Number>` → `Function<JsValue>`
   - `Function<T>` → `Function<U>` when `T: Upcast<U>`

Both can be combined: `Closure<dyn Fn() -> u32>` → `Function<Number>` → `Function<JsValue>`
