use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_nullable(a: Nullable<u32>, b: Nullable<Option<u32>>, c: Nullable<Option<u32>>) {}

#[wasm_bindgen]
pub fn some_nullable(a: Nullable<Option<u32>>, b: Nullable<u32>, c: Nullable<Option<u32>>) {}

#[wasm_bindgen]
pub fn nullable_string(s: Nullable<String>) -> Nullable<String> {
    s
}

#[wasm_bindgen]
pub fn optional_nullable_string(s: Nullable<Option<String>>) -> Nullable<Option<String>> {
    s
}

#[wasm_bindgen]
pub fn nullable_bytes(b: Nullable<Vec<u8>>, c: Nullable<bool>, d: Nullable<char>) -> Nullable<Vec<u8>> {
    b
}

#[wasm_bindgen]
pub fn optional_nullable_bytes(
    b: Nullable<Option<Vec<u8>>>,
    c: Nullable<Option<bool>>,
    d: Nullable<Option<char>>,
) -> Nullable<Option<Vec<u8>>> {
    b
}

#[wasm_bindgen]
pub fn nullable_i64(a: Nullable<i64>, b: Nullable<Option<i64>>) -> Nullable<Option<i64>> {
    match b {
        Nullable::Value(None) => a.map(Some),
        b => b,
    }
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
pub fn optional_nullable_string_enum(f: Nullable<Option<Flag>>) -> Nullable<Option<Flag>> {
    f
}

#[wasm_bindgen]
pub struct Opt {
    pub n: Nullable<u32>,
    pub o: Nullable<Option<u32>>,
}

#[wasm_bindgen]
pub fn nullable_struct(o: Nullable<Opt>, v: Nullable<JsValue>) -> Nullable<Opt> {
    o
}

#[wasm_bindgen]
pub fn optional_nullable_struct(
    o: Nullable<Option<Opt>>,
    v: Nullable<Option<JsValue>>,
) -> Nullable<Option<Opt>> {
    o
}
