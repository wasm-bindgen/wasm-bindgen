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
| [`JsOption<T>`](#jsoptiont) | Optional JS value (`T` or `undefined`) |
| [`JsNullable<T>`](#jsnullablet) | Nullable JS value (`T` or `null`) |
| `Number` | JavaScript number primitive |
| `JsString` | JavaScript string primitive |
| `Boolean` | JavaScript boolean primitive |
| `BigInt` | JavaScript BigInt primitive |
| `Symbol` | JavaScript Symbol primitive |
| `Undefined` | JavaScript `undefined` value |
| `Null` | JavaScript `null` value |

### Rust element types (unstable)

Under `--cfg=js_sys_unstable_apis`, the element APIs of `Array<T>`,
`Map<K, V>`, `Set<T>`, `Iterator<T>`, `AsyncIterator<T>` and `Promise<T>`
are [`experimental_generic_mono`](../attributes/on-js-imports/experimental_generic_mono.md)
imports, so `T` can be any type with an ABI, such as `String`, `bool` or
`f64`, crossing as itself:

```rust
use js_sys::{Array, Map, Promise};

let a: Array<String> = ["a", "b"].into_iter().map(String::from).collect();
a.push(&"c".into());
assert_eq!(a.get(0).as_deref(), Some("a"));
assert_eq!(a.iter().collect::<Vec<String>>(), ["a", "b", "c"]);

let m: Map<String, f64> = Map::new_typed();
m.set(&"k".into(), &1.0);
assert_eq!(m.get("k"), Some(1.0));

let v: String = Promise::resolve("x").await?;
```

The collections also behave as Rust collections under the cfg.
`Map::get`, `has` and `delete` and `Set::has` and `delete` take any borrowed
form of the key, as `HashMap` does, so a `Map<String, V>` is read with `"k"`.
`Map::get` is `Some` for every present key, even one whose value is
`undefined` or `null`, so a `Map<String, Option<String>>` reads as
`Option<Option<String>>`. An `Iterator<T>` iterates as `T`, rethrowing a
throwing `next()` into JS as `for...of` does, while `try_iter` and
`into_try_iter` keep the protocol's errors as `Err` items. `Map`, `Set` and
`Object<T>` have `iter()`, `IntoIterator`, `FromIterator` and `Extend`, `Map`
and `Object<T>` over `(key, value)` tuples (an `Object<T>` record over its
own enumerable string keys), and `Entries<K, V>` turns any `[key, value]`
iterator into tuples:

```rust
use js_sys::{Map, Object, Set};

let m: Map<String, f64> = [("a".to_string(), 1.0)].into_iter().collect();
for (key, value) in &m {
    // key: String, value: f64
}

let s: Set<u32> = [3, 1, 3].into_iter().collect();
assert!(s.has(&3));

let record: Object<String> = hash_map.iter().collect();
let pairs: Vec<(String, String)> = record.iter().collect();
```

Reads bound the element by `FromWasmAbi`, and writes take `&T` bounded by
`T: RefIntoWasmAbi`, which holds for JS types, `String` and every primitive.
Borrows deref-coerce to the element type, so
`Array<JsValue>::push(&js_string)` still works. Each element type gets its own
JS shim. Generic code over Rust and JS element types alike states the bound
each position needs, adding `Option<T>: FromWasmAbi` for nullable reads.
`JsGeneric` implies all of these, so generic code bounded by `T: JsGeneric`
can use the element APIs as before.

The same holds for `Function` signatures, `ArrayTuple` elements, `WeakMap`
and `DisposableStack` values and element callbacks, such as those of
`Array::map`, `Iterator::map`, `Map::group_by` and `Promise::then`:

```rust
use js_sys::{Array, ArrayTuple, Function};

let f: Function<fn(String, u32) -> String> =
    Function::new_with_args_typed("s, n", "return s.repeat(n)");
assert_eq!(f.call(&JsValue::NULL, (&"ab".into(), &2))?, "abab");

let a: Array<String> = ["a", "bb"].into_iter().map(String::from).collect();
let lengths: Array<u32> = a.map(&mut |s: String, _, _| s.len() as u32);
let pair: ArrayTuple<(String, u32)> = ArrayTuple::new2(&"a".into(), &1);
assert_eq!(pair.into_tuple(), ("a".to_string(), 1));
```

A `Function<fn(..)>` slot may be any type. Calling or binding it needs each
argument to be `RefIntoWasmAbi` and the return to be `FromWasmAbi`. A generic
callback return in a `Result`, as in `Promise::then` or `Array::try_map`, is
bounded by `Result<R, JsError>: ReturnWasmAbi`. `R: JsGeneric` implies it,
while other generic callers state it themselves.

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

// or through `From`
let func: Function<fn(Number) -> JsString> = closure.into();
```

Under `--cfg=js_sys_unstable_apis`, `Function::from_fn` builds a typed
`Function` straight from a Rust closure, whose argument types are inferred
from the signature, and `Function::once` does the same for an `FnOnce`, which
throws if JS calls it a second time:

```rust
let f: Function<fn(String, f64) -> String> = Function::from_fn(|s, n| format!("{s}{n}"));

let token = String::from("moved");
let g: Function<fn() -> String> = Function::once(move || token);
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

`Promise<T>` implements [`IntoFuture`], so it can be `.await`ed directly in
any `async` function. The resolved value has type `T` without any cast:

```rust
use js_sys::{Promise, Number};

#[wasm_bindgen]
extern "C" {
    fn fetchCount() -> Promise<Number>;
}

async fn example() -> Result<Number, JsValue> {
    let count: Number = fetchCount().await?;
    Ok(count)
}
```

If you need a named [`JsFuture`]
value, use `JsFuture::from`:

```rust
use js_sys::futures::JsFuture;

let future: JsFuture<Number> = JsFuture::from(fetchCount());
```

A second parameter types the rejection: `Promise<T, E>` awaits to
`Result<T, E>`. It defaults to `JsValue`, so `Promise<T>` is
`Promise<T, JsValue>`. Both parameters are covariant through `Upcast`, so a
`Promise<T, TypeError>` can be passed where a `Promise<T, Error>` or a
`Promise<T>` is expected.

```rust
use js_sys::{Number, Promise, TypeError};

#[wasm_bindgen]
extern "C" {
    fn fetchCount() -> Promise<Number, TypeError>;
}

async fn example() -> Result<Number, TypeError> {
    fetchCount().await
}
```

[`IntoFuture`]: https://doc.rust-lang.org/std/future/trait.IntoFuture.html
[`JsFuture`]: https://docs.rs/js-sys/*/js_sys/futures/struct.JsFuture.html

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

Represents a JS value that may be `T` or `undefined`, matching TypeScript's
`T | undefined`. Only `undefined` is treated as absent; JS `null` is a
distinct present value.

```rust
use js_sys::{JsOption, Number};

#[wasm_bindgen]
extern "C" {
    fn maybeGetValue() -> JsOption<Number>;
}

let value = maybeGetValue();

// Check emptiness
if value.is_empty() {
    // undefined
}

// Convert to Option
match value.into_option() {
    Some(num) => { /* use num */ }
    None => { /* handle undefined */ }
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

## JsNullable\<T\>

Represents a JS value that may be `T` or `null`, matching WebIDL nullable
types (`T?`). Following WebIDL's ECMAScript conversion rules, both `null`
and `undefined` are treated as absent; the canonical empty value produced
from Rust is `null`. `web-sys` uses `JsNullable<T>` for nullable types
nested inside generics, such as `Promise<GpuError?>`.

```rust
use js_sys::{JsNullable, Number};

#[wasm_bindgen]
extern "C" {
    fn maybeGetValue() -> JsNullable<Number>;
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

// Create values (the same unwrap methods as JsOption are also available)
let with_value = JsNullable::wrap(Number::from(42));
let empty: JsNullable<Number> = JsNullable::new(); // null
let from_opt = JsNullable::from_option(Some(Number::from(42)));
```

**`JsOption<T>` / `JsNullable<T>` vs `Option<T>`:**

| Type | Behavior |
| ---- | -------- |
| `Option<T>` | Undefined or Null check at ABI boundary, immediately converts to `Some(T)` or `None` |
| `JsOption<T>` | Defers undefined check, works in `JsGeneric` positions |
| `JsNullable<T>` | Defers null or undefined check, works in `JsGeneric` positions |

## Strings and bytes (unstable)

Under `--cfg=js_sys_unstable_apis`, small built-in string results are
`String`, such as `Error::message`, `Date::to_iso_string`, `Number::to_fixed`,
`RegExp::source`, `Function::name` and the `Intl` resolved options and parts,
and `JSON::stringify` returns `Result<Option<String>, JsValue>`, with `None`
for a value that has no JSON form. `Date::new_with_value` takes a time value
or a date string, and an `ArrayBuffer` converts to and from bytes:

```rust
let message: String = err.message();
let date = Date::new_with_value("2020-01-01");
let buf = ArrayBuffer::from(&bytes[..]);
let copy: Vec<u8> = buf.to_vec();
```
