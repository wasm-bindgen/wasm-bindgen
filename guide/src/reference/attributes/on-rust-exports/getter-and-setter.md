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

## Chaining Setters

Setters can support method chaining by taking `&mut self` and returning `&mut Self`:

```rust
#[wasm_bindgen]
pub struct User {
    name: String,
    age: i32,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new() -> User {
        User {
            name: String::new(),
            age: 0,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: String) -> &mut Self {
        self.name = value;
        self
    }

    #[wasm_bindgen(setter)]
    pub fn set_age(&mut self, value: i32) -> &mut Self {
        self.age = value;
        self
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn age(&self) -> i32 {
        self.age
    }
}
```

This enables ergonomic method chaining in Rust:

```rust
let mut user = User::new();
user.set_name("Alice".to_string())
    .set_age(30);

assert_eq!(user.name(), "Alice");
assert_eq!(user.age(), 30);
```

While in JavaScript, these are still exported as regular setters:

```js
const user = new User();
user.name = "Alice";
user.age = 30;

assert.equal(user.name, "Alice");
assert.equal(user.age, 30);
```

The appropriate pattern is automatically detected based on the setter signature.
