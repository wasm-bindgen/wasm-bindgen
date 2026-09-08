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
        js_name = "HTMLQuoteElement",
        typescript_type = "HTMLQuoteElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlQuoteElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub type HtmlQuoteElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLQuoteElement", js_name = "cite")]
    #[doc = "Getter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn cite(this: &HtmlQuoteElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLQuoteElement", js_name = "cite")]
    #[doc = "Like `cite()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn cite_js_string(this: &HtmlQuoteElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLQuoteElement", js_name = "cite")]
    #[doc = "Setter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn set_cite(this: &HtmlQuoteElement, value: &str);
}
