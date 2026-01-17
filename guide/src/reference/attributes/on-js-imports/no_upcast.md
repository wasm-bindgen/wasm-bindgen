# `no_upcast`

The `no_upcast` attribute disables automatic generation of `Upcast` trait implementations for an imported type. 

By default, wasm-bindgen automatically generates the following `Upcast` implementations for imported types:
- `Upcast<JsValue>` - all types can upcast to JsValue
- `Upcast<Self>` - identity upcast for non-generic types
- Structural covariance for generic types (`Type<T>` → `Type<U>` when `T: Upcast<U>`)
- `Upcast<SuperClass>` for each type in the `extends` attribute

Use `no_upcast` when you need to provide custom `Upcast` implementations, for example when a type has special covariance rules.

```rust
use wasm_bindgen::convert::Upcast;

#[wasm_bindgen]
extern "C" {
    // Automatic Upcast implementations are generated
    #[wasm_bindgen(extends = Object)]
    type NormalType;

    // No automatic Upcast implementations - must be provided manually
    #[wasm_bindgen(extends = Object, no_upcast)]
    type CustomType<T>;
}

// Provide custom Upcast implementations for CustomType
impl<T> Upcast<JsValue> for CustomType<T> {}
impl<T> Upcast<Object> for CustomType<T> {}
// ... additional custom implementations
```

This is useful for types that have:
- Complex generic covariance rules (e.g., contravariant type parameters)
- Custom inheritance hierarchies
- Special relationship with other types that require manual `Upcast` implementations
