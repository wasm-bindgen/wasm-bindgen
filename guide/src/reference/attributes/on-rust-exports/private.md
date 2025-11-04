# `private`

The `private` attribute can be applied to exported structs and enums to generate the exported binding but not export it on the public exports of the JavaScript module. This allows defining types that can be used as arguments or return values in exported functions without exposing them in the public exported API.

```rust
#[wasm_bindgen(private)]
pub struct Config {
    pub timeout: i32,
}

#[wasm_bindgen]
pub fn create_config(timeout: i32) -> Config {
    Config { timeout }
}

#[wasm_bindgen]
pub fn apply_config(config: Config) -> i32 {
    config.timeout * 2
}
```

This will generate JavaScript where `Config` is defined internally but not exported:

```js
import * as app from './app';

// app.Config is undefined - not exported
console.log(app.Config); // undefined

// But the functions that use Config are exported
const config = app.create_config(100);
const result = app.apply_config(config);
```

The TypeScript definitions will still export the type under its name as a type-only export:

```ts
export function create_config(timeout: number): Config;
export function apply_config(config: Config): number;

class Config {
    timeout: number;
}

export type { Config };
```

The `private` attribute is only supported on structs and enums, not on functions.
