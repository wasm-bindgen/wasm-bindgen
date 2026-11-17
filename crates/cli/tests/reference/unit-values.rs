// FLAGS: --target=bundler
// FLAGS: --target=bundler --debug

// Reference test for a sequence of `()`, which is `undefined`. Its cell has
// no words, so the sequence is only a length: nothing is allocated or freed
// for one, its address is the cell's alignment, and into Rust each element
// is checked to be `undefined`, in every build.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn units(v: Vec<()>) -> Vec<()> {
    v
}

#[wasm_bindgen]
pub fn opt_units(v: Option<Vec<()>>) -> Option<Vec<()>> {
    v
}

#[wasm_bindgen]
pub fn nested(v: Vec<Vec<()>>) -> Vec<Vec<()>> {
    v
}

#[wasm_bindgen]
pub fn beside(a: u32, names: Vec<String>, units: Vec<()>) -> u32 {
    a + (names.len() + units.len()) as u32
}

#[wasm_bindgen]
extern "C" {
    fn js_units(v: Vec<()>) -> Vec<()>;
    fn js_slice(v: &[()], w: Option<&[()]>);
}

#[wasm_bindgen]
pub fn driver() {
    js_units(js_units(vec![(), ()]));
    js_slice(&[(), ()], None);
}
