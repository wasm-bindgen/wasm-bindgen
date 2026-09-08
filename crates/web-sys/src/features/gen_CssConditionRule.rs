#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "CssGroupingRule",
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSConditionRule",
        typescript_type = "CSSConditionRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssConditionRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub type CssConditionRule;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSConditionRule",
        js_name = "conditionText"
    )]
    #[doc = "Getter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub fn condition_text(this: &CssConditionRule) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSConditionRule",
        js_name = "conditionText"
    )]
    #[doc = "Like `condition_text()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub fn condition_text_js_string(this: &CssConditionRule) -> ::js_sys::JsString;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CSSConditionRule",
        js_name = "conditionText"
    )]
    #[doc = "Setter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub fn set_condition_text(this: &CssConditionRule, value: &str);
}
