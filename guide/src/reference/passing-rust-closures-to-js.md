# Passing Rust Closures to Imported JavaScript Functions

The `ScopedClosure` (with static lifetime alias `Closure`) and `ImmediateClosure` type are the way to
pass Rust closures to JavaScript. It is defined in the `wasm_bindgen` crate and
exported in `wasm_bindgen::prelude`.

Closures are **unwind safe** by default: when built with `panic=unwind`, panics inside
closures are caught and converted to JavaScript `PanicError` exceptions. See
[Catching Panics](./catch-unwind.md) for details.

## Choosing a `Closure` API

| Use case | Import function signature | Accepts |
| -------- | ------------------------- | ------- |
| Immediate/synchronous callbacks | `&ImmediateClosure<C>` | `&ImmediateClosure` only |
| Known-lifetime callbacks | `&ScopedClosure<'lifetime, C>` | `&ScopedClosure<'a>`, `&ScopedClosure<'static>`, and `&From<ImmediateClosure>` by conversion |
| Indeterminate lifetime | `ScopedClosure<'static, C>` | `ScopedClosure<'static>` only |

While direct `&dyn Fn` and `&mut dyn FnMut` closures [are still supported](#legacy-dyn-fn-and-mut-dyn-fnmut), `&ImmediateClosure` is now recommended instead for unwind support.

### Constructor Patterns

| Type | Constructor | Aborting Constructor | Assert Unwind Safe |
| ---- | ----------- | -------------------- | ------------------ |
| [`&ImmediateClosure<C>`](#immediatesynchronous-callbacks-with-immediateclosure) | `ImmediateClosure::new` / `new_immutable` | `wrap_aborting` / `wrap_immutable_aborting` | `wrap_assert_unwind_safe` / `wrap_immutable_assert_unwind_safe` |
| [`&ScopedClosure<'a, C>`](#non-static-lifetimes-pass-by-reference-only) | `Closure::borrow` / `borrow_mut` | `borrow_aborting` / `borrow_mut_aborting` | use `AssertUnwindSafe` wrapper |
| [`ScopedClosure<'static, C>`](#static-lifetimes-with-closuret--scopedclosurestatic-t) | `Closure::own` (`Closure::new`) | `own_aborting` | use `AssertUnwindSafe` wrapper |
| [`ScopedClosure<'static, C>` (one-shot)](#one-shot-static-closures-with-scopedclosurestatic-tonce) | `Closure::once` | `Closure::once_aborting` | use `AssertUnwindSafe` wrapper |

`Closure<C>` is a backwards compatible alias for `ScopedClosure<'static, C>`, while providing constructors for arbitrary lifetimes.

The default constructors require `UnwindSafe` when building with `panic=unwind`, and catch panics, converting them to JavaScript `PanicError` exceptions.
The `_aborting` variants do NOT require `UnwindSafe` and do NOT catch panics—if the closure panics, the process will abort.
See [Catching Panics](./catch-unwind.md) for details.

The `_assert_unwind_safe` variants catch panics but don't require `MaybeUnwindSafe`, enabling type inference with inline closures while still catching panics. Use these when you need inference and are confident the closure is unwind-safe.

Alternatively, you can wrap your closure with `std::panic::AssertUnwindSafe` and use the regular constructors (`new`, `own`, `borrow`, `borrow_mut`). This is useful when you want to keep using the coercion-based constructors:

```rust
use std::panic::AssertUnwindSafe;
use wasm_bindgen::prelude::*;

let data = Rc::new(RefCell::new(0));
let closure = Closure::own(AssertUnwindSafe(move || {
    *data.borrow_mut() += 1;
}));
```

This constructor flexibility allows API consumers to decide on unwind safety behavior at the call site, rather than having it fixed by the function signature. A single function accepting `&ImmediateClosure<dyn FnMut(u32)>` can be called with closures created via `new` (verified unwind-safe), `wrap_assert_unwind_safe` (asserted unwind-safe with inference), or `wrap_aborting` (aborts on panic).

## Immediate/Synchronous Callbacks with `ImmediateClosure`

Use `ImmediateClosure` for callbacks that JavaScript calls immediately and does
not retain, such as `Array.forEach`, `Array.map`, sorting comparators, and
similar synchronous APIs. This is the recommended lightweight option with the same ABI
as `&dyn FnMut`, while providing unwind safety.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn forEach<'a>(f: &ImmediateClosure<'a, dyn FnMut(u32) + 'a>);
}

let mut sum = 0;
forEach(&ImmediateClosure::new(&mut |x| {
    sum += x;
}));
```

Type inference works automatically—no need to annotate closure parameter types
when the target type is known from context.

Use `ImmediateClosure::new` for `FnMut` closures (common case) or
`ImmediateClosure::new_immutable` for `Fn` closures.

### Lifetime Bounds in Extern Declarations

When declaring imported JavaScript functions that take `ImmediateClosure`, always
add the `+ 'a` lifetime bound to the trait object. Without it, the trait object
defaults to `+ 'static`, which prevents closures from borrowing local variables:

```rust
#[wasm_bindgen]
extern "C" {
    // ✓ Correct: trait object lifetime tied to ImmediateClosure lifetime
    fn forEach<'a>(f: &ImmediateClosure<'a, dyn FnMut(u32) + 'a>);
    
    // ✗ Wrong: missing + 'a defaults to 'static, rejecting borrowed closures
    fn forEach_bad<'a>(f: &ImmediateClosure<'a, dyn FnMut(u32)>);
}
```

> **Note:** This lifetime annotation is specific to `ImmediateClosure`. For
> `ScopedClosure`, the dyn type is a phantom type that gets erased, so the
> direct `ScopedClosure<'a, dyn FnMut(u32)>` signature works directly.

### Aborting and Assert Unwind Safe Variants

By default `ImmediateClosure::new` enforces unwind safety via `MaybeUnwindSafe`.
When you need to capture types that aren't `UnwindSafe` (like `Rc<RefCell<T>>`),
you have two options:

1. **`wrap_aborting` / `wrap_immutable_aborting`** — Aborts on panic instead of catching. Use when you prefer abort-on-panic behavior.

2. **`wrap_assert_unwind_safe` / `wrap_immutable_assert_unwind_safe`** — Catches panics but doesn't verify `MaybeUnwindSafe`. Use when you want panic catching and are confident the closure is unwind-safe.

```rust
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    fn forEach<'a>(f: &ImmediateClosure<'a, dyn FnMut(u32) + 'a>);
}

// RefCell is not UnwindSafe, but these variants don't require it
let data = Rc::new(RefCell::new(0));

// Option 1: Abort on panic
forEach(&ImmediateClosure::wrap_aborting(&mut |x| {
    *data.borrow_mut() += x;
}));

// Option 2: Catch panics (caller asserts unwind safety)
forEach(&ImmediateClosure::wrap_assert_unwind_safe(&mut |x| {
    *data.borrow_mut() += x;
}));
```

These variants also enable type inference from the expected type, since they
take the dyn type directly.

## Known-Lifetime Callbacks with `ScopedClosure`

For longer lived closures, use `ScopedClosure`, which operates in two separate modes
depending on whether it uses a static lifetime or a known lifetime.

When typing a JS function from Rust taking a closure argument there are two modes of operation:

1. **`&ScopedClosure<'a, T>` (pass by ref)**: The closure is borrowed out to JS, while retaining
   ownership in Rust. There is no JS GC finalizer integration. It is disposed by Rust when
   dropped. Works with any lifetime.

2. **`ScopedClosure<'static, T>` by value (pass by value)**: The closure is passed to JS
   ownership and integrated with JS GC finalizers. It is disposed entirely by JS. Only
   works with `'static` closures.

Note that `ScopedClosure<'static, T>` can also be passed by reference (`&ScopedClosure<'static, T>`)
if you want Rust to retain ownership of a static closure.

### Non-Static Lifetimes (Pass by Reference Only)

For creating a `ScopedClosure<'a, T>` from a non-static lifetime, use `Closure::borrow` (for `Fn`)
or `Closure::borrow_mut` (for `FnMut`) when JavaScript may store the callback temporarily but
you control when it becomes invalid. The closure is invalidated when the `ScopedClosure` is dropped.

**Non-static closures can only be passed by reference** since the underlying closure data
may live on the stack.

These are unwind safe. For non-unwind-safe closures, use `Closure::borrow_aborting` and
`Closure::borrow_mut_aborting` (aborts on panic), or wrap your closure with `AssertUnwindSafe`
to assert unwind safety while still catching panics.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // (Using the AsUpcast trait, accept any closure which can upcast under JS upcast rules)
    fn register_callback(cb: impl AsUpcast<&ScopedClosure<dyn FnMut(u32)>>);
    fn trigger_callbacks();
}

let mut result = 0;
{
    let mut func = |value: u32| {
        result += value;
    };
    let closure = Closure::borrow_mut(&mut func);
    register_callback(&closure);
    trigger_callbacks();  // Calls our closure
    // closure dropped here, invalidating the JS reference
}
```

The validity of the JavaScript function is tied to the lifetime of the `ScopedClosure`
in Rust. **Once a `ScopedClosure` is dropped, it will deallocate its internal memory
and invalidate the corresponding JavaScript function so that any further
attempts to invoke it raise an exception.**

If JavaScript calls the closure after the `ScopedClosure` is dropped, it will
throw: "closure invoked recursively or after being dropped".

### Static Lifetimes with `Closure<T> = ScopedClosure<'static, T>`

For `'static` closures, use `Closure::own()` when JavaScript needs to retain the
closure for an indeterminate period, such as for event listeners, timers, or
callbacks that outlive the current function call.

> Note: It is recommended in function signatures to use `ScopedClosure` and not
`Closure`, as it can accept both short-lived and static closures `Closure`. This
is because  `Closure<T>` is an alias for `ScopedClosure<'static, T>`. In a future
release, `Closure` will be directly aliased to `ScopedClosure` instead._

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

### Transferring Ownership to JavaScript

You can pass a `ScopedClosure` by value to transfer ownership to JavaScript. This is
useful for one-shot callbacks where you don't need to retain a handle in Rust:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn set_one_shot_callback(cb: ScopedClosure<dyn FnMut()>);
}

let cb = Closure::own(|| {
    // This closure must be 'static
});
set_one_shot_callback(cb);  // Ownership transferred to JS GC, no need to store or forget
```

Note that only `'static` closures (`ScopedClosure<'static, T>`) can be passed
by value. Borrowed closures must be passed by reference.

## One-Shot Static Closures with `ScopedClosure<'static, T>::once`

Use `Closure::once` (or `Closure::once` alias) for closures that should only be
called once, such as Promise handlers. This allows using `FnOnce` closures that
consume captured values.

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

## Legacy `&dyn Fn` and `&mut dyn FnMut`

> Raw `&dyn Fn` and `&mut dyn FnMut` may be deprecated in a future release,
> use `ImmediateClosure` instead, or via `wrap` if you do not need
> unwind safety.

The `#[wasm_bindgen]` attribute also supports passing closures as `&dyn Fn` or
`&mut dyn FnMut` trait object references directly. However, this pattern is
deprecated because:

1. **No unwind safety**: If a panic occurs, the process will abort
2. **Confusing semantics**: The `dyn` syntax suggests heap allocation but these are stack-borrowed

Prefer `ImmediateClosure` for all new code:

```rust
// Deprecated:
#[wasm_bindgen]
extern "C" {
    fn takes_closure(f: &dyn Fn());
    fn takes_mut_closure(f: &mut dyn FnMut());
}

// Preferred:
#[wasm_bindgen]
extern "C" {
    fn takes_closure(f: &ImmediateClosure<dyn Fn()>);
    fn takes_mut_closure(f: &ImmediateClosure<dyn FnMut()>);
}
```

`ImmediateClosure` can then support taking both unwind safe and non-unwind safe variants,
with the usage being consumer defined.

## Panic Handling

When built with `panic=unwind`, all `ScopedClosure` and `ImmediateClosure` variants
catch panics and convert them to JavaScript `PanicError` exceptions. This requires
the closure to satisfy Rust's `UnwindSafe` trait.

For more information on enabling panic catching, see [Catching
Panics](./catch-unwind.md).

### UnwindSafe Requirement

The closure constructors for `ImmediateClosure` and `ScopedClosure` all require that
closures be `UnwindSafe`. They act as marker traits that indicates a type is safe to
use across panic boundaries.

Common "not unwind safe" compiler errors are caused by capturing types with
interior mutability:

- `Rc<Cell<_>>`, `Rc<RefCell<_>>`
- Other interior mutability types

The compiler error will indicate which captured type is problematic.

#### Fix 1: Use aborting variants

If you don't need panic catching, use the `*_aborting` variants (`own_aborting`,
`once_aborting`, `Closure::borrow_aborting`, `Closure::borrow_mut_aborting`,
`ImmediateClosure::wrap_aborting`) which do not require `UnwindSafe`:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

let data = Rc::new(RefCell::new(0));

// No UnwindSafe requirement — aborts on panic instead of catching
let closure = Closure::own_aborting(move || {
    *data.borrow_mut() += 1;
});
```

#### Fix 1b: Use wrap_assert_unwind_safe

For `ScopedClosure`, you can use `Closure::wrap_assert_unwind_safe` with a boxed closure:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

let data = Rc::new(RefCell::new(0));

// No UnwindSafe requirement — catches panics, caller asserts safety
let closure: Closure<dyn FnMut()> = Closure::wrap_assert_unwind_safe(Box::new(move || {
    *data.borrow_mut() += 1;
}));
```

For `ImmediateClosure`, use `wrap_assert_unwind_safe` directly:

```rust
let data = Rc::new(RefCell::new(0));
forEach(&ImmediateClosure::wrap_assert_unwind_safe(&mut |x| {
    *data.borrow_mut() += x;
}));
```

#### Fix 2: Assert unwind safety

If you need panic catching and are confident your closure is safe to use across
panic boundaries, you can use `AssertUnwindSafe`:

```rust
use std::panic::AssertUnwindSafe;
use wasm_bindgen::prelude::*;

let closure = Closure::new(AssertUnwindSafe(move || {
    // you're asserting this is safe across panics
}));
```

## Converting Closures to Typed Functions

The `js_sys::Function::from_closure()` and `js_sys::Function::from_immutable_closure()` methods provide type-safe conversion from `Closure` to typed `Function` with comprehensive covariance support.

Conversion by value is only supported for `'static` owned closures. For borrowed closures, `Function::from_closure_ref()` and `Function::from_immutable_closure_ref()` can be used.

```rust
use js_sys::{Function, Number, JsString};
use wasm_bindgen::prelude::*;

// Owned static conversion cases
let closure: Closure<dyn FnMut() -> u32> = Closure::new(|| 42);
let func: Function<fn() -> Number> = Function::from_closure(closure);
// String types convert to JsString
let str_closure: Closure<dyn Fn() -> String> = Closure::new(|| "hello".to_string());
let str_func: Function<fn() -> JsString> = Function::from_immutable_closure(str_closure);

// Borrowed ScopedClosure conversion via ref
let mut val: u32 = 5;
let func = || val;
let closure = ScopedClosure::borrow(&func);
let func: &Function<fn() -> Number> = Function::from_immutable_closure_ref(&closure);
```
