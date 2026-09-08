#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLHeadingElement",
        typescript_type = "HTMLHeadingElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlHeadingElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub type HtmlHeadingElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLHeadingElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub fn align(this: &HtmlHeadingElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLHeadingElement", js_name = "align")]
    #[doc = "Like `align()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub fn align_js_string(this: &HtmlHeadingElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLHeadingElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub fn set_align(this: &HtmlHeadingElement, value: &str);
}
