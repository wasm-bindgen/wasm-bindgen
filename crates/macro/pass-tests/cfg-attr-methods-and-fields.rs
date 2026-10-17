use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {
    #[cfg_attr(all(), wasm_bindgen(skip))]
    pub bytes: [u8; 8],
    #[cfg_attr(all(), wasm_bindgen(getter_with_clone))]
    pub name: String,
}

#[wasm_bindgen]
impl A {
    #[cfg_attr(all(), wasm_bindgen(getter))]
    pub fn first(&self) -> u8 {
        self.bytes[0]
    }

    #[cfg_attr(all(), wasm_bindgen(setter))]
    pub fn set_first(&mut self, value: u8) {
        self.bytes[0] = value;
    }
}

fn main() {}
