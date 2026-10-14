# Number Slices: `[u8]`, `[i8]`, `[u16]`, `[i16]`, `[u32]`, `[i32]`, `[u64]`, `[i64]`, `[f32]`, `[f64]`, `[MaybeUninit<u8>]`, `[MaybeUninit<i8>]`, `[MaybeUninit<u16>]`, `[MaybeUninit<i16>]`, `[MaybeUninit<u32>]`, `[MaybeUninit<i32>]`, `[MaybeUninit<u64>]`, `[MaybeUninit<i64>]`, `[MaybeUninit<f32>]`, and `[MaybeUninit<f64>]`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<&T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| No | Yes | Yes | No | No | No | A JavaScript `TypedArray` view of the Wasm memory for the boxed slice of the appropriate type (`Int32Array`, `Uint8Array`, etc) |

> **Note:** A `&T` parameter of an exported function also accepts a plain
> JavaScript array of numbers (or of `bigint`s for the 64-bit types), which is
> copied into the Wasm memory the same way as a `TypedArray`. A `&mut T`
> parameter of an exported function only accepts a `TypedArray`, since the
> results are copied back into its buffer.

> **Note:** Numeric `MaybeUninit<T>` can always be assumed to be initialized
> upon transmission from Rust to JS and vice-versa. However, uninitialized
> values coming from Rust might contain unspecified values.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/number_slices.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/number_slices.js}}
```
