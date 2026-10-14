# Boxed Number Slices: `Box<[u8]>`, `Box<[i8]>`, `Box<[u16]>`, `Box<[i16]>`, `Box<[u32]>`, `Box<[i32]>`, `Box<[u64]>`, `Box<[i64]>`, `Box<[f32]>`, `Box<[f64]>`, `Box<[MaybeUninit<u8>]>`, `Box<[MaybeUninit<i8>]>`, `Box<[MaybeUninit<u16>]>`, `Box<[MaybeUninit<i16>]>`, `Box<[MaybeUninit<u32>]>`, `Box<[MaybeUninit<i32>]>`, `Box<[MaybeUninit<u64>]>`, `Box<[MaybeUninit<i64>]>`, `Box<[MaybeUninit<f32>]>`, and `Box<[MaybeUninit<f64>]>`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript `TypedArray` of the appropriate type (`Int32Array`, `Uint8Array`, etc...) |

> **Note:** The contents of the slice are copied into a JavaScript [`TypedArray`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray)
from the Wasm linear memory when returning a boxed slice to JavaScript, and vice
versa when receiving a JavaScript `TypedArray` as a boxed slice in Rust.

> **Note:** `T` and `Option<T>` parameters also accept a plain JavaScript array
> of numbers (or of `bigint`s for the 64-bit types), which is copied into the
> Wasm linear memory the same way as a `TypedArray`.

> **Note:** Numeric `MaybeUninit<T>` can always be assumed to be initialized
> upon transmission from Rust to JS and vice-versa. However, uninitialized
> values coming from Rust might contain unspecified values.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/boxed_number_slices.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/boxed_number_slices.js}}
```
