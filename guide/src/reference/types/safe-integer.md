# `SafeInteger<T>`

A `u64` or `i64` is a `BigInt` in JavaScript, but many JS APIs type integer
counts as `number`. `SafeInteger<u64>` and `SafeInteger<i64>` keep the Rust
integer type while crossing the boundary as a `number`. In TypeScript they are
`number`. `SafeU64` and `SafeI64` are aliases for the two.

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::SafeInteger;

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

A `number` received from JS is checked in every build. One that is not a safe
integer (a fraction, `NaN`, `Infinity`, out of range, or negative for `u64`)
throws a JS error at the boundary before any Rust code runs, whether it is an
argument to an export or a closure, or an element of a `Vec` or slice. A value
returned by a `catch` import, or resolved by a `catch` `async` import, is an
`Err` instead.

Values built in Rust are checked the way Rust checks integer overflow: only
with debug assertions enabled. In a debug build, constructing an out-of-range
value with `From` or arithmetic leaving the range panics. In a release build
an out-of-range value crosses to JS as the nearest `number`.

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
