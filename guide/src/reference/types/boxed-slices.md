# `Box<[T]>` and `Vec<T>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `Array` object |

You can pass boxed slices and `Vec`s to and from JS of any type that can be
passed on its own, including:

- `JsValue`s.
- Imported JavaScript types.
- Exported Rust types.
- C-style exported enums and string enums.
- `String`s.
- `bool`s and `char`s.
- `&str`s and borrowed slices of numbers (`&[u8]`), to JS only. Each slice
  reaches JS as a copy.
- `Option<T>` and `Nullable<T>` of any of these, and of numbers.
- Other `Vec`s.

Imported JS types and `JsValue`s are passed as their handles. Every other
element is converted the same way a single value of that type is. A `String`
or an exported Rust type is checked as it comes into Rust, so an element of
the wrong type throws.

Borrowed slices (`&[T]`) can be passed to imported JS functions for every
element type that can be passed by reference on its own, which exported Rust
types can't be; see
[`slice` and `vec`](../attributes/on-js-imports/slice.md).

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
