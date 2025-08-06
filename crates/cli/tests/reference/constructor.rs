use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClassConstructor(());

#[wasm_bindgen]
impl ClassConstructor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ClassConstructor {
        ClassConstructor(())
    }
}
