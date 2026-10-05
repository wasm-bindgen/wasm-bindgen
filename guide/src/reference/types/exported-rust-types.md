# Exported `struct Whatever` Rust Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | Yes | Yes | Yes | Yes | Instances of a `wasm-bindgen`-generated JavaScript `class Whatever { ... }` |

> **Note**: Public fields implementing `Copy` have automatically generated getters/setters.
> To generate getters/setters for non-`Copy` public fields, use `#[wasm_bindgen(getter_with_clone)]` for the struct
> or [implement getters/setters manually](https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html).

Passing an instance by value (`T` or `Option<T>` parameter) moves it into
Rust: the value is not copied, and the JavaScript object is left empty, as if
`.free()` had been called on it. Any later use of that object, whether passing
it to another function or accessing its fields and methods, throws an error
(`Attempt to use a moved value` when built with `--debug`, `null pointer
passed to rust` otherwise). Passing by reference (`&T` or `&mut T`) leaves the
JavaScript object usable.

Exported functions can use [generic JavaScript types](./js-sys.md) with concrete type parameters (like `Promise<Number>`), but cannot have their own generic type parameters.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/exported_types.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/exported_types.js}}
```
