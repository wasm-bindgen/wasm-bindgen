# `js_name = Blah`

The `js_name` attribute can be used to export a different name in JS than what
something is named in Rust. It can be applied to both exported Rust functions
and types.

For example, this is often used to convert between Rust's snake-cased
identifiers into JavaScript's camel-cased identifiers:

```rust
#[wasm_bindgen(js_name = doTheThing)]
pub fn do_the_thing() -> u32 {
    42
}
```

This can be used in JavaScript as:

```js
import { doTheThing } from './my_module';

const x = doTheThing();
console.log(x);
```

Like imports, `js_name` can also be used to rename types exported to JS:

```rust
#[wasm_bindgen(js_name = Foo)]
pub struct JsFoo {
    // ..
}
```

to be accessed like:

```js
import { Foo } from './my_module';

// ...
```

Note that attaching methods to the JS class `Foo` should be done via the
[`js_class` attribute](js_class.html):

```rust
#[wasm_bindgen(js_name = Foo)]
pub struct JsFoo { /* ... */ }

#[wasm_bindgen(js_class = Foo)]
impl JsFoo {
    // ...
}
```

It can also be used to rename parameters of exported functions and methods:

```rust
#[wasm_bindgen]
pub fn foo(
    #[wasm_bindgen(js_name = "firstArg")]
    arg1: String,
) {
    // function body
}

#[wasm_bindgen]
pub struct Foo {
    // properties
}

#[wasm_bindgen]
impl Foo {
    pub fn foo(
        &self,
        #[wasm_bindgen(js_name = "firstArg")]
        arg1: u32,
    ) {
        // function body
    }
}
```

Which will generate the following JS bindings:

```js
/**
 * @param {string} firstArg
 */
export function foo(firstArg) {
    // ...
}

export class Foo {
    /**
     * @param {number} firstArg
     */
    foo(firstArg) {
        // ...
    }
}
```

## Well-known symbols

`js_name` also accepts the explicit bracket-string form
`"[Symbol.<name>]"` to expose a method on a Rust-defined class under a
[well-known symbol][well-known-symbols]. This works for both methods and
[`getter` / `setter`](getter-and-setter.html) attributes.

```rust
#[wasm_bindgen]
pub struct Foo;

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(js_name = "[Symbol.toPrimitive]")]
    pub fn to_primitive(&self) -> String {
        "Foo".to_string()
    }

    #[wasm_bindgen(getter = "[Symbol.toStringTag]")]
    pub fn to_string_tag(&self) -> String {
        "Foo".to_string()
    }
}
```

This generates a class with computed-key members:

```js
export class Foo {
    [Symbol.toPrimitive]() { /* ... */ }
    get [Symbol.toStringTag]() { /* ... */ }
}
```

Only the exact form `"[Symbol.<ident>]"` is accepted; arbitrary
expressions inside `[ ... ]` are not supported.

[well-known-symbols]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol#well-known_symbols
