# Imported `extern Whatever;` JavaScript Types

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | Yes | Yes | Instances of the extern `Whatever` JavaScript class / prototype constructor |

Imported JavaScript types support [generic type parameters](./js-sys.md) for working with typed JavaScript collections and APIs.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/imported_types.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/imported_types.js}}
```
