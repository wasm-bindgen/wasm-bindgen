## Typed `JsRef<T>` Variant

`JsRef<T>` is a generic variant of `JsValue` that provides compile-time type information while maintaining the same runtime representation. This is useful when you want to track what type of JavaScript value you're working with at the Rust type level, while retaining ABI semantics of JsValue (which is after all just an `externref` in Wasm). Note that `JsValue` is a type alias for `JsRef`'s default any type, so that all functions accepting `JsRef<T>` that are generic on `T` can also accept `JsValue`.

### Creating Typed Values

- **`JsRef::to_js(value)`**: Create from a value of type `T`. Requires the value to be exactly type `T` and implements `Into<JsValue>`.
- **`JsRef::to_js_as(value)`**: Create from a value that implements `Into<T>`. Useful when you want to convert through an intermediate type (e.g., `&str` → `JsString` → `JsRef<JsString>`).
- **`value.to_js()`** (via `ToJs` trait): Convenience method available on any type that implements `Into<JsValue>`.

### Extracting Values

To convert back into a Rust value:

- **`try_from_js()`**: Performs runtime type validation and returns `Result<T, Error>`. Use this when you need to verify the type is correct.
- **`from_js()`**: Extracts the value via `wbg_cast`. Performs validation in debug mode but may skip checks in release mode for performance.

### Type Conversions

- **`into_value()`**: Convert `JsRef<T>` to untyped `JsValue` (consumes self, zero-cost)
- **`as_value()`**: Borrow `&JsRef<T>` as untyped `&JsValue` (zero-cost)
- **`to_value()`**: Clone `&JsRef<T>` into untyped `JsValue` (zero-cost)
- **`cast_unchecked()`**: Convert between `JsRef<T>` and `JsRef<U>` without validation (zero-cost)
- **`dyn_into()`** (via `JsCast` trait): Runtime-checked conversion to JS types

### Example: Creating and Using Typed Values

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsRef;
use js_sys::JsString;

// Create typed value - value must be exact type
let typed: JsRef<String> = JsRef::to_js(String::from("hello"));

// Create typed value with automatic conversion
let typed: JsRef<JsString> = JsRef::to_js_as("hello");  // &str -> JsString -> JsRef<JsString>

// Or use the convenience trait method
let typed = "hello".to_js();  // JsRef<&str>

// Extract with runtime type checking
let inner: String = typed.try_from_js()?;

// Extract with wbg_cast (faster, checked in debug mode)
let inner: String = typed.from_js();

// Convert to untyped JsValue
let typed: JsRef<JsString> = JsRef::to_js(JsString::from("world"));
let untyped: JsValue = typed.into_value();
```

### Example: Type Casting

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsRef;
use js_sys::JsString;

// Unchecked cast from JsValue to JsRef<T> (zero-cost, no validation)
let untyped = JsValue::from_str("hello");
let typed: JsRef<String> = untyped.cast_unchecked();

// Extract with wbg_cast - checked in debug mode
let string: String = typed.from_js();

// Safe alternative with guaranteed runtime checking
let untyped = JsValue::from_str("world");
let typed: JsRef<String> = untyped.cast_unchecked();
let string: String = typed.try_from_js()?;  // Returns Result
```
