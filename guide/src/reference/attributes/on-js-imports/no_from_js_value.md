# `no_from_js_value`

Every imported type gets a `From<JsValue>` impl that is an unchecked cast, and
it is the conversion applied to a `catch` import's error and to a
`Promise<T, E>` rejection, which can be any JS value. The `no_from_js_value`
attribute omits the generated impl, so that a type can provide one that
validates or normalizes the value instead:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Error, no_from_js_value)]
    type MyError;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> MyError;
}

impl From<JsValue> for MyError {
    fn from(value: JsValue) -> MyError {
        match value.dyn_into() {
            Ok(error) => error,
            Err(value) => MyError::new(&format!("{value:?}")),
        }
    }
}
```

`JsCast::unchecked_into` remains available as the unchecked cast.
