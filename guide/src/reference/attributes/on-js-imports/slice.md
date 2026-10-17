# `slice`

The `slice` attribute selects how a sequence argument of an imported
function reaches JS when its element type `T` is numeric (`u8`, `i32`,
`f64`, ...). A sequence is a borrowed slice (`&[T]`), an owned `Vec<T>` or
`Box<[T]>`, or an `Option` or `Clamped` form of either:

| mode | borrowed `&[T]` | owned `Vec<T>` / `Box<[T]>` |
|---|---|---|
| *(none)* | typed-array view of wasm memory | fresh typed array |
| `slice = view` | typed-array view of wasm memory | *(error)* |
| `slice = array` | fresh plain `Array` | fresh plain `Array` |
| `slice = copy` | fresh typed array | fresh typed array |

Owned data is freed once the call returns, so it always reaches JS as a
copy, and `slice = view` written on an owned argument is a compile error
(inherited from a function or block, it is a no-op there). The user-facing
Rust signature is unchanged by the mode — `&[T]` stays `&[T]`.

The modes only apply to numeric `T`. A borrowed slice of any other element
type has no typed-array form: it crosses as each element's own ABI and
reaches JS as a fresh `Array`, whatever the mode.

* JS handles (`JsValue`, `JsString`, imported JS types) are read in place,
  since the slice's memory is their table indices.
* `String`, `&str`, `bool`, `char` and exported enums are encoded into a
  temporary buffer of their ABIs, which the glue decodes with each
  element's own conversion and then frees.

A borrowed slice of exported structs is not supported, as a struct has no
borrowed ABI; an owned `Vec<T>` moves its elements to JS instead.

## `slice = view`

The default for borrowed slices: the slice arrives as a zero-copy
typed-array view into wasm linear memory (e.g. `Uint8Array`,
`Float64Array`). The view is only valid for the duration of the call: it
dangles if the callee retains it, and detaches if wasm memory grows during
the call (any allocation in a re-entrant callback). Writing the mode
explicitly opts a borrowed argument back out of an inherited `array` or
`copy`; an owned argument opts out with `slice = copy`.

## `slice = copy`

JS receives a *fresh typed array* it owns outright. The wire format is
unchanged (the borrowed slice; no Rust-side allocation) — only the JS glue
changes, copying the view before the call:

```js
var v0 = getArrayU8FromWasm0(ptr0, len0).slice();
```

After the call returns, the wasm buffer and the JS array are unrelated.
This is the mode for JS APIs that retain the buffer past the call, or that
are typed `Uint8Array` / `BufferSource` and so would reject a plain
`Array`. On owned sequences, which are already copied, it is a no-op.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives a fresh `Uint8Array` it may retain.
    #[wasm_bindgen(slice = copy)]
    fn store_bytes(v: &[u8]);
}
```

## `slice = array`

JS receives a plain `Array`, for APIs that expect `T[]` (e.g.
`Array<number>`) rather than a typed array.

For a borrowed slice the wire is the same zero-copy borrow as the default,
and the JS glue materialises the `Array` with `Array.from(..)`.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives `Array<number>` rather than `Uint16Array`.
    #[wasm_bindgen(slice = array)]
    fn set_indices(values: &[u16]);
}
```

## Per-argument, per-function, per `extern "C"` block

The mode can be written on an argument, on a function, or on an
`extern "C"` block, and a more specific level *overrides* a broader one:

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

The mode is a silent no-op on arguments that are not sequences (a method's
`this` receiver, scalars), so it is safe on a whole `extern "C"` block of
mixed-shape imports.

## Generic element types

Under [`experimental_generic_mono`](experimental_generic_mono.md), a
`&[T]` or `Option<&[T]>` argument whose element is a type parameter
crosses, at each instantiation, exactly as the concrete slice would, its
mode included:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn log_items<T>(items: &[T]);
}

log_items(&[1u8, 2]);                      // zero-copy Uint8Array view
log_items(&["a".to_string()]);             // fresh Array of strings
```

On the type-erasure generic path the slice instead erases to the handle
tier, which requires a JS-handle element type and already hands JS a fresh
`Array`, so the modes are a documented no-op.

## What this is not

* It does **not** apply to exported functions; only outgoing arguments
  (Rust calling JS) are affected.
* `array` and `copy` **cannot** be combined with a `&mut` slice, and the
  combination is rejected at compile time: JS receives an owned copy, so
  its writes would be silently discarded rather than written back through
  the `&mut`. Use a shared slice if JS only needs to read the elements, or
  set `slice = view` on that argument to keep the writable view.

## `slice_to_array`

`slice_to_array` is the deprecated predecessor of `slice = array`. It acts
on `&[T]` and `Option<&[T]>` arguments only, and leaves owned and `Clamped`
sequences unchanged.
