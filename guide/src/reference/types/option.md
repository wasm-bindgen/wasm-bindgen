# `Option<T>`

`Option<T>` is accepted wherever the page for `T` lists `Option<T>` as
supported: as an argument or return value of exported and imported functions,
as a public field of an exported struct, and as a closure argument or return
value.

Going from Rust to JS, `None` is `undefined`. Going from JS to Rust, both
`undefined` and `null` are `None`, for every `T`.

The two directions differ because only one of them makes a choice. Sending a
`None` has to pick a JS value, and that is `undefined`. Reading a JS empty
decides nothing: a `T | null` value loses no information as an `Option<T>`, so
a binding can read `T | null` as `Option<T>`. An exported optional argument is
typed `T | null | undefined` in TypeScript, or `arg?: T | null` when trailing,
and an optional return value `T | undefined`.

When a slot has to carry `null` out, or when both empties come in and the
caller has to tell them apart, use [`Nullable<T>`](./nullable.md) for
`T | null` and `Option<Nullable<T>>` for `T | null | undefined`. The `Option`
layer of `Option<Nullable<T>>` treats only `undefined` as `None`, since `null`
has to reach the `Nullable` layer.
