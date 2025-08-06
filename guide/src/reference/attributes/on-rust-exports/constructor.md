# `constructor`

When attached to a Rust "constructor" it will make the generated JavaScript
bindings callable as `new Foo()`.

For example, consider this exported Rust type and `constructor` annotation:

```rust
#[wasm_bindgen]
pub struct Foo {
    contents: u32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Foo {
        Foo { contents: 0 }
    }

    pub fn get_contents(&self) -> u32 {
        self.contents
    }
}
```

This can be used in JavaScript as:

```js
import { Foo } from './my_module';

const f = new Foo();
console.log(f.get_contents());
```

## Caveats

In versions `>=v0.2.48, <0.2.88` of `wasm-bindgen`, there is a bug which breaks inheritance of exported Rust structs from JavaScript side (see [#3213](https://github.com/wasm-bindgen/wasm-bindgen/issues/3213)). If you want to inherit from a Rust struct such as:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Parent {
    msg: String,
}

#[wasm_bindgen]
impl Parent {
    #[wasm_bindgen(constructor)]
    fn new() -> Self {
        Parent {
            msg: String::from("Hello from Parent!"),
        }
    }
}
```

You will need to reset the prototype of `this` back to the `Child` class prototype after calling the `Parent`'s constructor via `super`.

```js
import { Parent } from './my_module';

class Child extends Parent {
    constructor() {
        super();
        Object.setPrototypeOf(this, Child.prototype);
    }
}
```

This is no longer required as of v0.2.88.
