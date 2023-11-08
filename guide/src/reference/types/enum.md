# enum

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
| :-----------: | :------------: | :----------------: | :--------------: | :-------------------: | :----------------------: | :-----------------------: |
|       ?       |       ?        |         ?          |        ?         |           ?           |            ?             |             ?             |

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/enum.rs}}
```

## Example JavaScript Usage

```js
placeholder;
```

## TypeScript

Unfortunately, string enums don't fully work yet; no TypeScript is generated and functions using them accept or return `any`. The JavaScript part works fine, though. See [Issue #3057](https://github.com/rustwasm/wasm-bindgen/issues/3057)
