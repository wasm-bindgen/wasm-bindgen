# `no_promising`

The `no_promising` attribute can be used to prevent automatic generation of the `Promising` trait implementation for an imported type.

This allows for custom thenable types (objects with a `then` method) when working with generic types in Wasm Bindgen where you need to specify what type the thenable resolves to.

## Promising Trait

The `Promising` trait is used by generic Promise methods like `promise.then_map` to determine the resolution type of chained operations.

By default, wasm-bindgen assumes `impl Promising for T { type Resolution = T }`, meaning `Promise.resolve(t)` resolves to the type itself. However, for Promise-like objects, `Promise.resolve(t)` does not return a Promise, but its inner type, so this trait specifies what they actually resolve to.

## Example: Custom Thenable Implementation

```rust
use wasm_bindgen::{Promising, prelude::*};

#[wasm_bindgen]
extern "C" {
    // Typing a custom thenable that resolves to Number
    #[wasm_bindgen(no_promising)]
    type NumericThenable;

    #[wasm_bindgen(method)]
    fn then(this: &NumericThenable, callback: &js_sys::Function) -> NumericThenable;
}

// Manually specify what this thenable resolves to via the Promising trait
impl Promising for NumericThenable {
    type Resolution = js_sys::Number;
}

// Now then_map can correctly infer types
fn example(thenable: NumericThenable) {
    // Normally Promise.resolve accepts a Promise<Number> or a Number:
    let promise: js_sys::Promise<Number> = js_sys::Promise::resolve(Number::from(5));

    let promise = js_sys::Promise::resolve(thenable); // Correctly reflects as a Promise<Number>
}
```

Without `no_promising` and the manual implementation, the type system wouldn't know that `NumericThenable` resolves to `Number`, breaking generic inference for Promise operations.
