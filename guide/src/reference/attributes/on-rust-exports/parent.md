# `parent`

A field-level attribute that marks the field storing the parent instance
for a struct that uses `#[wasm_bindgen(extends = ...)]`. Required; exactly
one field per such struct must carry it.

```rust
#[wasm_bindgen(extends = Animal)]
pub struct Dog {
    #[wasm_bindgen(parent)]
    parent: Animal,
    breed: String,
}
```

The marked field is the projection target for the generated
`AsRef<Parent>` / `AsMut<Parent>` / `Deref<Target = Parent>` /
`DerefMut` impls. Because it's a plain Rust field, struct-literal
construction, `Drop`, and `Clone` behave exactly as you would expect —
wasm-bindgen does not insert any hidden state.

The field:

* **does not need to be `pub`**. Private parent fields stay private; they
  are *not* emitted as JS getters/setters even when the enclosing struct
  otherwise exposes its public fields.
* **is not serialized** in any automatic conversion — it's Rust-side
  storage only.
* **must not have `#[wasm_bindgen(getter_with_clone)]`** or other
  field-level attributes that would conflict with private storage.

For the full semantics and examples, see
[`extends`](./extends.html).
