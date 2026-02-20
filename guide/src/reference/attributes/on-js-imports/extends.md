# `extends = Class`

The `extends` attribute can be used to say that an imported type extends (in the
JS class hierarchy sense) another type. This will generate `AsRef`, `From`, 
`Deref`, and `Upcast` impls for converting a type into another given that we 
statically know the inheritance hierarchy:

```rust
#[wasm_bindgen]
extern "C" {
    type Foo;

    #[wasm_bindgen(extends = Foo)]
    type Bar;
}

let x: &Bar = ...;
let y: &Foo = x.as_ref(); // zero cost cast via AsRef
let z: Foo = x.clone().upcast(); // zero cost cast via Upcast
```

The trait implementations generated for the above block are:

```rust
impl From<Bar> for Foo { ... }
impl AsRef<Foo> for Bar { ... }
impl Deref for Bar { type Target = Foo; ... }
impl Upcast<Foo> for Bar { }
```

The first `extends` target is used as the `Deref` target. To disable `Deref` 
generation, use the [`no_deref`](./no_deref.md) attribute. To disable `Upcast`
generation, use the [`no_upcast`](./no_upcast.md) attribute.

The `extends = ...` attribute can be specified multiple times for longer
inheritance chains, and `AsRef`, `Upcast`, etc. impls will be generated for 
each of the types.

```rust
#[wasm_bindgen]
extern "C" {
    type Foo;

    #[wasm_bindgen(extends = Foo)]
    type Bar;

    #[wasm_bindgen(extends = Foo, extends = Bar)]
    type Baz;
}

let x: &Baz = ...;
let y1: &Bar = x.as_ref();
let y2: &Foo = y1.as_ref();

// Or using upcast:
let baz: Baz = ...;
let bar: Bar = baz.clone().upcast_into();
let foo: &Foo = baz.upcast();
```

> The `upcast()` method is also used when working with generic types. For example, when passing an `Array<Number>` to a function expecting `&Array<JsValue>`, use `my_array.upcast()`. See [Working with Generics](../../working-with-generics.md#upcasting-types) for more details.
