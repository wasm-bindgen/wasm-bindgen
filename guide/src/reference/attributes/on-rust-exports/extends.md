# `extends = Parent`

The `extends` attribute on an exported Rust struct declares that the struct
inherits from another exported Rust struct. This produces a JS class with a
real prototype chain (`class Child extends Parent`), so `instanceof Parent`
is true for every `Child` instance, and gives the child a Rust-side
`AsRef<Parent>`/`Deref<Target = Parent>` for free.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Animal {
    name: String,
}

#[wasm_bindgen]
impl Animal {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Animal {
        Animal { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen(extends = Animal)]
pub struct Dog {
    #[wasm_bindgen(parent)]
    parent: Animal,
    breed: String,
}

#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, breed: String) -> Dog {
        Dog {
            parent: Animal::new(name),
            breed,
        }
    }

    pub fn breed(&self) -> String {
        self.breed.clone()
    }
}
```

Which generates the following JS shape:

```js
class Animal { /* ... */ }
class Dog extends Animal { /* ... */ }

const rex = new Dog("Rex", "Labrador");
rex instanceof Dog;    // true
rex instanceof Animal; // true
rex.breed();           // "Labrador"
```

And the following Rust-side trait impls:

```rust
impl AsRef<Animal>    for Dog { /* projects &self.parent */ }
impl AsMut<Animal>    for Dog { /* projects &mut self.parent */ }
impl Deref            for Dog { type Target = Animal; /* ... */ }
impl DerefMut         for Dog { /* ... */ }
```

## The `parent` field

The struct must declare exactly one field marked with
`#[wasm_bindgen(parent)]`, typed as the parent struct. That field is where
the child physically stores its parent instance; the generated `AsRef`,
`Deref`, etc. projections borrow through it. The field does not need to be
`pub` — and if it isn't, it's not exposed to JS as a getter.

## Inheriting parent methods in JS

The JS prototype chain gives you `instanceof Parent` and any JS-level
methods (like `toString`) for free. For methods exported from Rust, you
re-export them explicitly on the child — this is a one-liner thanks to
`Deref`:

```rust
#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(js_name = name)]
    pub fn name_forward(&self) -> String {
        use std::ops::Deref;
        self.deref().name()
    }
}
```

Now `rex.name()` works in JS and returns `"Rex"`.

Automatic forwarding of every parent method onto the child is not yet
implemented; see the [tracking issue][1] for context.

[1]: https://github.com/wasm-bindgen/wasm-bindgen/issues/1721

## Limitations

* **One parent per struct.** Multi-parent inheritance is not supported;
  only one `extends = ...` and one `#[wasm_bindgen(parent)]` field.
* **Same module only.** The parent must be another `#[wasm_bindgen]` struct
  exported from the same Rust crate (same wasm module). Extending
  imported JS classes (e.g. `HTMLElement`) from a Rust-exported struct is
  a separate feature — see the [`extends` attribute on imports][2].
* **No method overriding across the hierarchy.** In this first pass a
  child may not define a method with the same name as a parent method.
* **Parent must have a user-defined `#[wasm_bindgen(constructor)]`** if
  it's going to be extended and the child has its own constructor —
  subclass construction calls `super(...)` with a module-level sentinel
  that short-circuits the parent's ctor body.
* **No generics** on a struct that uses `extends`.

[2]: ../on-js-imports/extends.html
