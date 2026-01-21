#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceMark , typescript_type = "PerformanceMark")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceMark` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    pub type PerformanceMark;
    # [wasm_bindgen (structural , method , getter , js_class = "PerformanceMark" , js_name = detail)]
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    pub fn detail(this: &PerformanceMark) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, constructor, js_class = "PerformanceMark")]
    #[doc = "The `new PerformanceMark(..)` constructor, creating a new instance of `PerformanceMark`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    pub fn new(mark_name: &str) -> Result<PerformanceMark, JsValue>;
    #[cfg(feature = "PerformanceMarkOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "PerformanceMark")]
    #[doc = "The `new PerformanceMark(..)` constructor, creating a new instance of `PerformanceMark`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`, `PerformanceMarkOptions`*"]
    pub fn new_with_mark_options(
        mark_name: &str,
        mark_options: &PerformanceMarkOptions,
    ) -> Result<PerformanceMark, JsValue>;
}
