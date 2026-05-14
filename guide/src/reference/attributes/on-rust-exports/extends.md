# `extends = Parent`

The `extends` attribute on an exported Rust struct declares that the struct
inherits from another exported Rust struct. This produces a JS class with a
real prototype chain (`class Child extends Parent`), so `instanceof Parent`
is true for every `Child` instance, and parent methods dispatched via the JS
prototype chain land on the correct parent value at runtime — each instance
holds an independent reference to each ancestor.

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
rex.name();            // "Rex" — Animal::name dispatched via the prototype chain
```

## The injected `parent` field

The macro injects a hidden `parent` field on any struct that uses
`#[wasm_bindgen(extends = Parent)]`. You never declare it yourself — it's
typed as `wasm_bindgen::Parent<Parent>` (a refcounted cell around the parent
data) and is visible to your own `impl` blocks as `self.parent`.

Initialize it in the constructor with the ergonomic `.into()` (which calls
the [`From<T>`] impl on `Parent<T>`):

```rust
Dog { parent: Animal::new(name).into(), breed }
```

If you prefer to be explicit, the equivalent is `wasm_bindgen::Parent::new(...)`
— but `.into()` keeps the constructor body free of `wasm_bindgen` imports.

[`From<T>`]: https://doc.rust-lang.org/std/convert/trait.From.html

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
child, so generic Rust code can accept any **direct** child where it
expects a borrowed reference to the parent's `Parent<…>` cell:

```rust
fn animal_name<T: AsRef<wasm_bindgen::Parent<Animal>>>(t: &T) -> String {
    t.as_ref().borrow().name()
}
animal_name(&dog); // "Rex"
```

The `AsRef` impl is direct-parent only — for a chain
`Animal <- Dog <- Puppy`, `Puppy: AsRef<Parent<Dog>>` is emitted but
`Puppy: AsRef<Parent<Animal>>` is not. Reaching `Animal` from a `Puppy`
goes *through* the `Dog` cell, which means opening a runtime borrow on
the `Dog` cell to read its `parent: Parent<Animal>` field. That borrow
has to be held for the entire time the `&Animal` is in use, but
`AsRef::as_ref(&self) -> &Target` returns a bare reference and gives the
caller no place to keep that guard alive. So the chain has to be walked
explicitly by the caller:

```rust
puppy.parent.borrow().parent.borrow().name()
```

Each `.borrow()` produces its own guard at the call site.

## Reaching the parent from Rust

JS callers see inherited methods via the prototype chain — `dog.name()`
just works without any extra code on `Dog`. Rust callers are different:
parent methods are *not* auto-forwarded onto the child type. If your Rust
code holds a `&Dog` and wants to call an inherited method, go through
the parent borrow:

```rust
fn describe(dog: &Dog) -> String {
    dog.parent.borrow().name()
}
```

If you want to expose a wrapped variant of an inherited method on the
**child's** JS class — for instance, to rename it or add behaviour — write
a one-line forwarder. (This shadows the parent's same-named method on the
child class; pure-prototype-chain inheritance still works for any method
you don't shadow.)

```rust
#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(js_name = describe)]
    pub fn describe(&self) -> String {
        format!("{} is a dog", self.parent.borrow().name())
    }
}
```

## How inheritance works at runtime

For every class in an `extends` chain, each JS instance carries one
pointer per ancestor:

```
dog.__wbg_ptr_Dog     // pointer to a Rc<WasmRefCell<Dog>>
dog.__wbg_ptr_Animal  // pointer to a Rc<WasmRefCell<Animal>>
```

The two are independent allocations. The `Animal` cell is reached from JS
through `__wbg_ptr_Animal` and from Rust through `dog.parent`; both are
clones of the same `Rc` and share its strong count.

Each exported method reads from the per-class field that matches the class
where it was defined. So `Animal.prototype.name`, when called on a `Dog`
instance via the prototype chain, passes `this.__wbg_ptr_Animal` to the
wasm `Animal::name` shim — the correct pointer type. Child-defined methods
read from the child's own per-class field.

On `dog.free()` (or garbage collection via the `FinalizationRegistry`),
every per-class pointer is released. Each release decrements one strong
count on its `Rc`; the cell is freed when the last clone is dropped. So
calling `dog.free()` while some other JS reference still holds the parent
pointer (via the prototype chain or otherwise) frees the `Dog` cell but
keeps the `Animal` cell alive until that other reference is also released.

## Extending a renamed or namespaced parent

When the parent struct sets [`js_name`](js_name.html) and/or
[`js_namespace`](js_namespace.html), the child must additionally
declare the parent's JS identity via `extends_js_class` and
`extends_js_namespace`. The child's macro is a separate procedural-
macro invocation from the parent's and cannot see the parent's
attributes, so the JS-side identity must be redeclared locally:

```rust
#[wasm_bindgen(js_name = Animal, js_namespace = zoo)]
pub struct AnimalImpl { /* ... */ }

#[wasm_bindgen(js_class = Animal, js_namespace = zoo)]
impl AnimalImpl { /* ... */ }

#[wasm_bindgen(
    extends = AnimalImpl,
    extends_js_class = "Animal",
    extends_js_namespace = zoo,
)]
pub struct DogImpl { /* ... */ }
```

Both attributes default sensibly:

* `extends_js_class` defaults to the last segment of the `extends` Rust
  path. This means the **no-rename** case (parent has no `js_name`)
  needs no extra ceremony — `extends = Animal` alone resolves correctly
  when the parent's JS name is also `Animal`.
* `extends_js_namespace` is only required when the parent uses `js_namespace`.

Diagnostics are provided if the class is not matched at code generation time.

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
* **No transparent `Deref<Target = Parent>`.** Reading the parent value
  requires holding an open runtime borrow on the cell that contains it
  (which is what makes parent methods re-entrancy-safe across JS
  callbacks). `Deref::deref` returns a bare `&Parent` and gives the
  caller no place to keep that borrow alive, so the macro doesn't emit
  it. Take the borrow yourself with `self.parent.borrow()` /
  `self.parent.borrow_mut()`.
* **Consuming-`self` parent methods fail at runtime on child instances.**
  Invoking a `self`-by-value parent method on a Rust descendant via the
  JS prototype chain throws a `TypeError` before the call reaches wasm
  — the generated JS glue rejects the cross-class dispatch. (Even
  without that guard, the parent's wasm shim would fail at
  `Rc::try_unwrap` because the JS-held parent reference keeps the
  refcount above 1.) Use `&self` / `&mut self` parent methods instead;
  they dispatch correctly on descendants.
* **Parent must have a user-defined `#[wasm_bindgen(constructor)]`** if
  it's going to be extended. Subclass construction calls `super(...)`
  with a module-level sentinel that short-circuits the parent's ctor
  body, but the parent's ctor must *exist* for the `super` call to be
  legal JS. Without it, in debug builds you'll get a runtime
  `cannot invoke 'new' directly` from the parent's auto-generated
  default constructor.
* **No generics** on a struct that uses `extends`.
* **Tuple structs are rejected.** A tuple struct has no field-name slot
  for the macro to inject `parent` into. Unit structs are accepted —
  the macro converts them to named-field structs containing only the
  injected `parent`.

[2]: ../on-js-imports/extends.html
