# `reexport`

The `reexport` attribute allows imported JavaScript items to be re-exported from
your wasm module, making them available to JavaScript code that imports your module.

```rust
#[wasm_bindgen(module = "my-utils")]
extern "C" {
    #[wasm_bindgen(reexport)]
    fn helper_function() -> u32;
}
```

generates JavaScript export glue like:

```js
import { helper_function } from "my-utils";
export { helper_function };
```

You can also provide a custom export name:

```rust
#[wasm_bindgen(module = "lodash")]
extern "C" {
    #[wasm_bindgen(reexport = "findIndex")]
    fn find_index(arr: &JsValue, val: &JsValue) -> i32;
}
```

which generates:

```js
import { find_index } from "lodash";
export { find_index as findIndex };
```

Only top-level imports can be re-exported, `reexport` cannot be used on methods, constructors, or static methods.
