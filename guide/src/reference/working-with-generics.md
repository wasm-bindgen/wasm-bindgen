# Working with wasm-bindgen Generics

This guide provides an overview of wasm-bindgen's generic type system, which brings TypeScript-like type safety to Rust-JavaScript interop.

For a complete reference of all generic types and traits, see [js-sys](./types/js-sys.md).

## Table of Contents

- [Overview](#overview)
- [Example](#example)
- [Defining Generic Import Types](#defining-generic-import-types)
  - [Generic Type Constraints](#generic-type-constraints)
- [The ErasableGeneric Trait](#the-erasablegeneric-trait)
  - [JsGeneric Trait](#jsgeneric-trait)
- [Upcasting Types](#upcasting-types)
  - [Automatic Upcast Generation](#automatic-upcast-generation)
  - [Upcast Rules](#upcast-rules)
- [Exporting Functions with Generic Types](#exporting-functions-with-generic-types)

---

## Overview

Wasm-bindgen generics use **type erasure** to provide rich typing information in Rust while generating efficient JavaScript bindings. Generic parameters exist only in Rust code—they are completely erased in the generated JavaScript.

Currently, all of `js-sys` now supports experimental type erased generics, with `web-sys` still pending.

> When passing a typed value (e.g., `Array<Number>`) to a function expecting a wider type (e.g., `&Array<JsValue>`), use the [`upcast()`](#upcasting-types) method: `my_array.upcast()`. This is a zero-cost compile-time conversion.

## Example

Consider importing a JS function that returns a Promise for an array of strings. This can be implemented using the typed `Promise<T>` and typed `Array<T>` in js-sys:

```rust
#[wasm_bindgen]
extern "C" {
    fn getPromiseArray() -> Promise<Array<JsString>>;
}
```

Now, when awaiting the future from the Promise, the types are directly inferred without any further type hints necessary:

```rust
let promise_array = getPromiseArray();

// inferred as Array<JsString>
let array = JsFuture::from(promise_array).await.unwrap();
for js_string in array {
    // inferred as a JsString already
    let string: String = js_string.into();
}
```

## Defining Generic Import Types

Generic types can be defined for all imported JavaScript types.

For example, to define a linked list you might type a `ListNode` type with the following definition:

```rust
#[wasm_bindgen]
extern "C" {
    pub type ListNode<T>;

    #[wasm_bindgen(constructor)]
    pub fn new<T>(value: T) -> ListNode<T>;

    #[wasm_bindgen(method, getter)]
    pub fn value<T>(this: &ListNode<T>) -> T;

    #[wasm_bindgen(method, getter)]
    pub fn next<T>(this: &ListNode<T>) -> Option<ListNode<T>>;

    #[wasm_bindgen(method, setter)]
    pub fn set_next<T>(this: &ListNode<T>, next: Option<ListNode<T>>);
}
```

Usage with concrete types:

```rust
// Constructor infers ListNode type
let node1 = ListNode::new(JsString::from("first"));
let node2 = ListNode::new(JsString::from("second"));
node1.set_next(Some(node2));

let value = node1.value(); // Returns JsString
let next_node = node1.next(); // Returns Option<ListNode<JsString>>
```

Trait bounds and lifetimes are also supported in generics definitions. The generic parameters and predicates are hoisted out of the extern "C" block to form the generics of the Rust wrapper around the JS bindgen function.

### Generic Type Constraints

Imported types and imported functions support the full Rust type constraint system.

Traits can also be used to form classes of JS types, for example in js-sys we have:

- `Iterable`, `AsyncIterable`: Traits for objects implementing the iterator protocol via `[Symbol.iterator]()` or `[Symbol.asyncIterator]()`. Uses an associated `Item` type.
- `Promising`: Trait for types that are either `T` or `Promise<T>`. Uses an associated `Resolution` type.

These can be used to constrain function generics, for example a function taking an iterable or a promising value can be written:

```rust
use js_sys::{Iterable, Promising, Promise, Number};

#[wasm_bindgen]
extern "C" {
    // Takes anything iterable (Array, Set, Generator, etc.)
    fn processIterable<T: Iterable<Item = Number>>(items: T) -> f64;

    // Takes either a Number or Promise<Number>
    fn getValue<T: Promising<Resolution = Number>>(value: T) -> Promise<Number>;
}
```

## The ErasableGeneric Trait

Generic parameters are erased during compilation, effectively turning `Promise<Array<JsString>>` into a singular `JsValue` for the Wasm Bindgen ABI binding generation, and similarly for values received in turn from JS.

Not all types erase into `JsValue` though, other types like Rust closures, or Rust `Option<Promise>` will erase into their corresponding Rust type with the JS erasure as `Option<JsValue>`.

To capture these semantics, generics are implemented on the `ErasableGeneric` trait. This trait has an associated `Repr` type which is the concrete erased type of the generic for binding generation.

Because all js-sys types implement `ErasableGeneric`, the type system can validate that `Promise<Array<JsString>>::Repr` is equal to `Promise<JsValue>::Repr` ensuring that the static concrete binding generation to works out soundly.

The default erasable generic uses `Repr = JsValue` as the most common case, while `Option<T>` uses `Option<T::Repr>` and `Result<T, E>` similarly uses `Result<T::Repr, E::Repr>`.

`ErasableGeneric<Repr = T>` may be used in trait bounds for generic code where necessary, but implementing it is an unsafe Wasm Bindgen internal without stability guarantees.

### JsGeneric Trait

For the common case, it is recommended to use the `JsGeneric` trait bound when needed in generic functions - this is a shorthand for `ErasableGeneric<Repr = JsValue>` and is the primary generic repr used for JavaScript typing.

**Types that implement `JsGeneric`:**

- All js-sys types: `Object`, `Array`, `Function`, `Promise`, `Map`, `Set`, etc.
- JS primitives: `JsValue`, `Number`, `BigInt`, `Boolean`, `JsString`, `Symbol`
- JS special values: `Undefined`, `Null`
- Wrapper types: `JsOption<T>` (for any `T: JsGeneric`)
- All web-sys generated types
- Custom types imported via `#[wasm_bindgen]` automatically generate this trait on the `JsValue` repr.

**Types that do NOT implement `JsGeneric`:**

- Rust primitives: `u32`, `i32`, `f64`, `bool`, `char`, `String`, etc.

This means you cannot use Rust primitives directly as generic parameters where `JsGeneric` is required. For example, `Array<u32>` is not valid—use `Array<Number>` instead.

```rust
use wasm_bindgen::convert::JsGeneric;
use js_sys::{Array, Number};

fn process_js_values<T: JsGeneric>(items: &Array<T>) {
    // T can be Number, JsString, Object, etc. — but not u32 or String
}

// ✓ Works
let numbers: Array<Number> = Array::new_typed();
process_js_values(&numbers);

// ✗ Won't compile — u32 is not JsGeneric
// let numbers: Array<u32> = ...;
```

## Upcasting Types

Generic JS types support type-safe upcasting through the `Upcast<Target>` trait, implemented using `FromUpcast<Source>` (just like `From` and `Into`). This enables widening from specific to general types via the `upcast()` and `upcast_into()` methods.

`upcast()` can be thought of as providing the TypeScript-like type safety on top of the erasable generics type system - it will not allow converting from a wider type to a narrower type and in the process providing type safety mechanisms at compile time through the Rust compiler.

This includes Function typing with covariance for return types, closure generics and all other container generic types, all implemented on the `FromUpcast<Source>` covariance primitive.

```rust
use js_sys::{Array, Number, Object};

// Number → JsValue
let num = Number::from(42);
let js_value: JsValue = num.upcast_into();

// Array<Number> → Array<JsValue>
let num_array: Array<Number> = Array::new_typed();
let js_array: Array<JsValue> = num_array.upcast_into();

// Array<Number> → Object (inheritance)
let obj: Object = num_array.upcast_into();
```

### Automatic Upcast Generation

Upcast implementations are automatically generated for all imported JavaScript types based on their `extends` attribute:

```rust
use wasm_bindgen::prelude::*;
use js_sys::Object;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type MyCustomType;

    #[wasm_bindgen(extends = Object)]
    pub type MyCollection<T>;
}

// Upcast implementations are automatically generated:
let my_type = MyCustomType::new();
let obj: &Object = my_type.upcast(); // ✓ Object upcast by ref (from extends)
let js_val: JsValue = my_type.clone().upcast_into(); // ✓ JsValue upcast by value (always generated)
```

Upcasts are always provided both for ref and value conversions.

The following Upcast implementations are automatically generated:

- **JsValue upcast**: All types → `JsValue`
- **Identity upcast**: Non-generic types → themselves  
- **Structural upcast**: `Container<T>` → `Container<U>` when `T: Upcast<U>`
- **Inheritance upcast**: For each type in the `extends` attribute

To disable automatic Upcast generation (e.g., for types with custom implementations), use the `no_upcast` attribute:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, no_upcast)]
    pub type MyCustomType;
}
```

### Upcast Rules

1. **JsValue upcast**: All types can upcast to `JsValue`
2. **Identity upcast**: Non-generic types can upcast to themselves
3. **Inheritance upcast**: Types upcast to their `extends` targets (e.g., Object)
4. **Structural upcast**: Generic types like `Array<T>` → `Array<U>` when `T` → `U`
5. **Nested upcast**: `Promise<Array<Number>>` → `Promise<Array<JsValue>>`

## Exporting Functions with Generic Types

Exported functions support generic types with **concrete** type parameters only.

```rust
#[wasm_bindgen]
pub fn create_number_array() -> Array<Number> {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr
}

#[wasm_bindgen]
pub fn sum_numbers(arr: Array<Number>) -> f64 {
    arr.iter().map(|n| n.value_of()).sum()
}

// ✗ Generic function parameters not supported:
// pub fn create_array<T>(value: T) -> Array<T> { ... }
```

**Note:** TypeScript generation does not currently support generic types. Exports will appear as `Promise<any>`, `Array<any>`, etc. in generated `.d.ts` files.
