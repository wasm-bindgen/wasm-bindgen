# `JsValue`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | Yes | No | Yes | No | No | Any JavaScript value |

`JsValue` is also the default representation for many [erasable generic types](./js-sys.md) at the ABI boundary.

## `&[JsValue]` slices

You can pass `&[JsValue]` slices from Rust to JavaScript. The slice is passed as
a JavaScript `Array`.

```rust
#[wasm_bindgen]
extern "C" {
    fn process_values(values: &[JsValue]);
}

pub fn call_js() {
    let values: Vec<JsValue> = vec![1.into(), "hello".into(), true.into()];
    process_values(&values);
}
```

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/js_value.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/js_value.js}}
```
