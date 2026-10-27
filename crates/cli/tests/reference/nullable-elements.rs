// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }

// `Option<T>` and `Nullable<T>` as borrowed sequence elements and in a
// `Function` signature: a borrowed slice encodes `Option<T>`'s ABI per
// element. A borrowed scalar or borrowed borrow crosses as the value.

use js_sys::{Function, JsString, Undefined};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn opt_f64s(v: &[Option<f64>]);
    fn nullable_strings(v: &[Nullable<String>]);
    fn nullable_js_strings(v: &[Nullable<JsString>]);
    fn call_nullable(f: &Function<fn(Nullable<JsString>) -> Undefined>);
    fn opt_ref_nullable_string(v: Option<&Nullable<String>>);
    fn ref_f64(v: &f64);
    fn ref_ref_str(v: &&str);
}

#[wasm_bindgen]
pub fn use_imports(f: &Function<fn(Nullable<JsString>) -> Undefined>) {
    opt_f64s(&[Some(1.0), None]);
    nullable_strings(&[Nullable::Null]);
    nullable_js_strings(&[Nullable::Null]);
    call_nullable(f);
    opt_ref_nullable_string(Some(&Nullable::Null));
    ref_f64(&1.0);
    ref_ref_str(&"a");
}
