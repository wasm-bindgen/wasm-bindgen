# enum

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
| :-----------: | :------------: | :----------------: | :--------------: | :-------------------: | :----------------------: | :-----------------------: |
|      Yes      |       No       |         No         |        No        |          Yes          |           Yes            |   `string` or `number`    |

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/enum.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/enum.js}}
```

## TypeScript

Unfortunately, string enums don't fully work yet; no TypeScript is generated for the enum itself and functions using them accept or return `any`.
They work correctly, it's just there's no type hints.
See [Issue #3057](https://github.com/rustwasm/wasm-bindgen/issues/3057)

The generated TypeScript declarations for the above:

<!-- remember to keep this up to date! copy enum.rs (above) into the lib.rs file of a new wasm-bindgen crate; use `wasm-pack build`; then copy pkg/testcrate.d.ts. also ran it through a formatter. -->

```ts
/**
 * @param {NumberEnum} x
 */
export function take_number(x: NumberEnum): void;
/**
 * @param {any} x
 */
export function take_string(x: any): void;
/**
 * @param {NumberEnum | undefined} [x]
 */
export function take_number_option(x?: NumberEnum): void;
/**
 * @param {any | undefined} [x]
 */
export function take_string_option(x?: any): void;
/**
 * @returns {NumberEnum}
 */
export function return_number(): NumberEnum;
/**
 * @returns {any}
 */
export function return_string(): any;
/**
 * @returns {NumberEnum | undefined}
 */
export function return_number_option(): NumberEnum | undefined;
/**
 * @returns {any | undefined}
 */
export function return_string_option(): any | undefined;
/**
 */
export enum NumberEnum {
  Foo = 0,
  Bar = 1,
  Baz = 2,
}

// no types generated for StringEnum (yet) :(
// see the note above
```
