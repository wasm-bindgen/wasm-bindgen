# `getter` and `setter`

The `getter` and `setter` attributes can be used in Rust `impl` blocks to define
properties in JS that act like getters and setters of a field. For example:

```rust
#[wasm_bindgen]
pub struct Baz {
    field: i32,
}

#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(constructor)]
    pub fn new(field: i32) -> Baz {
        Baz { field }
    }

    #[wasm_bindgen(getter)]
    pub fn field(&self) -> i32 {
        self.field
    }

    #[wasm_bindgen(setter)]
    pub fn set_field(&mut self, field: i32) {
        self.field = field;
    }
}
```

Can be combined in `JavaScript` like in this snippet:

```js
const obj = new Baz(3);
assert.equal(obj.field, 3);
obj.field = 4;
assert.equal(obj.field, 4);
```

You can also configure the name of the property that is exported in JS like so:

```rust
#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(getter = anotherName)]
    pub fn field(&self) -> i32 {
        self.field
    }

    #[wasm_bindgen(setter = anotherName)]
    pub fn set_field(&mut self, field: i32) {
        self.field = field;
    }
}
```

Getters are expected to take no arguments other than `&self` and return the
field's type. Setters are expected to take one argument other than `&mut self`
(or `&self`) and return no values.

The name for a `getter` is by default inferred from the function name it's
attached to. The default name for a `setter` is the function's name minus the
`set_` prefix, and if `set_` isn't a prefix of the function it's an error to not
provide the name explicitly.

## Well-known symbols

Both `getter` and `setter` accept the explicit bracket-string form
`"[Symbol.<name>]"` to define an accessor keyed by a
[well-known symbol][well-known-symbols] on the generated JS class:

```rust
#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(getter = "[Symbol.toStringTag]")]
    pub fn to_string_tag(&self) -> String {
        "Foo".to_string()
    }
}
```

generates:

```js
export class Foo {
    get [Symbol.toStringTag]() { /* ... */ }
}
```

Only the exact form `"[Symbol.<ident>]"` is accepted. The same syntax is
also supported by [`js_name`](js_name.html) for non-getter/setter
methods.

[well-known-symbols]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol#well-known_symbols
