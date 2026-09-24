// Argument names that match identifiers used by the generated JS glue must not
// shadow them.

use wasm_bindgen::prelude::*;

/// Named like the module-level `wasm` object.
#[wasm_bindgen]
pub fn wasm_arg(wasm: &[u8]) -> String {
    format!("{wasm:?}")
}

/// Named like locals of the generated function body.
#[wasm_bindgen]
pub fn local_args(ptr0: &str, len0: u32, ret: u32) -> String {
    format!("{ptr0}{len0}{ret}")
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

    /// Names that don't collide stay unchanged.
    pub fn add(&self, value: u32) -> u32 {
        self.value + value
    }
}
