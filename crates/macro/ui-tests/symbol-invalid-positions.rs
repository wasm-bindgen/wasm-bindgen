use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "[notSymbol]")]
pub struct BadComputedKey;

#[wasm_bindgen(js_name = "[Symbol.]")]
pub struct BadEmptySymbol;

#[wasm_bindgen(js_name = "[Symbol.iterator]")]
pub struct BadStruct;

#[wasm_bindgen(js_name = "[Symbol.iterator]")]
pub enum BadEnum {
    A = 0,
}

#[wasm_bindgen(js_name = "[Symbol.iterator]")]
pub fn bad_free_export() {}

#[wasm_bindgen]
pub fn bad_arg(#[wasm_bindgen(js_name = "[Symbol.iterator]")] _arg: u32) {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "[Symbol.iterator]")]
    type BadType;

    #[wasm_bindgen(js_name = "[Symbol.iterator]")]
    static BAD_STATIC: JsValue;

    #[wasm_bindgen(js_name = "[Symbol.iterator]")]
    fn bad_free_import();
}

fn main() {}
