// FLAGS: --target=bundler
// FLAGS: --target=bundler --debug

// Argument names that match identifiers the generated JS glue binds or
// references must not shadow them.

use wasm_bindgen::prelude::*;

/// Named like the module-level `wasm` object; `wasm2` is already taken.
#[wasm_bindgen]
pub fn wasm_args(wasm: &[u8], wasm2: u32) -> String {
    format!("{wasm:?}{wasm2}")
}

/// Named like locals of the generated function body.
#[wasm_bindgen]
pub fn local_args(ptr0: &str, len0: u32, ret: u32) -> String {
    format!("{ptr0}{len0}{ret}")
}

/// Named like a helper function and a JS global the body references.
#[wasm_bindgen]
pub fn global_args(isLikeNone: Option<u32>, undefined: u32) -> Option<u32> {
    isLikeNone.map(|v| v + undefined)
}

/// Named like a class the generated function body refers to.
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn class_arg(Foo: &Foo) -> Foo {
    Foo { value: Foo.value }
}

#[wasm_bindgen]
pub struct Foo {
    value: u32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> Foo {
        Foo { value }
    }

    /// Named like the `ptr` local of methods taking `self` by value.
    pub fn consume(self, ptr: u32) -> u32 {
        self.value + ptr
    }

    /// Only appears in an error message of the debug glue, so stays unchanged.
    pub fn add(&self, value: u32) -> u32 {
        self.value + value
    }
}
