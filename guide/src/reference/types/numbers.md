# Numbers: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `isize`, `usize`, `f32`, and `f64`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript number or bigint value |

[JavaScript `Number`s](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number#number_encoding) are 64-bit floating point value under the hood and cannot accurately represent all of Rust's numeric types. `wasm-bindgen` will automatically use either [`BigInt`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) or `Number` to accurately represent Rust's numeric types in JavaScript:

- `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `isize`, `usize`, `f32`, and `f64` will be represented as `Number` in JavaScript.
- `u64`, `i64`, `u128`, and `i128` will be represented as `BigInt` in JavaScript. To pass a `u64` or `i64` as a `Number` instead, use [`SafeInteger<T>`](./safe-integer.md).

> **Note**: Wasm is currently a 32-bit architecture, so `isize` and `usize` are 32-bit integers and "fit" into a JavaScript `Number`.

> **Note**: `u128` and `i128` require `wasm-bindgen` version 0.2.96 or later.

## Converting from JavaScript to Rust

`wasm-bindgen` will automatically handle the conversion of JavaScript numbers to Rust numeric types. The conversion rules are as follows:

### `Number` to `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `isize`, and `usize`

If the JavaScript number is `Infinity`, `-Infinity`, or `NaN`, then the Rust value will be 0. Otherwise, the JavaScript number will rounded towards zero (see [`Math.trunc`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc) or [`f64::trunc`](https://doc.rust-lang.org/std/primitive.f64.html#method.trunc)). If the rounded number is too large or too small for the target integer type, it will wrap around.

For example, if the target type is `i8`, Rust will see the following values for the following inputs:

| JS input number | Rust value (`i8`) |
| --------------: | :---------------- |
|              42 | 42                |
|             -42 | -42               |
|           1.999 | 1                 |
|          -1.999 | -1                |
|             127 | 127               |
|             128 | -128              |
|             255 | -1                |
|             256 | 0                 |
|              -0 | 0                 |
|     `±Infinity` | 0                 |
|           `NaN` | 0                 |

This is the same behavior as assigning the JavaScript `Number` to a [typed array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray) of the appropriate integer type in JavaScript, i.e. `new Uint8Array([value])[0]`.

Except for the handling of `Infinity` and `-Infinity`, this is the same behavior as [casting](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) `f64` to the appropriate integer type in Rust, i.e. `value_f64 as u32`.

### `BigInt` to `u64`, `i64`, `u128`, and `i128`

If the JavaScript `BigInt` is too large or too small for the target integer type, it will wrap around.

This is the same behavior as assigning the JavaScript `BigInt` to a [typed array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray) for 64-bit integer types in JavaScript, i.e. `new Int64Array([value])[0]`.

### `Number` to `f32`

The JavaScript `Number` is converted to a Rust `f32` using the same rules as [casting](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) `f64` to `f32` in Rust, i.e. `value_f64 as f32`.

This is the same behavior as [`Math.fround`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/fround) or assigning the JavaScript `Number` to a [`Float32Array`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array) in JavaScript, i.e. `new Float32Array([value])[0]`.

### `Number` to `f64`

Since JavaScript numbers are 64-bit floating point values, converting a JavaScript `Number` to a Rust `f64` is a no-op.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/numbers.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/numbers.js}}
```
