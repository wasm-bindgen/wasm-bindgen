# `Option<T>`

`Option<T>` is accepted wherever the page for `T` lists `Option<T>` as
supported: as an argument or return value of exported and imported functions,
as a public field of an exported struct, and as a closure argument or return
value.

Going from Rust to JS, `None` is always `undefined`. Going from JS to Rust,
both `undefined` and `null` are `None` today, for every `T`.

## Strict `Option<T>` (unstable)

Setting `--cfg wasm_bindgen_unstable_strict_option` makes every JS to Rust
conversion of `Option<T>` treat only `undefined` as `None`. `null` is passed
to `T`'s own conversion, exactly as any other value that isn't a `T` would be.
Rust to JS is unchanged: `None` is still `undefined`.

The cfg applies to `js-sys` and `web-sys` as well, which still use `Option<T>`
for JS APIs that return `null`: under it, `RegExp::exec` with no match and
`Document::get_element_by_id` with a missing id return `Some(null)`, and
`Storage::get_item` on a missing key returns `Err` rather than `Ok(None)`.
Those bindings move to [`Nullable<T>`](./nullable.md) before the default
flips, so the cfg is only suitable for testing a crate's own boundary today.

The cfg must be set for the whole build, since the `wasm-bindgen` crate itself
records the choice when it describes the type, so it goes in `RUSTFLAGS` or in
`.cargo/config.toml` rather than on a single crate:

```toml
# .cargo/config.toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg=wasm_bindgen_unstable_strict_option"]
```

What `null` then does depends on `T`, and in some cases on whether the glue is
generated with debug assertions (`--debug`, the default for
`wasm-bindgen-test`) or without them:

| `T` | `null` with debug assertions | `null` without |
|---|---|---|
| integers, floats, numeric enums, raw pointers | throws, as for any non-number | reaches Rust as `0`, so `Some(0)` or the variant with discriminant `0` |
| `NonNull<T>` | throws, as for any non-number | reaches Rust as the null pointer, which is `None` |
| `i64`, `u64`, `i128`, `u128` | throws, as for a non-`BigInt` | throws a `TypeError` converting to `BigInt` |
| `bool` | throws, as for any non-boolean | reaches Rust as `Some(false)` |
| `char` | throws a `TypeError` | throws a `TypeError` |
| `String`, `&str` | throws, as for any non-string | throws a `TypeError` |
| slices and `Vec`s | throws, as for any non-array | throws a `TypeError` |
| string enums | the invalid variant, as for any unknown string | the same |
| exported structs | throws, as for any non-instance | the same |
| `JsValue` and imported JS types | `Some(null)` | `Some(null)` |

Where the table says a value reaches Rust, that is what a plain `T` argument
receives for `null` today: the glue adds no release-mode checks that `T`
itself doesn't have.

With the cfg set, an exported function's optional argument is typed
`T | undefined` in TypeScript, or `arg?: T` when it's trailing, and an optional
struct field is `field?: T`. Without it they include `null`.

This behaviour becomes the default in the next breaking release. Code that
relies on `null` becoming `None` should move to [`Nullable<T>`](./nullable.md)
for a `T | null` value, or `Option<Nullable<T>>` for `T | null | undefined`.
The `Option` layer of `Option<Nullable<T>>` is strict with or without the cfg,
since `null` has to reach the `Nullable` layer.
