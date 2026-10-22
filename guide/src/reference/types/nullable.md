# `Nullable<T>`

`Nullable<T>` is the Rust-side type for a JS value of type
`T | null | undefined`. It is accepted everywhere `Option<T>` is: as an
argument or return value of exported and imported functions, as a public field
of an exported struct, and as a closure argument or return value, including
the reference forms (`Nullable<&str>`, `Nullable<&[T]>`, `Nullable<&JsValue>`)
that `Option` supports in imports.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nullable<T> {
    Null, // also the `Default`
    Undefined,
    Value(T),
}
```

Where `Option<T>` folds the empty JS states into `None` and always sends
`undefined` back, `Nullable<T>` keeps them apart in both directions:

| Type | JS type | JS to Rust | Rust to JS |
|---|---|---|---|
| `Option<T>` | `T \| undefined` | `undefined` is `None`; anything else, including `null` under the [strict cfg](./option.md#strict-optiont-unstable), goes to `T`'s conversion | `None` is `undefined` |
| `Nullable<T>` | `T \| null \| undefined` | `null` is `Null`, `undefined` is `Undefined`, anything else goes to `T`'s conversion | `Null` is `null`, `Undefined` is `undefined`, `Value(v)` is `v` |

Three variants, because the type admits all three JS states and the value
sent to JS is the one held: a `T | null` API gets `Null`, and an optional
`T | null` setter can still send `Undefined`.

```rust
#[wasm_bindgen]
pub fn find(key: &str) -> Nullable<String> {
    match lookup(key) {
        Some(value) => Nullable::Value(value),
        None => Nullable::Null,
    }
}

#[wasm_bindgen]
pub fn describe(value: Nullable<u32>) -> String {
    match value {
        Nullable::Null => "null".into(),
        Nullable::Undefined => "undefined".into(),
        Nullable::Value(n) => n.to_string(),
    }
}
```

```ts
find(key: string): string | null | undefined;
describe(value?: number | null): string;
```

The API follows `Option`'s names: `is_null`, `is_undefined`, `is_empty`,
`is_value`, `as_ref`, `as_mut`, `map`, `value()` / `into_option()`, `unwrap`,
`expect`, `unwrap_or`, `unwrap_or_else` and `unwrap_or_default`. `From<T>`
gives `Value`, `From<Option<T>>` maps `None` to `Null` (the canonical empty
value of a nullable type, as `JsNullable::new()` is), and `Option<T>` can be
built from a `Nullable<T>` with both empties becoming `None`.

## `Nullable<T>` and `JsNullable<T>`

`Nullable<T>` and `Option<T>` are Rust-side types that convert at the
boundary. [`JsNullable<T>`](./js-sys.md) and `JsOption<T>` are JS-side
handles: the value stays a JS value, and they are what to use inside another
JS type, such as `Array<JsNullable<T>>` or `Promise<JsOption<T>>`, and in
generic positions. For `T: JsGeneric` the two convert into each other with
`From`, preserving `null` versus `undefined`.

The type is also re-exported as `js_sys::Nullable`.
