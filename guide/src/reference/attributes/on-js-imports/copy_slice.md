# `copy_slice`

By default, an `&[T]` argument to an imported JS function with a primitive
numeric element type (`u8`, `i32`, `f64`, ...) arrives as a zero-copy
typed-array *view* into wasm linear memory (e.g. `Uint8Array`,
`Float64Array`). The view is only valid for the duration of the call: it
dangles if the callee retains it, and detaches if wasm memory grows during
the call (any allocation in a re-entrant callback).

The `copy_slice` attribute makes every `&[T]` (and `Option<&[T]>`) argument
of an imported function arrive as a *fresh typed array* that JS owns
outright. The user-facing Rust signature is unchanged — `&[T]` stays
`&[T]` — and so is the wire format (the borrowed slice; no Rust-side
allocation). Only the JS glue changes, copying the view before the call:

```js
var v0 = getArrayU8FromWasm0(ptr0, len0).slice();
```

After the call returns, the wasm buffer and the JS array are unrelated.

This is useful when binding JS APIs that retain the buffer past the call or
that are typed `Uint8Array` / `BufferSource` — for which
[`slice_to_array`](slice_to_array.md)'s plain `Array` would be rejected with
a `TypeError`.

## Per-function, per-argument, per `extern "C"` block

`copy_slice` is accepted everywhere `slice_to_array` is:

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives a fresh `Uint8Array` it may retain.
    #[wasm_bindgen(copy_slice)]
    fn store_bytes(v: &[u8]);
}

#[wasm_bindgen(module = "/lib.js", copy_slice)]
extern "C" {
    fn take_bytes(v: &[u8]);
    fn take_floats(v: Option<&[f64]>);
}
```

A more specific level *overrides* a broader one: a per-fn attribute
overrides the block default, and a per-arg attribute overrides both. This
makes mixed shapes expressible under one block-level default:

```rust
#[wasm_bindgen(module = "/lib.js", copy_slice)]
extern "C" {
    fn mixed(copied: &[u8], #[wasm_bindgen(slice_to_array)] as_array: &[u8]);
}
```

Writing both `slice_to_array` and `copy_slice` at the *same* level is a
compile error.

## The three-way choice on primitive slices

| attribute | JS receives | cost |
|---|---|---|
| *(none)* | typed-array view of wasm memory | zero-copy; caller asserts no retention / no re-entry |
| `slice_to_array` | fresh plain `Array` | one copy |
| `copy_slice` | fresh typed array | one copy |

## What this is not

* It does **not** apply to exported functions. Only outgoing arguments
  (Rust calling JS) are affected.
* It does **not** change non-slice arguments (a method's `this` receiver,
  `Vec<T>`, scalars): the attribute is a silent no-op there, so it is safe
  on a whole `extern "C"` block of mixed-shape imports.
* It does **not** change slices whose element kind has no typed-array form.
  `&[JsString]`, `&[JsValue]` and other handle-shaped elements already reach
  JS as a fresh `Array` of shared handles, so the attribute is a no-op
  there. (`&[String]` has no borrowed ABI at all without `slice_to_array`,
  with or without `copy_slice`.)
* It does **not** reject a generic element type. `&[T]` for a type
  parameter `T` resolves per concrete instantiation under
  [`experimental_generic_mono`](experimental_generic_mono.md) (a typed copy
  at `T = u8`, a no-op at `T = JsString`), and is a no-op on the
  type-erasure generic path, where the slice erases to the handle tier.
  The same now holds for `slice_to_array`.
* It **cannot** be combined with a `&mut` slice, and the combination is
  rejected at compile time: JS receives an owned copy, so its writes would
  be silently discarded rather than written back through the `&mut`. Use a
  shared slice if JS only needs to read the elements, or drop `copy_slice`
  for that argument to keep the writable view.
