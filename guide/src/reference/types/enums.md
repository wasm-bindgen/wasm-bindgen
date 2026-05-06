# Exported Rust Enums

Rust enums can be exported to JavaScript in different forms depending on their structure.

## Numeric Enums

C-style enums (enums without associated data) are exported as JavaScript objects with numeric values.

### Basic Usage

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[wasm_bindgen]
pub fn cycle_color(color: Color) -> Color {
    match color {
        Color::Red => Color::Green,
        Color::Green => Color::Blue,
        Color::Blue => Color::Red,
    }
}
```

In JavaScript:

```js
import { Color, cycle_color } from './my_module';

console.log(Color.Red);    // 0
console.log(Color.Green);  // 1
console.log(Color.Blue);   // 2

const next = cycle_color(Color.Red);
console.log(next === Color.Green); // true
```

### Custom Discriminant Values

You can specify custom numeric values for enum variants:

```rust
#[wasm_bindgen]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}
```

In JavaScript:

```js
console.log(HttpStatus.Ok);            // 200
console.log(HttpStatus.NotFound);      // 404
console.log(HttpStatus.InternalError); // 500
```

### String Discriminants

Enums can also use string values as discriminants:

```rust
#[wasm_bindgen]
pub enum Theme {
    Light = "light",
    Dark = "dark",
    Auto = "auto",
}
```

In JavaScript:

```js
console.log(Theme.Light); // "light"
console.log(Theme.Dark);  // "dark"
console.log(Theme.Auto);  // "auto"
```


## Dynamic Unions

A `#[wasm_bindgen]` enum that mixes string-literal variants with single-field
tuple variants is exported as an untagged TypeScript union. Discrimination is
*dynamic* — performed at the JS↔Rust boundary by inspecting the runtime value
of each candidate variant in declaration order.

For example:

```rust
#[wasm_bindgen]
pub struct Foo { /* ... */ }

#[wasm_bindgen]
pub enum ApiResult {
    Success = "success",
    Failure = "failure",
    Data(String),
    Foo(Foo),
}
```

produces the TypeScript type

```typescript
export type ApiResult = "success" | "failure" | string | Foo;
```

When a value crosses from JS into Rust, all string-literal variants share a
single coalesced `as_string` check that runs before any tuple-variant
dispatch, so a value like `"success"` always wins against a generic
`Data(String)` regardless of declaration order. Tuple variants are then
tested in source order via the same machinery as `JsCast::dyn_into`, with
the first successful conversion taking the value.

Each variant payload type must be convertible from `JsValue`. Place
narrower types before broader ones — for example, a specific exported
struct before `String` — since dispatch is in source order and the first
match wins. Large unions should be avoided in hot paths since each tuple
variant is tested in turn.

### Runtime witnesses

Discrimination only works when the variant's payload type has a meaningful
runtime check. Exported `#[wasm_bindgen]` structs have a real prototype
chain. Imported classes with a JS-side constructor are checked via
`instanceof`. Primitive types like `String`, `bool`, and the numeric types
are checked structurally.

**Interface-only imports do not have a runtime witness.** A type imported
purely for the type system, with no JS-side construct of that name, has an
`instanceof` check that always returns `false`:

```rust
#[wasm_bindgen]
extern "C" {
    // No JS-side `Shape` exists at runtime.
    #[wasm_bindgen(typescript_type = "Shape")]
    pub type Shape;
}
```

A dynamic union variant whose payload is `Shape` will *never* match
through the normal dispatch chain. The next section describes the
supported pattern for this case.

For interface-only types you can also supply your own predicate via
`#[wasm_bindgen(is_type_of = my_check)]` if a runtime test is genuinely
possible (e.g., checking for a discriminant property).

### Fallback variant

Mark the enum with `#[wasm_bindgen(fallback)]` to make the *last* tuple
variant act as an unconditional catch-all. This is the supported pattern
for unions where the trailing variant has no meaningful runtime check:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Shape")]
    pub type Shape;
}

#[wasm_bindgen(fallback)]
pub enum ApiResult {
    Loading = "loading",
    Empty = "empty",
    Anything(Shape),
}

#[wasm_bindgen]
pub fn echo(r: ApiResult) -> ApiResult { r }
```

```typescript
export type ApiResult = "loading" | "empty" | Shape;
```

Dispatch still runs the literal-string match first, so `"loading"` and
`"empty"` route to their named variants. Anything else — any object,
number, or other value — is unconditionally accepted as `Anything(_)`.

The fallback rule applies only to the last tuple variant. Earlier tuple
variants still run their normal runtime check, so you can place narrower
typed variants before the catch-all:

```rust
#[wasm_bindgen(fallback)]
pub enum Mixed {
    Loading = "loading",
    Specific(MyExportedStruct),  // tested first via instanceof
    Anything(Shape),             // accepts anything that didn't match above
}
```

### Nesting and `Option`

A variant payload may itself be another dynamic union or a string enum, and
the union may appear inside `Option`:

```rust
#[wasm_bindgen]
pub enum Inner { Foo = "foo", Bar = "bar", Number(u32) }

#[wasm_bindgen]
pub enum Outer { Loading = "loading", Wrapped(Inner), Bare(Foo) }

#[wasm_bindgen]
pub fn maybe(o: Option<Outer>) -> Option<Outer> { o }
```

The generated TypeScript flattens the inner alias by name and renders the
optional with the standard wasm-bindgen pattern:

```typescript
export type Inner = "foo" | "bar" | number;
export type Outer = "loading" | Inner | Foo;

export function maybe(o?: Outer | null): Outer | undefined;
```

### Hiding the type alias

By default the generated TypeScript alias is emitted as `export type`. To
keep the alias module-private, mark the enum with `private`:

```rust
#[wasm_bindgen(private)]
pub enum InternalState { /* ... */ }
```

The same flag is honoured by string enums and by C-style enums.
