use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_nullable(a: Nullable<u32>, b: Nullable<u32>, c: Nullable<u32>) {}

#[wasm_bindgen]
pub fn some_nullable(a: Nullable<u32>, b: u32, c: Nullable<u32>) {}

#[wasm_bindgen]
pub fn nullable_string(s: Nullable<String>) -> Nullable<String> {
    s
}

#[wasm_bindgen]
pub fn nullable_bytes(b: Nullable<Vec<u8>>, c: Nullable<bool>, d: Nullable<char>) -> Nullable<Vec<u8>> {
    b
}

#[wasm_bindgen]
pub enum Flag {
    Yes = "yes",
    No = "no",
}

#[wasm_bindgen]
pub fn nullable_string_enum(f: Nullable<Flag>) -> Nullable<Flag> {
    f
}

#[wasm_bindgen]
pub struct Opt {
    pub n: Nullable<u32>,
}

#[wasm_bindgen]
pub fn nullable_struct(o: Nullable<Opt>, v: Nullable<JsValue>) -> Nullable<Opt> {
    o
}
