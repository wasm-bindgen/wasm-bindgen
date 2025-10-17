# `JsValue`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | No | No | Any JavaScript value |

## Using `JsValue`

`JsValue` represents an unknown JS value in Rust. For referencing a known JS value in Rust, see [`JsRef`](jsref.md), which `JsValue` is actually an anytype variant of (`JsValue = JsRef<AnyType>`).

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
