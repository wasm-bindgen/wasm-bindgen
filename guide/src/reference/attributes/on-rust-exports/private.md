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

## Name collisions

Because private types are not part of the public JavaScript API, a name may
be defined privately more than once in a build without colliding, whether in
different crates or in different modules of one crate. This is useful for
crates that use private types as implementation details: two versions of such
a crate can coexist in one dependency graph, and generated bindings can define
the same enum in several modules. When the same name is defined more than
once, the private copies are numbered (`Config`, `Config2`, `Config3`, ...) in
the generated bindings.

This applies to structs, numeric enums, string enums and dynamic unions,
which all share one name space in the generated bindings. Two structs with the
same name in one crate, or two dynamic unions given the same `js_name` in one
module, are still rejected by the Rust compiler, as their generated symbols
are only unique per crate or per module.

Exporting the same name publicly more than once remains an error, reported by
the `wasm-bindgen` CLI with the qualified name (taking `js_namespace` into
account).
