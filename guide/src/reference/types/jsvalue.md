# `JsValue`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | No | No | Any JavaScript value |

## Using `JsValue`

`JsValue` is the fundamental type for representing an arbitrary JavaScript value in Rust.

You can create `JsValue` instances using various `from_*` methods or by converting from Rust types. The `JsValue` type provides methods to check the type of the underlying value (like `is_string()`, `is_object()`, `is_null()`) and to convert to specific types. When you need to work with more specific JavaScript types, you can use the [`JsCast`](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/trait.JsCast.html) trait to perform checked or unchecked conversions to types like `JsString`, `Array`, or `Object`.

For accessing properties on untyped JavaScript values, see the documentation on [accessing properties of untyped JS values](../accessing-properties-of-untyped-js-values.html).

### Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/js_value.rs}}
```

### Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/js_value.js}}
```

## Typed `JsValue<T>` Variant

`JsValue` supports an optional generic type parameter `JsValue<T>` that provides compile-time type information while maintaining the same runtime representation. This is useful when you want to track what type of JavaScript value you're working with at the Rust type level, while retaining ABI semantics of JsValue (which is after all just an `externref` in Wasm).

To create a typed `JsValue<T>`, use `JsValue::new()` or `JsValue::from_typed()` with any value that can be converted to `JsValue`. On creation, Rust values will be converted into JS values where necessary. To convert back into a Rust value, use the `try_unwrap()` or `unwrap()` method, which also performs runtime validation to ensure the type matches. `cast_unchecked()` may also be used for zero-cost conversions between `JsValue<T>` types when you're certain of the underlying type, or use the `JsCast` trait's `dyn_into()` method for runtime-checked conversions.

Typed `JsValue<T>` can be converted back into `JsValue` via `upcast()`. `cast_unchecked()` may be used to convert between `JsValue<T>` and `JsValue<U>`.

### Example: Creating and Using Typed Values

```rust
use wasm_bindgen::prelude::*;
use js_sys::JsString;

// Create typed values from any Into<JsValue>
let typed: JsValue<String> = JsValue::new(String::from("hello"));

// Extract the inner value with runtime checking
let inner = typed.try_unwrap()?;

// Convert to untyped JsValue
let typed: JsValue<JsString> = JsValue::new(JsString::from("world"));
let untyped: JsValue = js_typed.upcast();
```

### Example: Type Casting

```rust
use wasm_bindgen::prelude::*;
use js_sys::JsString;

// Unchecked cast (zero-cost, but unsafe if types don't match)
let untyped = JsValue::from_str("hello");
let typed: JsValue<String> = untyped.cast_unchecked();

// The typed value can be unwrapped without a type hint to get the JS value
let js_string = typed.unwrap();
```
