# `js_class = Blah`

The `js_class` attribute is used to indicate that all the methods inside an
`impl` block should be attached to the specified JS class instead of inferring
it from the self type in the `impl` block. The `js_class` attribute is most
frequently paired with [the `js_name` attribute](js_name.html) on structs:

```rust
#[wasm_bindgen(js_name = Foo)]
pub struct JsFoo { /* ... */ }

#[wasm_bindgen(js_class = Foo)]
impl JsFoo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsFoo { /* ... */ }

    pub fn foo(&self) { /* ... */ }
}
```

which is accessed like:

```rust
import { Foo } from './my_module';

const x = new Foo();
x.foo();
```

## When `js_class` is required

`js_class` must match the struct's [`js_name`](js_name.html) whenever the
struct sets one. The impl macro is a separate procedural-macro invocation
from the struct macro and cannot read the struct's attributes, so the JS
identity must be redeclared locally on every `impl` block. The same is
true for [`js_namespace`](js_namespace.html): if the struct declares it,
the `impl` block must declare a matching namespace too.

```rust
#[wasm_bindgen(js_name = Foo, js_namespace = bar)]
pub struct JsFoo { /* ... */ }

#[wasm_bindgen(js_class = Foo, js_namespace = bar)]   // both required
impl JsFoo { /* ... */ }
```

If you forget, `wasm-bindgen` emits a build error with the exact
attribute to add:

```
class `Foo` referenced by an impl block does not match any exported struct.
help: a struct with the same `js_name` exists in a different namespace.
The impl block must declare the matching `js_namespace`:
  - `js_namespace = bar` (matches struct `bar__Foo`)
```
