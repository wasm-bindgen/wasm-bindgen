# Supported Rust Types and their JavaScript Representations

This section provides an overview of all the types that `wasm-bindgen` can send
and receive across the WebAssembly ABI boundary, and how they translate into
JavaScript.

Many core JavaScript types support [generic type parameters](./types/js-sys.md) using type erasure, allowing you to work with typed collections like `Array<T>`, `Promise<T>`, `Map<K, V>`, and more.

Each page's table says whether the type is accepted inside `Option`. How
`None`, `undefined` and `null` then map across the boundary is described on
the [`Option<T>`](./types/option.md) page, and
[`Nullable<T>`](./types/nullable.md) is accepted wherever `Option<T>` is for a
`T | null` value, with `Option<Nullable<T>>` for `T | null | undefined`.
