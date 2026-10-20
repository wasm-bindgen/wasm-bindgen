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

Owned data is freed once the call returns, so it is never a view.

Only numeric elements have a choice. A sequence of anything else
(`String`, `&str`, `bool`, `char`, exported enums, `JsValue` or imported
JS types) always arrives as a fresh `Array`, so `array` is the only mode
that fits it. Every [`JsStringLike`](experimental_generic_mono.md#string-bounds)
shape is a slice element, `&String`, `&Box<str>`, `&Cow<str>` and
`&JsString` included.

Borrowed slices of exported structs aren't supported; pass a `Vec<T>`
instead, which moves the elements to JS.

## `view`

The `slice` default: the slice arrives as a zero-copy typed-array view into
wasm linear memory (e.g. `Uint8Array`, `Float64Array`). The view is only
valid for the duration of the call: it dangles if the callee retains it, and
detaches if wasm memory grows during the call (any allocation in a
re-entrant callback). Writing `slice = view` opts a borrowed argument back
out of an inherited `copy` or `array`.

## `copy`

The `vec` default: JS receives a *fresh typed array* it owns outright. For a
borrowed slice, Rust passes the slice as usual, and the generated JS copies
it before the call:

```js
var v0 = getArrayU8FromWasm0(ptr0, len0).slice();
```

After the call returns, the wasm buffer and the JS array are unrelated.
This is the mode for JS APIs that retain the buffer past the call, or that
are typed `Uint8Array` / `BufferSource` and so would reject a plain
`Array`.

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
`Array<number>`) rather than a typed array. For a borrowed slice, the
generated JS builds the `Array` straight from wasm memory, with no copy in
Rust.

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

A mode written on an argument must fit it. When generating bindings, the
`wasm-bindgen` CLI checks the argument's actual type, through type aliases
and generic instantiations. It reports an error for `slice` on an owned
argument or `vec` on a borrowed one, for `view` or `copy` on non-numeric
elements, and for any `slice` mode other than `view` on a `&mut` slice,
including one inherited from the function or block.

## Generic element types

Under [`experimental_generic_mono`](experimental_generic_mono.md), a
`&[T]` or `Option<&[T]>` argument whose element is a type parameter
arrives, at each instantiation, exactly as the concrete slice would, its
mode included. The element is bounded by `VectorRefIntoWasmAbi`:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn log_items<T: VectorRefIntoWasmAbi>(items: &[T]);
}

log_items(&[1u8, 2]);                      // zero-copy Uint8Array view
log_items(&["a".to_string()]);             // fresh Array of strings
```

A generic `Vec<T>` argument resolves its `vec` mode at each instantiation in
the same way.

Without `experimental_generic_mono`, a generic slice's elements must be JS
types, so it always arrives as a fresh `Array`.

## What this is not

* Neither key applies to exported functions: on an export, its arguments, an
  exported method or its `impl` block, either one is an error.
