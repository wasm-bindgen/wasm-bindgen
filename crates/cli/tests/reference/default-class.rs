use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = default)]
pub struct DefaultClass {
    value: i32,
}

#[wasm_bindgen(js_class = default)]
impl DefaultClass {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i32) -> DefaultClass {
        DefaultClass { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}
