# `slice`

The `slice` attribute selects how a `&[T]` (or `Option<&[T]>`) argument of an
imported function reaches JS. For primitive numeric element types (`u8`,
`i32`, `f64`, ...) there are three modes:

| mode | JS receives | cost |
|---|---|---|
| `slice = view` *(default)* | typed-array view of wasm memory | zero-copy; caller asserts no retention / no re-entry |
| `slice = array` | fresh plain `Array` | one copy |
| `slice = copy` | fresh typed array | one copy |

The user-facing Rust signature is unchanged by the mode — `&[T]` stays
`&[T]`.

`slice_to_array` is an alias of `slice = array`; see
[its page](slice_to_array.md) for the `array` mode's details, including
non-primitive element types (`&[String]`, `&[JsValue]`, imported JS types),
which only that mode changes.

## `slice = view`

The default: the slice arrives as a zero-copy typed-array view into wasm
linear memory (e.g. `Uint8Array`, `Float64Array`). The view is only valid for
the duration of the call: it dangles if the callee retains it, and detaches
if wasm memory grows during the call (any allocation in a re-entrant
callback). Writing the mode explicitly opts an argument back out of an
inherited `array` / `copy` default (see below).

## `slice = copy`

JS receives a *fresh typed array* it owns outright. The wire format is
unchanged (the borrowed slice; no Rust-side allocation) — only the JS glue
changes, copying the view before the call:

```js
var v0 = getArrayU8FromWasm0(ptr0, len0).slice();
```

After the call returns, the wasm buffer and the JS array are unrelated. This
is the mode for JS APIs that retain the buffer past the call or that are
typed `Uint8Array` / `BufferSource` — for which `slice = array`'s plain
`Array` would be rejected with a `TypeError`.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives a fresh `Uint8Array` it may retain.
    #[wasm_bindgen(slice = copy)]
    fn store_bytes(v: &[u8]);
}
```

## Per-argument, per-function, per `extern "C"` block

The mode can be written on an argument, on a function, or on an
`extern "C"` block, and a more specific level *overrides* a broader one.
With `slice = view` naming the default explicitly, an argument can also opt
back *out* of an inherited mode:

```rust
#[wasm_bindgen(module = "/lib.js", slice = copy)]
extern "C" {
    fn take_bytes(v: &[u8]);
    fn take_floats(v: Option<&[f64]>);

    fn mixed(
        copied: &[u8],
        #[wasm_bindgen(slice = array)] as_array: &[u8],
        #[wasm_bindgen(slice = view)] zero_copy: &[u8],
    );
}
```

Writing both `slice` and its `slice_to_array` alias at the same level is a
compile error.

## What this is not

* It does **not** apply to exported functions. Only outgoing arguments
  (Rust calling JS) are affected.
* It does **not** change non-slice arguments (a method's `this` receiver,
  `Vec<T>`, scalars): the mode is a silent no-op there, so it is safe on a
  whole `extern "C"` block of mixed-shape imports.
* `slice = copy` does **not** change slices whose element kind has no
  typed-array form. `&[JsString]`, `&[JsValue]` and other handle-shaped
  elements already reach JS as a fresh `Array` of shared handles, so `copy`
  is a no-op there. (`&[String]` has no borrowed ABI at all except under
  `slice = array`.)
* It does **not** reject a generic element type. `&[T]` for a type
  parameter `T` resolves per concrete instantiation under
  [`experimental_generic_mono`](experimental_generic_mono.md) (a typed copy
  at `T = u8`, a no-op at `T = JsString`), and is a no-op on the
  type-erasure generic path, where the slice erases to the handle tier.
* `array` and `copy` **cannot** be combined with a `&mut` slice, and the
  combination is rejected at compile time: JS receives an owned copy, so its
  writes would be silently discarded rather than written back through the
  `&mut`. Use a shared slice if JS only needs to read the elements, or set
  `slice = view` on that argument to keep the writable view.
