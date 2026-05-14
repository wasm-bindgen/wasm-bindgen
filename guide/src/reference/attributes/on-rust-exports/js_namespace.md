# `js_namespace = blah`

This attribute indicates that the exported Rust function or type should be
placed within the given JavaScript namespace. Instead of exporting items
individually, they will be grouped together in a namespace object. This is
useful for organizing related exports and avoiding namespace pollution.

```rust
#[wasm_bindgen(js_namespace = math)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_namespace = math)]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

This will generate JavaScript that exports a namespace object containing the
functions:

```js
export const math = { add, multiply };
```

Which can be used from JavaScript as:

```js
import { math } from './my_module';

const sum = math.add(2, 3);
const product = math.multiply(4, 5);
```

## Namespaced structs and `impl` blocks

When a namespaced struct has methods, the `impl` block must repeat the
`js_namespace` (and any `js_class` from a `js_name` rename):

```rust
#[wasm_bindgen(js_namespace = math)]
pub struct Counter { /* ... */ }

#[wasm_bindgen(js_namespace = math)]   // required
impl Counter { /* ... */ }
```

The impl macro is a separate procedural-macro invocation from the
struct macro and cannot read the struct's attributes, so the JS-side
identity (`js_class` + `js_namespace`) must be redeclared on the impl
block. If you forget, the build fails with a precise hint pointing at
the exact attribute to add.

## Default Export

This feature can be used to define a default export object:

```rust
#[wasm_bindgen(js_namespace = "default")]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen(js_namespace = "default")]   // required (see note above)
impl Counter { /* ... */ }
```

resulting in the output:

```js
export default {
    Counter
}
```

which can be imported with:

```js
import module from './my_module';

const counter = new module.Counter();
```

## Nested Namespaces

It is also possible to create nested namespaces by providing an array of
strings to specify the namespace path:

```rust
#[wasm_bindgen(js_namespace = ["utils", "string"])]
pub fn concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

#[wasm_bindgen(js_namespace = ["utils", "string"])]
pub fn uppercase(s: &str) -> String {
    s.to_uppercase()
}
```

This will generate nested namespace objects:

```js
const utils = {
    string: { concat, uppercase }
};
export { utils };
```

Which can be accessed in JavaScript as:

```js
import { utils } from './my_module';

const result = utils.string.concat("hello", "world");
const upper = utils.string.uppercase("hello");
```
