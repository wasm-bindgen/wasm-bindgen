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


## Discriminated Unions

The **discriminated union** feature allows constructing type unions as enums.

While not optimized like numeric enums, these allow expressing common JavaScript union types.

For example:

```rust
extern "C" {
    #[wasm_bindgen]
    type Bar;
    // ...
}

#[wasm_bindgen]
pub struct Foo { /* ... */ }

#[wasm_bindgen]
pub enum Result {
    Success = "success",
    Failure = "failure",
    Data(String),
    Foo(Foo),
    Bar(Bar),
}
```

produces the flattened TypeScript type

```typescript
export type Result = "success" | "failure" | string | Foo | Bar;
```

All types are supported on the JS conversion interface, where
when reading a JS value into Rust, an attempt will be made to
read each variant successively, falling back to the next.

Since each item is checked, large enums should be avoided for performance.
