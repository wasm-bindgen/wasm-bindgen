// A lent sequence lends each element, and a reference lends as itself, so
// what it holds may be borrowed in turn, and a borrowed vector may be absent.
// Only the `&[u8]` argument itself is a view: a slice that a sequence holds,
// or that is behind a second reference, is a copy, with nothing of Rust's
// freed but the buffers that encode a sequence.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn bytes(v: &[u8]);
    fn ref_bytes(v: &&[u8]);
    fn opt_ref_bytes(v: Option<&&[u8]>);
    fn ref_opt_bytes(v: &Option<&[u8]>);
    fn byte_slices(v: &[&[u8]]);
    fn string_slices(v: &[&[String]]);
    fn opt_values(v: &[Option<&JsValue>]);
    fn opt_byte_vec(v: Option<&Vec<u8>>);
    fn opt_string_vec(v: Option<&Vec<String>>);
    fn opt_vecs(v: &[Option<Vec<u32>>]);
}

#[wasm_bindgen]
pub fn use_imports(value: &JsValue) {
    let strings = vec![String::from("a")];
    bytes(&[1]);
    ref_bytes(&&[1][..]);
    opt_ref_bytes(None);
    ref_opt_bytes(&None);
    byte_slices(&[&[1], &[]]);
    string_slices(&[&strings, &[]]);
    opt_values(&[Some(value), None]);
    opt_byte_vec(Some(&vec![1]));
    opt_string_vec(Some(&strings));
    opt_vecs(&[Some(vec![1]), None]);
}
