# `start`

When attached to a function this attribute will configure the `start`
section of the Wasm executable to be emitted, executing the tagged function as
soon as the Wasm module is instantiated.

```rust
#[wasm_bindgen(start)]
fn start() {
    // executed automatically ...
}
```

The `start` section of the Wasm executable will be configured to execute the
`start` function here as soon as it can. Note that due to various practical
limitations today the start section of the executable may not literally point to
`start`, but the `start` function here should be started up automatically when the
wasm module is loaded.

## Multiple start functions

Multiple `#[wasm_bindgen(start)]` functions can be specified across a module and
its dependencies. They will be chained together and all executed during
initialization. The execution order is arbitrary, so functions should not rely
on other start functions having run before them.

```rust
#[wasm_bindgen(start)]
fn init_logging() {
    // ...
}

#[wasm_bindgen(start)]
fn init_state() {
    // ...
}
```

## Private start functions

By default, start functions are also exported to JS as regular functions. To
register a start function that runs at initialization but is not exported to JS,
use the `private` attribute:

```rust
#[wasm_bindgen(start, private)]
fn my_init() {
    // runs at startup, but not callable from JS
}
```

## Caveats

* The `start` function must take no arguments and must either return `()` or
  `Result<(), JsValue>`
* The `start` function will not be executed when testing.
