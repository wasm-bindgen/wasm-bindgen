//! https://github.com/wasm-bindgen/wasm-bindgen/issues/4597

use wasm_bindgen::prelude::wasm_bindgen;

mod foo {
    use crate::wasm_bindgen;

    #[wasm_bindgen]
    struct Foo;
}

mod bar {
    use crate::wasm_bindgen;

    #[wasm_bindgen]
    struct Bar;
}

#[allow(unused)]
use bar::*;
#[allow(unused)]
use foo::*;
