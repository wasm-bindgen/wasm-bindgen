# `SafeInteger<T>`

A `u64` or `i64` is a `BigInt` in JavaScript, but many JS APIs type integer
counts as `number`. `SafeInteger<u64>` and `SafeInteger<i64>` keep the Rust
integer type while crossing the boundary as a `number`. In TypeScript they are
`number`.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type QueueMetrics;

    #[wasm_bindgen(method, getter, js_name = backlogCount)]
    fn backlog_count(this: &QueueMetrics) -> SafeInteger<u64>;
}

#[wasm_bindgen]
pub fn needs_attention(metrics: &QueueMetrics) -> bool {
    let count = metrics.backlog_count();
    count > 10_000 && count.is_power_of_two()
}
```

`SafeInteger<T>` derefs to `T`, compares with both `SafeInteger<T>` and `T`,
supports `+`, `-`, `*`, `/` and `%` (and unary `-` for `i64`) with either, and
converts to and from `T` with `From`.

The conversion is carried by the type itself, so it applies wherever the type
appears: arguments and return values of imports and exports, `catch`
results, `Option<T>` and `Nullable<T>`, `Vec<T>` and `Box<[T]>` (as a
`Float64Array`), `&[T]` arguments to exports, closure arguments and returns,
`async` import results, and `experimental_generic_mono` generic positions. A
plain `u64` in the same signature stays a `BigInt`.

## Range

A safe integer is within `±(2^53 - 1)` (`Number.MIN_SAFE_INTEGER` to
`Number.MAX_SAFE_INTEGER`) and, for `SafeInteger<u64>`, non-negative. Every
value in that range is exactly representable as a JS `number`.

The range is checked the way Rust checks integer overflow: only with debug
assertions enabled. In a debug build, constructing an out-of-range value with
`From`, arithmetic leaving the range, or receiving a `number` from JS that is
not an integer or is out of range panics. In a release build the conversion is
a plain `as` cast, so out-of-range values convert lossily, the same way a `u32`
argument wraps.

`SafeInteger::new(value)` returns `None` for an out-of-range value, and
`SafeInteger::try_from(js_value)` returns an error for a `JsValue` that is not
a safe integer, in every build.

| Rust | JS / TypeScript |
|---|---|
| `SafeInteger<u64>`, `SafeInteger<i64>` | `number` |
| `Option<SafeInteger<T>>` | `number \| undefined` |
| `Vec<SafeInteger<T>>`, `Box<[SafeInteger<T>]>` | `Float64Array` |

A borrowed `&[SafeInteger<T>]` cannot be passed from Rust to JS, since each
element has to be converted; pass a `Vec` or `Box<[_]>` instead.
