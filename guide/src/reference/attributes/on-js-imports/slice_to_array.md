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

Per-function and per-block `slice_to_array` combine additively — the
attribute is opt-in at either level. The mode only acts on `&[T]` /
`Option<&[T]>` arguments; any other argument shape (e.g. the `this`
argument of a method, or unrelated scalar arguments) is left
untouched, so it's safe to set the attribute on a method or on an
entire `extern "C"` block of mixed-shape imports.

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
  passed by value continue to use their existing wire format. Because
  `Vec<T>` is not slice-shaped, `slice_to_array` is a silent no-op on
  such an argument (as it is on any other non-slice argument, such as
  the `this` receiver of a method).
* It does **not** affect the default `&[T]` (zero-copy typed-array view)
  behaviour for functions where `slice_to_array` was not opted into.
* It does **not** work with a generic element type. `&[T]` for a type
  parameter `T` is rejected at compile time, because
  `VectorRefIntoWasmAbi` is implemented per concrete ABI shape and no
  bound the caller can write makes an arbitrary `T` satisfy it. This
  applies on both the type-erasure generic path and `experimental_generic_mono`.
  The element type must be concrete, e.g. `&[u16]`. A concrete element type
  in a generic function is fine — it is the element type that has to be
  concrete, not the function. Note that `slice_to_array` is inheritable
  from the enclosing `extern "C"` block, so a generic function in such a
  block must not take a `&[T]` argument.
* It **cannot** be combined with a `&mut` slice, and the combination is
  rejected at compile time. `slice_to_array` hands JS an owned `Array`
  copied out of linear memory, so there is nowhere for JS's writes to
  that `Array` to go — they would be discarded when the call returns,
  whereas a plain `&mut [T]` argument gives JS a typed-array *view*
  whose writes do land in the caller's buffer. Since that difference is
  invisible at runtime, it is an error rather than a silent no-op. Use
  `&[T]` if JS only needs to read the elements, or drop
  `slice_to_array` for that argument to keep the writable view. This
  matters most when `slice_to_array` is inherited from the enclosing
  `extern "C"` block, where the attribute is nowhere near the argument
  it would have applied to.
