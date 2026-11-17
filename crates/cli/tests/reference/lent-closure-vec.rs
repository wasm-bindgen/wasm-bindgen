use wasm_bindgen::prelude::*;

// A closure lent as an element is JS's until the import it was passed to
// returns, however deep in the argument it is.
#[wasm_bindgen]
extern "C" {
    fn js_each(fs: Vec<&dyn Fn(u32) -> u32>, nested: Vec<Vec<&mut dyn FnMut()>>);
}

#[wasm_bindgen]
pub fn driver() {
    js_each(vec![&|x| x + 1], vec![vec![&mut || {}]]);
}
