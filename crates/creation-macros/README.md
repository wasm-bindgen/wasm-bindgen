# wasm-bindgen-creation-macros

This crate provides procedural macros for the `wasm-bindgen` project, specifically focused on code generation and WebAssembly bindings creation.

## Overview

The `json!` and `array!` macros help in the creation of `js_sys::Object` and `js_sys::Array`, respectively. Specifically, they cut down on the verbose repetitive code needed to initialize `Object` and `Array` objects using plain Rust. Both macros support the use of any literals or variables that implement [`Into<JsValue>`](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html#trait-implementations). That includes rust strings, floating point numbers and integers, etc. 

### Examples

```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object};
use wasm_bindgen_creation_macros::{array, json};

fn create_person() -> Object {
    let address = json! {
        street: "123 Main St",
        city: "Tech City",
        country: "Rustland"
    };

    json! {
        name: "Alice",
        age: 30,
        hobbies: ["reading", "coding", "hiking"],
        address: address, // Note the use of a variable!
        fav_floats: [ 1.618, 3.14, 2.718 ],
        nested_obj: {
            num: 42,
            inner_obj: {
                msg: "Arbitrary nesting is supported!"
            }
        }
    }
}

fn create_numbers() -> Array {
    // variables work in array! as well.
    const FIVE: u32 = 5;
    array![1, 2, 3, 4, FIVE]
}

// Since variables are supported, array! and json! can be 
// used together to create complex objects.
fn mix_and_match() -> Object {
    let evens = array![2, 4, 6, 8];
    let odds = array![1, 3, 6, 7];

    let rust = json! {
        language: "Rust",
        mascot: "Crab"
    };

    let go = json! {
        language: "Go",
        mascot: "Gopher"
    };

    let languages_array = array![ rust, go, { language: "Python", mascot: "Snakes" } ];

    json! {
        evens: evens,
        odds: odds,
        languages: languages_array
    }
}
```

## A Note on Parsing

The parser used is not sophisticated; Rust values that are not **simple** Rust literals should be stored in a variable first, then the variable should be added to the macro. Attempting to pass non-simple rust syntax will cause compilation to fail.

```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object};
use wasm_bindgen_creation_macros::{array, json};

pub struct CustomJsValue(u32);

impl Into<JsValue> for CustomJsValue {
    fn into(self) -> JsValue {
        self.0.into()
    }
}

// WILL NOT COMPILE
fn incorrect() -> Object {
    json! {
        custom: CustomJsValue(42)
    }
}

// Do this instead
fn correct() -> Object {
    let custom = CustomJsValue(42);
    json! {
        js_value: custom
    }
}

// WILL NOT COMPILE
fn also_invalid() -> Object {
    json! {
        array: array![1, 2, 3]
    }
}

// Do this instead
fn also_correct() -> Object {
    let array = array![1, 2, 3];
    json! {
        array: array
    }

}
```

Also, `json!` does not (currently) support string literal JSON keys. 


```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object};
use wasm_bindgen_creation_macros::{array, json};

// WILL NOT COMPILE
fn incorrect() -> Object {
    json! {
        "key": 42
    }
}

// Do this instead
fn correct() -> Object {
    json! {
        key: 42
    }
}
```

## Testing
To run the test suite, run `cargo test --target wasm32-unknown-unknown`.

```bash
cargo test --target wasm32-unknown-unknown
```