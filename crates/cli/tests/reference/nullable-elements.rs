// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }

// `Option<T>` and `Nullable<T>` as sequence elements and in a `Function`
// signature: a borrowed slice encodes `Option<T>`'s ABI per element, an
// owned vector of scalars encodes the same cells, and an owned vector of
// strings or handles crosses as handles typed with the optional element.

use js_sys::{Function, JsString, Undefined};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn opt_f64s(v: &[Option<f64>]);
    fn nullable_strings(v: &[Nullable<String>]);
    fn nullable_js_strings(v: &[Nullable<JsString>]);
    fn vec_nullable_strings(v: Vec<Nullable<String>>) -> Vec<Nullable<String>>;
    fn vec_opt_u8s(v: Vec<Option<u8>>) -> Vec<Option<u8>>;
    fn call_nullable(f: &Function<fn(Nullable<JsString>) -> Undefined>);
}

#[wasm_bindgen]
pub fn export_nullable_strings(v: Vec<Nullable<String>>) -> Vec<Option<Nullable<String>>> {
    v.into_iter().map(Some).collect()
}

#[wasm_bindgen]
pub fn export_opt_u8s(v: Vec<Option<u8>>) -> Vec<Option<u8>> {
    v
}

#[wasm_bindgen]
pub fn use_imports(f: &Function<fn(Nullable<JsString>) -> Undefined>) {
    opt_f64s(&[Some(1.0), None]);
    nullable_strings(&[Nullable::Null]);
    nullable_js_strings(&[Nullable::Null]);
    vec_nullable_strings(Vec::new());
    vec_opt_u8s(Vec::new());
    call_nullable(f);
}
