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
        js_name = "HTMLLinkElement",
        typescript_type = "HTMLLinkElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLinkElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub type HtmlLinkElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn disabled(this: &HtmlLinkElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_disabled(this: &HtmlLinkElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn href(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "href")]
    #[doc = "Like `href()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn href_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "href")]
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_href(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "crossOrigin")]
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn cross_origin(this: &HtmlLinkElement) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "crossOrigin")]
    #[doc = "Like `cross_origin()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn cross_origin_js_string(this: &HtmlLinkElement) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "crossOrigin")]
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_cross_origin(this: &HtmlLinkElement, value: Option<&str>);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "rel")]
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rel(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "rel")]
    #[doc = "Like `rel()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rel_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "rel")]
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_rel(this: &HtmlLinkElement, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "relList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    pub fn rel_list(this: &HtmlLinkElement) -> DomTokenList;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn media(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "media")]
    #[doc = "Like `media()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn media_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "media")]
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_media(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "hreflang")]
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn hreflang(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "hreflang")]
    #[doc = "Like `hreflang()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn hreflang_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "hreflang")]
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_hreflang(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn type_(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "type")]
    #[doc = "Like `type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn type_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_type(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLLinkElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn referrer_policy(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLLinkElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Like `referrer_policy()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn referrer_policy_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLLinkElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_referrer_policy(this: &HtmlLinkElement, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "sizes")]
    #[doc = "Getter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    pub fn sizes(this: &HtmlLinkElement) -> DomTokenList;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "charset")]
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn charset(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "charset")]
    #[doc = "Like `charset()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn charset_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "charset")]
    #[doc = "Setter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_charset(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "rev")]
    #[doc = "Getter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rev(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "rev")]
    #[doc = "Like `rev()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rev_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "rev")]
    #[doc = "Setter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_rev(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn target(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "target")]
    #[doc = "Like `target()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn target_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_target(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "integrity")]
    #[doc = "Getter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn integrity(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "integrity")]
    #[doc = "Like `integrity()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn integrity_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "integrity")]
    #[doc = "Setter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_integrity(this: &HtmlLinkElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "as")]
    #[doc = "Getter for the `as` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn as_(this: &HtmlLinkElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "as")]
    #[doc = "Like `as()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn as_js_string(this: &HtmlLinkElement) -> ::js_sys::JsString;
    #[wasm_bindgen(method, setter, js_class = "HTMLLinkElement", js_name = "as")]
    #[doc = "Setter for the `as` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_as(this: &HtmlLinkElement, value: &str);
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "HTMLLinkElement", js_name = "sheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`, `StyleSheet`*"]
    pub fn sheet(this: &HtmlLinkElement) -> Option<StyleSheet>;
}
