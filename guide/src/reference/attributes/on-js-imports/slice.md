# `slice` and `vec`

The `slice` and `vec` attributes select how a sequence argument of an
imported function reaches JS when its element type `T` is numeric (`u8`,
`i32`, `f64`, ...). The key names the Rust shape, and the value what JS
receives:

| key | shapes | values | default |
|---|---|---|---|
| `slice` | `&[T]`, `&mut [T]`, `Clamped<&[u8]>`, or an `Option` of one | `view`, `copy`, `array` | `view` |
| `vec` | `Vec<T>`, `Box<[T]>`, `Clamped<Vec<u8>>`, or an `Option` of one | `copy`, `array` | `copy` |

* `view`: a live typed array over wasm memory.
* `copy`: a fresh typed array.
* `array`: a fresh plain `Array`.

Owned data is freed once the call returns, so it is never a view. The
user-facing Rust signature is unchanged by the mode — `&[T]` stays `&[T]`.

Every sequence holds its elements in one of two ways:

* In place: the element is its own JS encoding, so the slice's memory is
  read directly. That is numbers, and JS handles (`JsValue`, `JsString`,
  imported JS types), whose memory is their table indices.
* Encoded: each element crosses as a single argument of its type would,
  through a temporary buffer. That is `String`, `&str`, `bool`, `char` and
  exported enums.

The modes only apply to numeric `T`. Any other sequence has no typed-array
form and always reaches JS as a fresh `Array`.

A borrowed slice of exported structs is not supported, as a struct has no
borrowed ABI; an owned `Vec<T>` moves its elements to JS instead.

## `view`

The `slice` default: the slice arrives as a zero-copy typed-array view into
wasm linear memory (e.g. `Uint8Array`, `Float64Array`). The view is only
valid for the duration of the call: it dangles if the callee retains it, and
detaches if wasm memory grows during the call (any allocation in a
re-entrant callback). Writing `slice = view` opts a borrowed argument back
out of an inherited `copy` or `array`.

## `copy`

JS receives a *fresh typed array* it owns outright. For a borrowed slice the
wire format is unchanged (the borrowed slice; no Rust-side allocation) —
only the JS glue changes, copying the view before the call:

```js
var v0 = getArrayU8FromWasm0(ptr0, len0).slice();
```

After the call returns, the wasm buffer and the JS array are unrelated.
This is the mode for JS APIs that retain the buffer past the call, or that
are typed `Uint8Array` / `BufferSource` and so would reject a plain
`Array`. It is the `vec` default, as owned data is copied anyway.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives a fresh `Uint8Array` it may retain.
    #[wasm_bindgen(slice = copy)]
    fn store_bytes(v: &[u8]);
}
```

## `array`

JS receives a plain `Array`, for APIs that expect `T[]` (e.g.
`Array<number>`) rather than a typed array. For a borrowed slice the wire is
the same zero-copy borrow as the default, and the JS glue materialises the
`Array` with `Array.from(..)`.

```rust
#[wasm_bindgen]
extern "C" {
    // JS receives `Array<number>` rather than `Uint16Array`.
    #[wasm_bindgen(slice = array)]
    fn set_indices(values: &[u16]);
    #[wasm_bindgen(vec = array)]
    fn set_owned_indices(values: Vec<u16>);
}
```

## Per-argument, per-function, per `extern "C"` block

Each key can be written on an argument, on a function, or on an
`extern "C"` block, and a more specific level *overrides* a broader one,
for each key independently:

```rust
#[wasm_bindgen(module = "/lib.js", slice = copy, vec = array)]
extern "C" {
    fn take_bytes(v: &[u8], owned: Vec<u8>);

    fn mixed(
        copied: &[u8],
        #[wasm_bindgen(slice = array)] as_array: &[u8],
        #[wasm_bindgen(slice = view)] zero_copy: &[u8],
        #[wasm_bindgen(vec = copy)] typed: Vec<u8>,
    );
}
```

Inherited from a function or block, a key acts only on arguments of its own
shape with numeric elements, and is a silent no-op on every other argument
(a method's `this` receiver, scalars, other sequences), so it is safe on a
whole `extern "C"` block of mixed-shape imports.

Written on an argument, a mode must fit it. `wasm-bindgen` reports an error
for `slice` on an owned argument or `vec` on a borrowed one, for a mode
other than `slice = view` on a `&mut` slice, and for any mode on a sequence
of non-numeric elements. The argument's actual type is checked, through
type aliases and generic instantiations.

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
`Array`, so it has no typed-array form for a mode to act on.

## What this is not

* Neither key applies to exported functions; only outgoing arguments
  (Rust calling JS) are affected.
* `slice = copy` and `slice = array` **cannot** be combined with a `&mut`
  slice, and the combination is rejected at compile time: JS receives an
  owned copy, so its writes would be silently discarded rather than written
  back through the `&mut`. Use a shared slice if JS only needs to read the
  elements, or set `slice = view` on that argument to keep the writable
  view.
