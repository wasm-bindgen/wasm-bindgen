#![allow(dead_code)]

use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsGeneric;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type G<T: JsGeneric>;

    #[wasm_bindgen(extends = G)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type H;

    #[wasm_bindgen(extends = js_sys::Map)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type J;

    #[derive(core::cmp::PartialEq, ::std::cmp::Eq)]
    pub type K;

    #[derive(Clone, Debug, PartialEq)]
    pub type PartialOnly;

    #[derive(Clone, Debug)]
    pub type NoEq;
}

fn assert_eq<T: Eq>() {}
fn assert_partial_eq<T: PartialEq>() {}
fn assert_debug<T: Debug>() {}

fn main() {
    assert_eq::<G<JsValue>>();
    assert_eq::<G<NoEq>>();
    assert_eq::<H>();
    assert_eq::<J>();
    assert_eq::<K>();
    assert_partial_eq::<PartialOnly>();
    assert_debug::<G<JsValue>>();
    assert_eq::<js_sys::Object>();
    assert_eq::<js_sys::Object<NoEq>>();
    assert_eq::<js_sys::Array>();
}
