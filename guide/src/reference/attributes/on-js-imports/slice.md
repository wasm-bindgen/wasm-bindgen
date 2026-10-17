# `slice`

The `slice` attribute selects how a sequence argument of an imported
function reaches JS. It applies to borrowed slices (`&[T]`,
`Option<&[T]>`), owned sequences (`Vec<T>`, `Box<[T]>`, `Option<Vec<T>>`)
and their `Clamped` forms alike. For numeric element types (`u8`, `i32`,
`f64`, ...) there are three modes:

| mode | borrowed `&[T]` | owned `Vec<T>` |
|---|---|---|
| `slice = view` *(default)* | typed-array view of wasm memory | fresh typed array |
| `slice = array` | fresh plain `Array` | fresh plain `Array` |
| `slice = copy` | fresh typed array | fresh typed array |

Owned data is freed once the call returns, so it always reaches JS as a
copy. The user-facing Rust signature is unchanged by the mode — `&[T]`
stays `&[T]`.

String and handle elements (`&[String]`, `&[JsValue]`, `&[JsString]`,
imported JS types) have no typed-array form and always reach JS as a fresh
`Array`; `view` and `copy` leave them exactly as they are without a mode.

## `slice = view`

The default for borrowed slices: the slice arrives as a zero-copy
typed-array view into wasm linear memory (e.g. `Uint8Array`,
`Float64Array`). The view is only valid for the duration of the call: it
dangles if the callee retains it, and detaches if wasm memory grows during
the call (any allocation in a re-entrant callback). Writing the mode
explicitly opts an argument back out of an inherited `array` or `copy`.

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

For numeric elements of a borrowed slice the wire is the same zero-copy
borrow as the default, and the JS glue materialises the `Array` with
`Array.from(..)`. Under `array`, a `&[String]` argument is also supported
(it has no default ABI): the Rust side builds a fresh buffer of JS values,
one per element, which JS reads into an `Array` and frees. For handle
elements this is a refcount bump each; for `String` a fresh JS string.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives `Array<number>` rather than `Uint16Array`.
    #[wasm_bindgen(slice = array)]
    fn set_indices(values: &[u16]);

    #[wasm_bindgen(slice = array)]
    fn set_names(values: &[String]);
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

A slice whose element type is a type parameter resolves its mode per
concrete instantiation under
[`experimental_generic_mono`](experimental_generic_mono.md) — e.g. a typed
copy at `T = u8` and the handle tier's fresh `Array` at `T = JsString` —
and is a no-op on the type-erasure generic path, where the slice erases to
the handle tier. On the per-mono path a generic element keeps the slice's
default borrowed ABI, so `T = String` fails to compile exactly as without
the attribute, even though the concrete `&[String]` spelling is supported
under `array`.

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
