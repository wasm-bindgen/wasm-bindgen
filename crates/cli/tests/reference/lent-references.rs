// A borrowed vector may be absent, as an argument and as what a lent
// sequence holds. It is a copy, where the `&[u8]` argument is a view, with
// nothing of Rust's freed but the buffers that encode a sequence.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn bytes(v: &[u8]);
    fn opt_byte_vec(v: Option<&Vec<u8>>);
    fn opt_string_vec(v: Option<&Vec<String>>);
    fn opt_vecs(v: &[Option<Vec<u32>>]);
}

#[wasm_bindgen]
pub fn use_imports() {
    let strings = vec![String::from("a")];
    bytes(&[1]);
    opt_byte_vec(Some(&vec![1]));
    opt_string_vec(Some(&strings));
    opt_vecs(&[Some(vec![1]), None]);
}
