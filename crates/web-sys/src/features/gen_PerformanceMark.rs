#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceMark , typescript_type = "PerformanceMark")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `PerformanceMark` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    pub type PerformanceMark;
}
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<PerformanceEntry> for PerformanceMark {}
