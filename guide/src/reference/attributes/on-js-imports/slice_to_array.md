# `slice_to_array`

By default, an `&[T]` argument to an imported JS function uses one of two
representations on the JS side:

* For primitive numeric `T` (`u8`, `i32`, `f64`, ...) the slice arrives as a
  zero-copy typed-array *view* into linear memory (e.g. `Uint8Array`,
  `Float64Array`).
* For `String`, an imported JS type, or another `JsValue`-shaped element
  type, the slice is materialised as a plain `Array` of values.

The `slice_to_array` attribute makes every `&[T]` (and `Option<&[T]>`)
argument of an imported function arrive as a plain JS `Array` regardless
of the element kind. The user-facing Rust signature is unchanged —
`&[T]` stays `&[T]`. Only the wire format and the JS-visible type
change.

This is useful when binding JS APIs that expect a plain `T[]` (e.g.
`Array<number>`) rather than a typed array.

## Per-function

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives `Array<number>` rather than `Uint16Array`.
    #[wasm_bindgen(slice_to_array)]
    fn set_indices(values: &[u16]);
}
```

## Per `extern "C"` block

The attribute can also be written on the block to apply to every
imported function inside:

```rust
#[wasm_bindgen(module = "/lib.js", slice_to_array)]
extern "C" {
    fn take_numbers(v: &[i32]);
    fn take_strings(v: &[String]);
    fn take_optional(v: Option<&[u16]>);
}
```

Per-function and per-arg `slice_to_array` are additive — the attribute
is opt-in at any level.

## Wire format

For primitive element kinds the wire is the *same* zero-copy borrow of
the slice memory used by plain `&[T]`; the only difference is that the
JS-side shim wraps the typed-array view in `Array.from(...)` to
materialise a plain `Array`. No allocation, no copy on the Rust side.

For `String`, `JsValue`, and JS-imported types the Rust side builds a
freshly allocated `[u32]` buffer of externref indices — one per element
— that JS reads into a plain `Array` and then frees. Per-element
conversion is `&T -> JsValue`, which for handle-shaped types is a
refcount bump on the existing JS slot, and for `String` allocates a
fresh JS string.

## What this is not

* It does **not** apply to exported functions. Only outgoing arguments
  (Rust calling JS) are rewritten.
* It does **not** support exported Rust struct types as the element
  type — `&[ExportedT]` remains unsupported. Use `Vec<ExportedT>` to
  transfer ownership of a sequence of exported struct values to JS.
* It does **not** change the semantics of owned `Vec<T>`. Owned vectors
  passed by value continue to use their existing wire format.
* It does **not** affect the default `&[T]` (zero-copy typed-array view)
  behaviour for functions where `slice_to_array` was not opted into.
