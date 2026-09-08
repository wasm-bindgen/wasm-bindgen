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
        js_name = "HTMLParamElement",
        typescript_type = "HTMLParamElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlParamElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub type HtmlParamElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn name(this: &HtmlParamElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "name")]
    #[doc = "Like `name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn name_js_string(this: &HtmlParamElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLParamElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_name(this: &HtmlParamElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value(this: &HtmlParamElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "value")]
    #[doc = "Like `value()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value_js_string(this: &HtmlParamElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLParamElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_value(this: &HtmlParamElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn type_(this: &HtmlParamElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "type")]
    #[doc = "Like `type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn type_js_string(this: &HtmlParamElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLParamElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_type(this: &HtmlParamElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "valueType")]
    #[doc = "Getter for the `valueType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value_type(this: &HtmlParamElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLParamElement", js_name = "valueType")]
    #[doc = "Like `value_type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value_type_js_string(this: &HtmlParamElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLParamElement", js_name = "valueType")]
    #[doc = "Setter for the `valueType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_value_type(this: &HtmlParamElement, value: &str);
}
