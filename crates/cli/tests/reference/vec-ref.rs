// Reference test for borrowed-vector outgoing arguments (`&Vec<T>`).
//
// `&Vec<T>` shares the wire format of owned `Vec<T>` (a buffer
// allocated by Rust that JS owns and frees) but lands on the JS side
// as a plain `Array`. For primitive element kinds the typed-array view
// is wrapped in `Array.from(...)` before being handed to the JS shim.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn js_take_vec_u16_as_array(v: &Vec<u16>);
    fn js_take_vec_string_as_array(v: &Vec<String>);
    fn js_take_optional_vec_u16_as_array(v: Option<&Vec<u16>>);
}

#[wasm_bindgen]
pub fn driver() {
    let v = vec![1u16, 2, 3];
    js_take_vec_u16_as_array(&v);

    let s = vec!["a".to_string(), "b".to_string()];
    js_take_vec_string_as_array(&s);

    js_take_optional_vec_u16_as_array(Some(&v));
    js_take_optional_vec_u16_as_array(None);
}
