# `Box<[T]>` and `Vec<T>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `Array` object |

You can pass boxed slices and `Vec`s of several different types to and from JS:

- `JsValue`s.
- Imported JavaScript types.
- Exported Rust types.
- C-style exported enums and string enums.
- `String`s.
- `bool`s and `char`s.
- `&str`s, to JS only.

Imported JS types and `JsValue`s are passed as their handles. `bool`s,
`char`s, string enums and `&str`s are converted element by element, the same
way a single value of that type is. `String`s, exported Rust types and
C-style enums are passed as JS values, and each is checked as it comes into
Rust, so an element of the wrong type throws.

Borrowed slices (`&[T]`) of the same element types, apart from exported Rust
types, can be passed to imported JS functions; see
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
