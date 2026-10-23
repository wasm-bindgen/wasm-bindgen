use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_optional(a: Option<u32>, b: Option<u32>, c: Option<u32>) {}

#[wasm_bindgen]
pub fn some_optional(a: Option<u32>, b: u32, c: Option<u32>) {}

#[wasm_bindgen]
pub fn optional_string(s: Option<String>) -> Option<String> {
    s
}

#[wasm_bindgen]
pub fn optional_bytes(b: Option<Vec<u8>>, c: Option<bool>, d: Option<char>) {}

#[wasm_bindgen]
pub enum Flag {
    Yes = "yes",
    No = "no",
}

#[wasm_bindgen]
pub fn optional_string_enum(f: Option<Flag>) {}

#[wasm_bindgen]
pub struct Opt {
    pub n: Option<u32>,
}

#[wasm_bindgen]
pub fn optional_struct(o: Option<Opt>, v: Option<JsValue>) {}
