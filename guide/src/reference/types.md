# Supported Rust Types and their JavaScript Representations

This section provides an overview of all the types that `wasm-bindgen` can send
and receive across the WebAssembly ABI boundary, and how they translate into
JavaScript.

Many core JavaScript types support [generic type parameters](./types/js-sys.md) using type erasure, allowing you to work with typed collections like `Array<T>`, `Promise<T>`, `Map<K, V>`, and more.

Each page's table says whether the type is accepted inside `Option`. How
`None`, `undefined` and `null` then map across the boundary is described on
the [`Option<T>`](./types/option.md) page, and
[`Nullable<T>`](./types/nullable.md) is accepted wherever `Option<T>` is for a
`T | null` value, with `Nullable<Option<T>>` for `T | null | undefined`.

`()` is `undefined`. A function that returns `()` returns nothing. As an
argument, `()` has its place in the JS signature, typed `undefined`: an
exported function or a closure is passed `undefined` there, which a debug
build checks, and an imported function receives `undefined` there. As an
element it makes a [`Vec<()>`](./types/boxed-slices.md) an `Array` of
`undefined`. It can't be wrapped in an `Option`, whose `None` is `undefined`
already.
