# Generic JavaScript Types

Wasm Bindgen supports generic typing using type erasure for imported JavaScript types.

This allows annotating JavaScript's dynamically typed values with corresponding type parameters in Rust to obtain richer typing information when interfacing with JavaScript from Rust.

The generic parameters exist only in Rust code—they are completely erased in the generated JavaScript bindings.

Currently, all of `js-sys` now supports experimental type erased generics, with `web-sys` still pending.

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

## js-sys Generic Types

The `js-sys` crate provides generic versions of JavaScript built-in types, including:

* `Array<T>`: Typed Array builtin.
* `ArrayTuple<T1, ..., T9>`: JS Array builtin variant with typing per item up to 9 items.
* `Function<Return, A1, ..., A9>`: JS function external type, with a typed return and up to 9 arguments.
* `Generator<T>` and `AsyncGenerator<T>`: Typed generator builtin.
* `Map<K, V>`: Typed map builtin.
* `Iterator<T>`: Typed iterators.
* `AsyncIterator<T>`: Typed async iterators.
* `Object<T>`: Typed object records.
* `Reflect`: Provides generic operators.
* `Set<T>`: Typed sets.
* `WeakMap<K, V>`: Weak maps.
* `WeakSet<T>`: Typed weak sets.
* `WeakRef<T>`: Typed weak refs.
* `Promise<T>`: Typed promises.

All of the above types default to the non-generic form of `JsValue` parameters when generic parameters are unspecified.

### Examples

```rust
use js_sys::{Array, Map, Set, Promise, Number, JsString};

// Create typed collections
let numbers: Array<Number> = Array::new_typed();
numbers.push(&Number::from(42));

let map: Map<JsString, Number> = Map::new_typed();
map.set(&JsString::from("key"), &Number::from(100));

let set: Set<JsString> = Set::new_typed();
set.add(&JsString::from("value"));

// Work with promises
let promise: Promise<Number> = Promise::resolve(&Number::from(42));
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

## Exporting Functions using Generic Types

Exported functions can use generic types as long as they use concrete type parameters.

```rust
use js_sys::{Promise, Array, Number, JsString};

// ✓ Works: concrete type parameters
#[wasm_bindgen]
pub fn create_number_promise(value: f64) -> Promise<Number> {
    Promise::resolve(&Number::from(value))
}

#[wasm_bindgen]
pub fn create_string_array() -> Array<JsString> {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("hello"));
    arr
}

#[wasm_bindgen]
pub fn process_number_array(arr: Array<Number>) -> f64 {
    let mut sum = 0.0;
    for i in 0..arr.length() {
        sum += arr.get(i).value_of();
    }
    sum
}

// ✗ Doesn't work: generic function parameters
// #[wasm_bindgen]
// pub fn create_promise<T>(value: T) -> Promise<T> { ... }
```

TypeScript generation support for generic types is not currently supported, so these
generic types will still show up as `Promise<any>` etc in TypeScript.

## Generic Type Constraints

Imported types and imported functions support the full Rust type constraint system.

Traits can also be used to form classes of JS types, for example in js-sys we have:

* `Iterable<T>`, `AsyncIterable<T>`: An object with a `[Symbol.async?iterator]()` function field implementing the iterator protocol.
* `Promising<T>`: Either `T` or `Promise<T>`.

These can be used to constraint function generics, for example a function taking an iterable or a promising value can be written:

```rust
use js_sys::{Iterable, Promising, Promise, Number};

#[wasm_bindgen]
extern "C" {
    // Takes anything iterable (Array, Set, Generator, etc.)
    fn processIterable<T: Iterable<Number>>(items: T) -> f64;

    // Takes either a Number or Promise<Number>
    fn getValue<T: Promising<Number>>(value: T) -> Promise<Number>;
}
```

## The ErasableGeneric Trait

Generic parameters are erased during compilation, effectively turning `Promise<Array<JsString>>` into a singular `Promise<JsValue>` function for the Wasm Bindgen ABI binding generation, and similarly for values received in turn from JS.

Generics are implemented on the `ErasableGeneric` trait. This trait has an associated `Repr` type which is the concrete type of the generic for binding.

Because all js-sys types implement `ErasableGeneric`, the type system can validate that `Promise<Array<JsString>>::Repr` is equal to `Promise<JsValue>::Repr` ensuring that the static concrete binding generation to works out soundly.

While the default erasable generic uses `Repr = JsValue`, erasable generics don't only apply to `ErasableGeneric<Repr = JsValue>`, they also apply to other Rust types that retain equivalent ABI. For example `Option<T>` and `Result<T, E>` also
implement `ErasableGeneric` and so does `&mut dyn FnMut(T) -> T` for `T` itself as `ErasableGeneric`. This allows for example functions like `arr.forEach` to work for arbitrary `T` values.

While uncommon, alternative generic reprs can be defined for generic parameters of imported types and functions by providing a default value with an alternative `Repr` value.

`ErasableGeneric<Repr = T>` may be used in trait bounds for generic code where necessary, but implementing it is an unsafe Wasm Bindgen internal without stability guarantees.
