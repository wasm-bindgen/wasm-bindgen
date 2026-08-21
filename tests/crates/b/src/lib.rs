extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "C" {
    fn foo(x: u32);
}

pub fn test() {
    foo(10);
}

#[wasm_bindgen(private)]
pub struct Dupe {
    pub factor: u32,
}

#[wasm_bindgen]
impl Dupe {
    pub fn apply(&self, x: u32) -> u32 {
        x + self.factor
    }
}

pub fn make_dupe() -> Dupe {
    Dupe { factor: 7 }
}
