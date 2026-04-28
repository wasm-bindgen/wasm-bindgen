# `extends = Parent`

The `extends` attribute on an exported Rust struct declares that the struct
inherits from another exported Rust struct. This produces a JS class with a
real prototype chain (`class Child extends Parent`), so `instanceof Parent`
is true for every `Child` instance, and parent methods dispatched via the JS
prototype chain work soundly because every instance carries a separate,
properly-refcounted pointer for each ancestor.

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
    breed: String,
}

#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, breed: String) -> Dog {
        Dog {
            parent: Animal::new(name).into(),
            breed,
        }
    }

    pub fn breed(&self) -> String {
        self.breed.clone()
    }

    pub fn parent_name(&self) -> String {
        self.parent.borrow().name()
    }
}
```

This generates:

```js
class Animal { /* ... */ }
class Dog extends Animal { /* ... */ }

const rex = new Dog("Rex", "Labrador");
rex instanceof Dog;    // true
rex instanceof Animal; // true
rex.breed();           // "Labrador"
rex.name();            // "Rex" — dispatched via the JS prototype chain
```

## The injected `parent` field

The macro injects a hidden `parent` field on any struct that uses
`#[wasm_bindgen(extends = Parent)]`. You never declare it yourself — it's
typed as `wasm_bindgen::Parent<Parent>` (a refcounted cell around the parent
data) and is visible to your own `impl` blocks as `self.parent`.

Initialize it in the constructor via `Parent::new(...)` or the ergonomic
`.into()`:

```rust
Dog { parent: Animal::new(name).into(), breed }
```

Access it in child methods through `self.parent.borrow()` or
`self.parent.borrow_mut()`:

```rust
impl Dog {
    pub fn greet(&self) -> String {
        format!("Hi, I'm {}!", self.parent.borrow().name())
    }
}
```

Declaring your own field named `parent`, or any field typed as
`wasm_bindgen::Parent<T>`, is an error — the macro owns that field.

The macro also derives `impl AsRef<wasm_bindgen::Parent<Parent>>` on the
child, so generic Rust code can accept any descendant where it expects a
borrowed reference to the parent's `Parent<…>` cell:

```rust
fn animal_name<T: AsRef<wasm_bindgen::Parent<Animal>>>(t: &T) -> String {
    t.as_ref().borrow().name()
}
animal_name(&dog); // "Rex"
```

## How inheritance works at runtime

For every class in an `extends` chain, each JS instance carries one
pointer per ancestor:

```
dog.__wbg_ptr_Dog     // Rc<WasmRefCell<Dog>>    raw pointer
dog.__wbg_ptr_Animal  // Rc<WasmRefCell<Animal>> raw pointer (clone of Dog's inner Parent<Animal>)
```

Each exported method reads from the per-class field that matches the class
where it was defined. So `Animal.prototype.name`, when called on a `Dog`
instance via the prototype chain, passes `this.__wbg_ptr_Animal` to the
wasm `Animal::name` shim — which is the correct type. Child-defined methods
read from the child's own per-class field.

On `dog.free()` (or garbage collection via the `FinalizationRegistry`),
every pointer is released. The child's ancestor pointer is a JS-held clone
of the Rust-side `Parent<T>` field's `Rc`, so dropping both the child
(which releases its inner `Rc` clone) and the JS-held clone brings the
ancestor's refcount to zero exactly once.

## Limitations

* **One parent per struct.** Multi-parent inheritance is not supported;
  only one `extends = ...` per struct.
* **Same module only.** The parent must be another `#[wasm_bindgen]` struct
  exported from the same Rust crate (same wasm module). Extending
  imported JS classes (e.g. `HTMLElement`) from a Rust-exported struct is
  a separate feature — see the [`extends` attribute on imports][2].
* **No `super.foo()` from Rust.** Method overriding works as you'd expect
  via JS prototype lookup: a child method with the same name as a parent
  method shadows the parent's, and dispatch on a child instance lands on
  the child's method. Each class's wasm shim reads its own per-class
  pointer, so this is sound. What is *not* surfaced is invoking the
  parent's same-named method from within the child — there is no
  `super.foo()` analogue in Rust. Reach the parent via
  `self.parent.borrow().foo()` instead.
* **No transparent `Deref<Target = Parent>`.** Because the parent field
  is wrapped in `Parent<T>` (which owns a dynamically-borrowed cell),
  parent access always goes through `.borrow()` / `.borrow_mut()`.
* **Consuming-`self` parent methods fail at runtime on child instances.**
  The child's inner `Rc` clone means the parent's `try_unwrap` can't
  succeed, so any parent method taking `self` will throw when dispatched
  on a child. Use `&self` / `&mut self` parent methods.
* **Parent must have a user-defined `#[wasm_bindgen(constructor)]`** if
  it's going to be extended — subclass construction calls `super(...)`
  with a module-level sentinel that short-circuits the parent's ctor
  body, but the parent's ctor must exist for the `super` call to be
  legal JS.
* **No generics** on a struct that uses `extends`.
* **Named-field structs only.** Tuple structs and unit structs cannot use
  `extends` — the macro needs somewhere to inject the `parent` field.

[2]: ../on-js-imports/extends.html
