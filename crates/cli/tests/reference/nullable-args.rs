use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_nullable(a: Nullable<u32>, b: Option<Nullable<u32>>, c: Option<Nullable<u32>>) {}

#[wasm_bindgen]
pub fn some_nullable(a: Option<Nullable<u32>>, b: Nullable<u32>, c: Option<Nullable<u32>>) {}

#[wasm_bindgen]
pub fn nullable_string(s: Nullable<String>) -> Nullable<String> {
    s
}

#[wasm_bindgen]
pub fn optional_nullable_string(s: Option<Nullable<String>>) -> Option<Nullable<String>> {
    s
}

#[wasm_bindgen]
pub fn nullable_bytes(b: Nullable<Vec<u8>>, c: Nullable<bool>, d: Nullable<char>) -> Nullable<Vec<u8>> {
    b
}

#[wasm_bindgen]
pub fn optional_nullable_bytes(
    b: Option<Nullable<Vec<u8>>>,
    c: Option<Nullable<bool>>,
    d: Option<Nullable<char>>,
) -> Option<Nullable<Vec<u8>>> {
    b
}

#[wasm_bindgen]
pub fn nullable_i64(a: Nullable<i64>, b: Option<Nullable<i64>>) -> Option<Nullable<i64>> {
    b.or(Some(a))
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
pub fn optional_nullable_string_enum(f: Option<Nullable<Flag>>) -> Option<Nullable<Flag>> {
    f
}

#[wasm_bindgen]
pub struct Opt {
    pub n: Nullable<u32>,
    pub o: Option<Nullable<u32>>,
}

#[wasm_bindgen]
pub fn nullable_struct(o: Nullable<Opt>, v: Nullable<JsValue>) -> Nullable<Opt> {
    o
}

#[wasm_bindgen]
pub fn optional_nullable_struct(
    o: Option<Nullable<Opt>>,
    v: Option<Nullable<JsValue>>,
) -> Option<Nullable<Opt>> {
    o
}
