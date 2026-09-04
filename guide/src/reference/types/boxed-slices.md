# `Box<[T]>` and `Vec<T>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `Array` object |

You can pass boxed slices and `Vec`s of several different types to and from JS:

- `JsValue`s.
- Imported JavaScript types.
- Exported Rust types.
- C-style exported enums.
- `String`s.

[You can also pass boxed slices of numbers to JS](boxed-number-slices.html),
except that they're converted to typed arrays (`Uint8Array`, `Int32Array`, etc.)
instead of regular arrays.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/boxed_js_value_slice.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/boxed_js_value_slice.js}}
```
