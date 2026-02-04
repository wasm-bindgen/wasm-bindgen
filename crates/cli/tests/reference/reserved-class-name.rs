// Test for issue #4885: class names that conflict with JS builtins
// should be properly aliased in all generated code, including the
// FinalizationRegistry reference in constructors.
//
// This test imports the global Array type which will register "Array" as an
// identifier, causing our custom Array struct to be aliased to "Array2".

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Import the global Array constructor to cause a naming conflict
    #[wasm_bindgen(js_name = Array)]
    type JsArray;

    #[wasm_bindgen(constructor, js_class = "Array")]
    fn new() -> JsArray;
}

#[wasm_bindgen]
pub struct Array(u32);

#[wasm_bindgen]
impl Array {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Array {
        Array(0)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[wasm_bindgen]
pub fn use_js_array() -> JsArray {
    JsArray::new()
}
