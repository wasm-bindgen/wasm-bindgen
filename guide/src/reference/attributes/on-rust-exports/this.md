# `this`

Typically, `this` bindings are achieved via `impl` block definitions for methods.

But since all regular JavaScript functions may accept arbitrary `this` bindings, it is possible to support this as well for free functions.

The `#[wasm_bindgen(this)]` attribute can be applied to exported Rust functions to make them receive the JavaScript `this` value as their first parameter. This allows creating functions that can be called with `.call()` or `.apply()` to set the `this` context.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(this)]
pub fn get_property(this: &JsValue, property: &str) -> JsValue {
    js_sys::Reflect::get(this, &property.into()).unwrap()
}

#[wasm_bindgen(this)]
pub fn add_to_count(foo: &JsValue, value: u32) -> u32 {
    let current = js_sys::Reflect::get(foo, &"count".into())
        .unwrap()
        .as_f64()
        .unwrap() as u32;
    current + value
}
```

With the above called via `get_property.call({ str: 'foo' }, 'str')` or `add_to_count.apply({ count: 1 }, [7])` respectively.

Can only be used on functions, not methods or static methods within `impl` blocks, and the function must have at least one parameter.
