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

## Typed `JsVal<T>` Variant

`JsVal<T>` is a generic variant of `JsValue` that provides compile-time type information while maintaining the same runtime representation. This is useful when you want to track what type of JavaScript value you're working with at the Rust type level, while retaining ABI semantics of JsValue (which is after all just an `externref` in Wasm). Note that `JsValue` is a type alias for `JsVal<AnyType>`.

### Creating Typed Values

There are two ways to create a typed `JsVal<T>`:

- **`JsVal::wrap(value)`**: Create from a value of type `T`. Requires the value to be exactly type `T`.
- **`JsVal::new(value)`**: Create with a type annotation. Accepts any `Into<JsValue>`, useful when the input type differs from `T`.

### Extracting Values

To convert back into a Rust value:

- **`try_unwrap()`**: Performs runtime type validation and returns `Result<T, Error>`. Use this when you need to verify the type is correct.
- **`unwrap()`**: Performs an unchecked cast via `wbg_cast`. Fast but unsafe if the type doesn't match.

### Type Conversions

- **`upcast()`**: Convert `JsVal<T>` to untyped `JsValue` (zero-cost)
- **`cast_unchecked()`**: Convert between `JsVal<T>` and `JsVal<U>` without validation (zero-cost)
- **`dyn_into()`** (via `JsCast` trait): Runtime-checked conversion to JS types

### Example: Creating and Using Typed Values

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsVal;
use js_sys::JsString;

// Method 1: wrap() - value must be exact type
let typed: JsVal<String> = JsVal::wrap(String::from("hello"));

// Method 2: new() - requires type annotation, accepts Into<JsValue>
let typed: JsVal<String> = JsVal::new("hello");  // &str -> JsValue -> JsVal<String>

// Extract with runtime type checking
let inner: String = typed.try_unwrap()?;

// Convert to untyped JsValue
let typed: JsVal<JsString> = JsVal::wrap(JsString::from("world"));
let untyped: JsValue = typed.upcast();
```

### Example: Type Casting

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsVal;
use js_sys::JsString;

// Unchecked cast from JsValue to JsVal<T> (zero-cost, no validation)
let untyped = JsValue::from_str("hello");
let typed: JsVal<String> = untyped.cast_unchecked();

// Unchecked unwrap - fast but unsafe if type doesn't match
let string: String = typed.unwrap();

// Safe alternative with runtime checking
let typed: JsVal<String> = JsVal::new("hello");
let string: String = typed.try_unwrap()?;  // Returns Result
```
