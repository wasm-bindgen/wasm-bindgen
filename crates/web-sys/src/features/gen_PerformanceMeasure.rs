#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceMeasure , typescript_type = "PerformanceMeasure")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceMeasure` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMeasure`*"]
    pub type PerformanceMeasure;
    # [wasm_bindgen (structural , method , getter , js_class = "PerformanceMeasure" , js_name = detail)]
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMeasure`*"]
    pub fn detail(this: &PerformanceMeasure) -> ::wasm_bindgen::JsValue;
}
