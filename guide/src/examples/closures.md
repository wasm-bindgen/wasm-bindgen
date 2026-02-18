# web-sys: Closures

[View full source code][code] or [view the compiled example online][online]

[online]: https://wasm-bindgen.github.io/wasm-bindgen/exbuild/closures/
[code]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/closures

The `Closure` type allows passing Rust closures to JavaScript. This example
demonstrates different `Closure` APIs for various use cases.

## Choosing a `Closure` API

- **`ImmediateClosure::new`** / **`ImmediateClosure::new_mut`** — For
  immediate/synchronous callbacks where JavaScript calls the closure right away
  and doesn't retain it. Lightweight with no JS wrapper overhead. `new` is for
  immutable `Fn` closures (easier to satisfy unwind safety), `new_mut` is for
  mutable `FnMut` closures.

- **`ScopedClosure::borrow`** / **`ScopedClosure::borrow_mut`** — For known-lifetime
  callbacks where JavaScript may briefly retain the closure but you control when
  it becomes invalid. `borrow` is for immutable `Fn`, `borrow_mut` is for mutable `FnMut`.

- **`Closure::new`** — For indeterminate-lifetime closures like event handlers or
  timers. The closure must be `'static` and you must manage its lifetime (store
  it somewhere or call `forget()`).

- **`Closure::once`** / **`Closure::once_into_js`** — For one-shot callbacks
  that will only be called once.

All closure types are unwind safe — panics are caught and converted to
JavaScript exceptions when built with `panic=unwind`. This requires the closure
to implement `UnwindSafe`. If your closure captures non-unwind-safe types (like
`Rc<RefCell<_>>`), use `AssertUnwindSafe` to wrap the closure, or use the
`*_aborting` variants which don't require `UnwindSafe`.

See [Passing Rust Closures to JS](../reference/passing-rust-closures-to-js.md)
for detailed documentation.

## `src/lib.rs`

```rust
{{#include ../../../examples/closures/src/lib.rs}}
```
