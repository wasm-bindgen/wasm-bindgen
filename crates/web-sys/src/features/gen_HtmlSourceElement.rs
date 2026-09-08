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
        js_name = "HTMLSourceElement",
        typescript_type = "HTMLSourceElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSourceElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub type HtmlSourceElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn src(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "src")]
    #[doc = "Like `src()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn src_js_string(this: &HtmlSourceElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_src(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn type_(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "type")]
    #[doc = "Like `type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn type_js_string(this: &HtmlSourceElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_type(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "srcset")]
    #[doc = "Getter for the `srcset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn srcset(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "srcset")]
    #[doc = "Like `srcset()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn srcset_js_string(this: &HtmlSourceElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "srcset")]
    #[doc = "Setter for the `srcset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_srcset(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "sizes")]
    #[doc = "Getter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn sizes(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "sizes")]
    #[doc = "Like `sizes()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn sizes_js_string(this: &HtmlSourceElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "sizes")]
    #[doc = "Setter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_sizes(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn media(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "media")]
    #[doc = "Like `media()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn media_js_string(this: &HtmlSourceElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "media")]
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_media(this: &HtmlSourceElement, value: &str);
}
