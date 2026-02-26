# `unchecked_return_type`, `unchecked_param_type`, and `unchecked_optional_param_type`

Return and parameter types of exported functions and methods can be overwritten with `#[wasm_bindgen(unchecked_return_type)]` and `#[wasm_bindgen(unchecked_param_type)]`.

Parameters can also be marked as optional using `#[wasm_bindgen(unchecked_optional_param_type)]`, which generates TypeScript's `?:` syntax and JSDoc's `[paramName]` syntax.

> **Note**: Types that are provided using `#[wasm_bindgen(unchecked_return_type)]`, `#[wasm_bindgen(unchecked_param_type)]`, and `#[wasm_bindgen(unchecked_optional_param_type)]` aren't checked for their contents. They will end up in a function signature and JSDoc exactly as they have been specified. E.g. `#[wasm_bindgen(unchecked_return_type = "number")]` on a function returning `String` will return a `string`, not a `number`, even if the TS signature and JSDoc will say otherwise.

> **Note**: `unchecked_param_type` and `unchecked_optional_param_type` are mutually exclusive and cannot be used on the same parameter. As in TypeScript, a required parameter cannot follow an optional parameter.

```rust
#[wasm_bindgen(unchecked_return_type = "Foo")]
pub fn foo(
    #[wasm_bindgen(unchecked_param_type = "Bar")]
    arg1: JsValue,
) -> JsValue {
    // function body
}

#[wasm_bindgen]
pub fn greet(
    #[wasm_bindgen(unchecked_optional_param_type = "string")]
    name: JsValue,
) -> JsValue {
    if name.is_undefined() {
        "Hello, World!".into()
    } else {
        format!("Hello, {}!", name.as_string().unwrap_or_default()).into()
    }
}

#[wasm_bindgen]
pub struct Foo {
    // properties
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(unchecked_return_type = "Baz")]
    pub fn foo(
        &self,
        #[wasm_bindgen(unchecked_param_type = "Bar")]
        arg1: JsValue,
    ) -> JsValue {
        // function body
    }
}
```

Which will generate the following JS bindings:
```js
/**
 * @param {Bar} arg1
 * @returns {Foo}
 */
export function foo(arg1) {
    // ...
}

/**
 * @param {string} [name]
 * @returns {any}
 */
export function greet(name) {
    // ...
}

export class Foo {
    /**
     * @param {Bar} arg1
     * @returns {Baz}
     */
    foo(arg1) {
        // ...
    }
}
```

And the following TypeScript definitions:
```ts
export function foo(arg1: Bar): Foo;
export function greet(name?: string): any;
```
