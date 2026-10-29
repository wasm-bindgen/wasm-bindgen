# `Nullable<T>`

`Nullable<T>` is the Rust-side type for a JS value of type `T | null`, as
`Option<T>` is for `T | undefined`. The union `T | null | undefined` is
`Option<Nullable<T>>`. All three are accepted in the same positions: as an
argument or return value of exported and imported functions, as a public field
of an exported struct, and as a closure argument or return value, including
the reference forms (`Nullable<&str>`, `Nullable<&[T]>`, `Nullable<&JsValue>`)
that `Option` supports in imports.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nullable<T> {
    Null, // also the `Default`
    Value(T),
}
```

| Type | JS type | JS to Rust | Rust to JS |
|---|---|---|---|
| `Option<T>` | `T \| undefined` | `undefined` and `null` are `None`; anything else goes to `T`'s conversion | `None` is `undefined` |
| `Nullable<T>` | `T \| null` | `null` is `Null`; anything else, including `undefined`, goes to `T`'s conversion | `Null` is `null` |
| `Option<Nullable<T>>` | `T \| null \| undefined` | `undefined` is `None`, `null` is `Some(Null)`, anything else is `Some(Value(..))` | `None` is `undefined`, `Some(Null)` is `null`, `Some(Value(v))` is `v` |

The `Option` layer of `Option<Nullable<T>>` treats only `undefined` as `None`,
since `null` has to reach the `Nullable` layer. A plain `Nullable<T>` passes
`undefined` to `T`'s own conversion, exactly as any other value that isn't a
`T`: the debug glue throws its type assertion, and the release glue coerces or
throws as `T` itself would.

No other nesting is supported. `Nullable<Nullable<T>>`, `Nullable<Option<T>>`
and `Option<Option<T>>` do not cross the boundary.

```rust
#[wasm_bindgen]
pub fn find(key: &str) -> Nullable<String> {
    match lookup(key) {
        Some(value) => Nullable::Value(value),
        None => Nullable::Null,
    }
}

#[wasm_bindgen]
pub fn describe(value: Option<Nullable<u32>>) -> String {
    match value {
        None => "undefined".into(),
        Some(Nullable::Null) => "null".into(),
        Some(Nullable::Value(n)) => n.to_string(),
    }
}
```

```ts
find(key: string): string | null;
describe(value?: number | null): string;
```

A `Nullable<T>` argument is never omittable in TypeScript, since `undefined`
is not one of its states; an `Option<Nullable<T>>` one is `arg?: T | null`
when trailing, and such a struct field is `field?: T | null`.

The API follows `Option`'s names: `is_null`, `is_value`, `is_value_and`,
`is_null_or`, `as_ref`, `as_mut`, `as_deref`, `as_deref_mut`, `copied`,
`cloned`, `map`, `and_then`, `filter`, `ok_or`, `ok_or_else`, `take`,
`replace`, `iter`, `iter_mut`, `value()` / `into_option()`, `unwrap`,
`expect`, `unwrap_or`, `unwrap_or_else` and `unwrap_or_default`, and
`Nullable<T>` and its references are `IntoIterator` over the value, so
`into_iter().flatten()` folds nullables into an iterator pipeline. `From<T>`
gives `Value`; as lossy conveniences, `From<Option<T>>` maps `None` to `Null`
and `Option<T>` can be built from a `Nullable<T>` with `Null` becoming
`None`. `opt.and_then(Nullable::value)` collapses an `Option<Nullable<T>>` to
an `Option<T>`.

On stable, `?` goes through `ok_or` or `.value()?`. On nightly,
`--cfg wasm_bindgen_unstable_try_trait` enables `try_trait_v2` so `?` works
on a `Nullable<T>` directly: `Null` short-circuits a function returning
`Nullable<U>` to `Null` and one returning `Option<U>` to `None`, and `None`
from an `Option<T>` short-circuits a `Nullable<U>` function to `Null`.

## `Nullable<NonNull<T>>`

Every non-null address is a `NonNull<T>`, so a `Nullable<NonNull<T>>` crosses
as a JS number with the pointer sentinels, as `Nullable<*const T>` does, and
the null pointer itself is `Null`. A JS `0` passed for either a
`Nullable<NonNull<T>>` or an `Option<Nullable<NonNull<T>>>` reaches Rust as
`Null`, as it is `None` for `Option<NonNull<T>>`.

## In a `Function` signature

A js-sys `Function` signature may name them, since its slots are JS types
rather than handles: a `Closure<dyn FnMut(Nullable<JsString>)>` makes a
`Function<fn(Nullable<JsString>) -> Undefined>` with `Function::from_closure`,
and a `Promise<Nullable<JsString>>` awaits to a `Nullable<JsString>`.

## `Nullable<T>` and `JsNullable<T>`

`Nullable<T>` and `Option<T>` are Rust-side types that convert at the
boundary, so they fit wherever the value, or the promise holding it, is
converted on crossing. [`JsNullable<T>`](./js-sys.md) and `JsOption<T>` are
JS-side handles: the value stays a JS value, and they are what the
type-erased generic methods of js-sys's containers work on. An
`Array<Nullable<JsString>>` or `Map<JsString, Nullable<JsString>>` is a valid
type, and crosses whole, but `Array::get` or `Map::get` on it does not
resolve: those marshal the element as a `JsValue` handle, which a Rust enum
cannot be, as `Promise::resolve(&Nullable::Null)` cannot borrow one. Name the
element `JsNullable<T>` to use them, converting with `From` at the use site:
for `T: JsGeneric`, a `Nullable<T>` and an `Option<Nullable<T>>` convert into
a `JsNullable<T>`, `None` becoming `undefined`, and a `JsNullable<T>`
converts into an `Option<Nullable<T>>`, keeping all three states.

The type is also re-exported as `js_sys::Nullable`.
