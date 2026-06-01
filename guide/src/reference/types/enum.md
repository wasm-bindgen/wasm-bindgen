# enum

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
| :-----------: | :------------: | :----------------: | :--------------: | :-------------------: | :----------------------: | :-----------------------: |
|      Yes      |       No       |         No         |       Yes        |          Yes          |           Yes            |   `string` or `number`    |

Only C-style enums are currently supported; see

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/enums.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/enums.js}}
```
