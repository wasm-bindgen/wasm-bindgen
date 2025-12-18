# Exported `struct Whatever` Rust Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | `Option<&T>` parameter | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | Yes | Yes | Yes | Yes | Yes | Instances of a `wasm-bindgen`-generated JavaScript `class Whatever { ... }` |

## `Option<&T>` Parameters

You can pass optional borrowed references to exported structs. Both `Option<&T>` and
`&Option<T>` have the same representation and can be used as function parameters:

```rust
#[wasm_bindgen]
pub fn process_optional(value: Option<&MyStruct>) -> u32 {
    match value {
        Some(s) => s.get_value(),
        None => 0,
    }
}
```

JavaScript passes either `undefined`/`null` for `None`, or an instance of the struct for `Some`:

```javascript
// Pass Some value
const result1 = process_optional(new MyStruct(42)); // returns 42

// Pass None
const result2 = process_optional(undefined); // returns 0
```

> **Note**: Public fields implementing `Copy` have automatically generated getters/setters.
> To generate getters/setters for non-`Copy` public fields, use `#[wasm_bindgen(getter_with_clone)]` for the struct
> or [implement getters/setters manually](https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html).

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/exported_types.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/exported_types.js}}
```
