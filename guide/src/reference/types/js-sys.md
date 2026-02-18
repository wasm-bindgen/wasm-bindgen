# `js-sys`

The `js-sys` crate provides bindings to all JavaScript global APIs guaranteed to exist in every JavaScript environment by the ECMAScript standard.

Wasm-bindgen supports generic typing using type erasure for imported JavaScript types. Generic parameters exist only in Rust code—they are completely erased in the generated JavaScript bindings.

For a conceptual overview and usage guide, see [Working with Generics](../working-with-generics.md).

> When passing a typed value (e.g., `Array<Number>`) to a function expecting a wider type (e.g., `&Array<JsValue>`), use the `upcast()` method: `my_array.upcast()`. See [Upcasting Types](../working-with-generics.md#upcasting-types) for details.

All generic types listed implement `JsGeneric` and default to `JsValue` when type parameters are unspecified.

| Type | Description |
|------|-------------|
| [`Array<T>`](#arrayt) | Typed JavaScript array |
| [`ArrayTuple<T1, ..., T8>`](#arraytuplet1--t8) | Array with per-item typing (up to 8) |
| [`Function<fn(A) -> R>`](#functionfna---r) | Typed function with return and arguments |
| [`Promise<T>`](#promiset) | Promise resolving to `T` |
| [`Map<K, V>`](#mapk-v) | Typed map |
| [`Set<T>`](#sett) | Typed set |
| [`Iterator<T>` / `AsyncIterator<T>`](#iteratort-and-asynciteratort) | Typed iterators |
| [`Generator<T>` / `AsyncGenerator<T>`](#generatort-and-asyncgeneratort) | Typed generators |
| [`Object<T>`](#objectt) | Object with typed values |
| [`WeakMap<K, V>` / `WeakSet<T>` / `WeakRef<T>`](#weakmapk-v-weaksett-weakreft) | Weak collections |
| [`JsOption<T>`](#jsoptiont) | Nullable JS value (`T`, `null`, or `undefined`) |
| `Number` | JavaScript number primitive |
| `JsString` | JavaScript string primitive |
| `Boolean` | JavaScript boolean primitive |
| `BigInt` | JavaScript BigInt primitive |
| `Symbol` | JavaScript Symbol primitive |
| `Undefined` | JavaScript `undefined` value |
| `Null` | JavaScript `null` value |

---

## Array\<T\>

Typed JavaScript `Array` builtin.

```rust
use js_sys::{Array, Number, JsString};

let numbers: Array<Number> = Array::new_typed();
numbers.push(&Number::from(42));

let first: Number = numbers.get(0);

for num in numbers.iter() {
    // num is Number
}
```

## ArrayTuple\<T1, ..., T8\>

JavaScript `Array` with per-item typing, supporting up to 8 items.

```rust
use js_sys::{ArrayTuple, Number, JsString, Boolean};

let tuple: ArrayTuple<Number, JsString, Boolean> = ArrayTuple::new();
tuple.set0(&Number::from(1));
tuple.set1(&JsString::from("hello"));
tuple.set2(&Boolean::from(true));

let n: Number = tuple.get0();
let s: JsString = tuple.get1();
let b: Boolean = tuple.get2();
```

## Function\<fn(A) -> R\>

Typed JavaScript function with return type and up to 8 arguments.

```rust
use js_sys::{Function, Number, JsString};

#[wasm_bindgen]
extern "C" {
    fn getFormatter() -> Function<fn(Number) -> JsString>;
}

let formatter = getFormatter();
let result: JsString = formatter.call1(&JsValue::UNDEFINED, &Number::from(42))?;
```

### Type aliases

| Alias | Description |
| ----- | ----------- |
| `TypedFunction` | Strict arity checking (arguments default to `None`) |
| `AnyFunction` | Lenient behavior (arguments default to `JsValue`) |
| `VoidFunction` | `Function<fn(...) -> Undefined>` with strict arity |

### Calling with tuples

```rust
let f: Function<fn(JsString, Boolean) -> Number> = get_function();

// call accepts a tuple of references
let result = f.call(&context, ())?;                      // no args
let result = f.call(&context, (&my_string,))?;           // one arg
let result = f.call(&context, (&my_string, &my_bool))?;  // two args

// bindn returns a new function with arguments pre-bound
let bound: Function<fn(Boolean) -> Number> = f.bindn(&context, (&my_string,));

// Argument upcasting: pass subtypes where supertypes are expected
let f: Function<fn(JsValue, JsValue) -> Number> = get_function();
let result = f.call(&context, (&my_number, &my_string))?;  // Number, JsString upcast to JsValue
```

### Converting from Closures

Use `Function::from_closure()` to convert owned `Closure` types to typed `Function`, and `Function::closure_ref()` for borrowed closures:

```rust
use js_sys::{Function, Number, JsString};
use wasm_bindgen::prelude::*;

// Owned closure to Function (transfers ownership to JS)
let closure: Closure<dyn FnMut(Number) -> JsString> = Closure::new(|n: Number| {
    JsString::from(format!("Value: {}", n.value_of()))
});
let func: Function<fn(Number) -> JsString> = Function::from_closure(closure);
```

For borrowed closures, use `closure_ref`:

```rust
let mut count = 0u32;
let mut increment = || {
    count += 1;
    Number::from(count)
};
let closure = ScopedClosure::borrow_mut(&mut increment);
let func: &Function<fn() -> Number> = Function::closure_ref(&closure);
```

### Primitive type coercion

Rust primitives automatically coerce to JS types in closure return positions:

```rust
// u32 coerces to Number
let closure: Closure<dyn Fn() -> u32> = Closure::new(|| 42);
let func: Function<fn() -> Number> = Function::from_closure(closure);

// String coerces to JsString
let closure: Closure<dyn Fn() -> String> = Closure::new(|| "hello".to_string());
let func: Function<fn() -> JsString> = Function::from_closure(closure);
```

This works for all numeric primitives (`u8`, `i32`, `f64`, etc.) and string types (`String`, `&str`, `char`).

See [Passing Rust Closures to JS](../passing-rust-closures-to-js.md) for more details on closure types and lifecycle management.

## Promise\<T\>

Typed JavaScript `Promise` that resolves to `T`.

```rust
use js_sys::{Promise, Number};

#[wasm_bindgen]
extern "C" {
    fn fetchCount() -> Promise<Number>;
}

let count: Number = JsFuture::from(fetchCount()).await?;
```

### The Promising trait

The `Promising` trait represents types that are either `T` or `Promise<T>`. Use it to accept both immediate values and promises in function signatures:

```rust
use js_sys::{Promising, Promise, Number};

#[wasm_bindgen]
extern "C" {
    // Accepts either Number or Promise<Number>
    fn getValue<T: Promising<Resolution = Number>>(value: T) -> Promise<Number>;
}
```

## Map\<K, V\>

Typed JavaScript `Map` builtin.

```rust
use js_sys::{Map, JsString, Number};

let map: Map<JsString, Number> = Map::new_typed();
map.set(&JsString::from("key"), &Number::from(100));

let value: Number = map.get(&JsString::from("key"));
```

## Set\<T\>

Typed JavaScript `Set` builtin.

```rust
use js_sys::{Set, JsString};

let set: Set<JsString> = Set::new_typed();
set.add(&JsString::from("value"));

let has: bool = set.has(&JsString::from("value"));
```

## Iterator\<T\> and AsyncIterator\<T\>

Typed iterators for synchronous and asynchronous iteration.

```rust
use js_sys::{Iterator, Number};

#[wasm_bindgen]
extern "C" {
    fn getNumberIterator() -> Iterator<Number>;
}

let iter = getNumberIterator();
while let Some(num) = iter.next() {
    // num is Number
}
```

### The Iterable and AsyncIterable traits

The `Iterable` and `AsyncIterable` traits represent objects implementing the iterator protocol via `[Symbol.iterator]()` or `[Symbol.asyncIterator]()`. Use them to accept any iterable type in function signatures:

```rust
use js_sys::{Iterable, AsyncIterable, Number};

#[wasm_bindgen]
extern "C" {
    // Accepts Array, Set, Generator, or any other iterable
    fn processIterable<T: Iterable<Item = Number>>(items: T);
    
    // Accepts AsyncGenerator or any other async iterable
    fn processAsyncIterable<T: AsyncIterable<Item = Number>>(items: T);
}
```

## Generator\<T\> and AsyncGenerator\<T\>

Typed generator builtins.

```rust
use js_sys::{Generator, Number};

#[wasm_bindgen]
extern "C" {
    fn createGenerator() -> Generator<Number>;
}

let gen = createGenerator();
let result = gen.next();
```

## Object\<T\>

Typed object records where values are of type `T`.

```rust
use js_sys::{Object, Number};

let obj: Object<Number> = Object::new_typed();
```

## WeakMap\<K, V\>, WeakSet\<T\>, WeakRef\<T\>

Typed weak collection builtins.

```rust
use js_sys::{WeakMap, WeakSet, WeakRef, Object, Number};

let weak_map: WeakMap<Object, Number> = WeakMap::new_typed();
let weak_set: WeakSet<Object> = WeakSet::new_typed();
let weak_ref: WeakRef<Object> = WeakRef::new(&my_object);
```

## JsOption\<T\>

Represents a JS value that may be `T`, `null`, or `undefined`.

```rust
use wasm_bindgen::JsOption;
use js_sys::Number;

#[wasm_bindgen]
extern "C" {
    fn maybeGetValue() -> JsOption<Number>;
}

let value = maybeGetValue();

// Check emptiness
if value.is_empty() {
    // null or undefined
}

// Convert to Option
match value.into_option() {
    Some(num) => { /* use num */ }
    None => { /* handle null */ }
}

// Unwrap methods
let num = value.unwrap();
let num = value.expect("should exist");
let num = value.unwrap_or_default();
let num = value.unwrap_or_else(|| Number::from(0));

// Create values
let with_value = JsOption::wrap(Number::from(42));
let empty: JsOption<Number> = JsOption::new();
let from_opt = JsOption::from_option(Some(Number::from(42)));
```

**`JsOption<T>` vs `Option<T>`:**

| Type | Behavior |
| ---- | -------- |
| `Option<T>` | Undefined or Null check at ABI boundary, immediately converts to `Some(T)` or `None` |
| `JsOption<T>` | Defers undefined or null check, works in `JsGeneric` positions |
