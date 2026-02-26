#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OptionalAndUnionArguments , typescript_type = "OptionalAndUnionArguments")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OptionalAndUnionArguments` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub type OptionalAndUnionArguments;
    #[wasm_bindgen(catch, constructor, js_class = "OptionalAndUnionArguments")]
    #[doc = "The `new OptionalAndUnionArguments(..)` constructor, creating a new instance of `OptionalAndUnionArguments`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/OptionalAndUnionArguments)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn new() -> Result<OptionalAndUnionArguments, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m(this: &OptionalAndUnionArguments, a: &str) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b(this: &OptionalAndUnionArguments, a: &str, b: bool) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_i16(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: i16,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_str(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: &str,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_i16_and_opt_i32(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: i16,
        d: Option<i32>,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_str_and_opt_i32(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: &str,
        d: Option<i32>,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_i16_and_opt_f64(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: i16,
        d: Option<f64>,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_str_and_opt_f64(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: &str,
        d: Option<f64>,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_i16_and_opt_bool(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: i16,
        d: Option<bool>,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "OptionalAndUnionArguments" , js_name = m)]
    #[doc = "The `m()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OptionalAndUnionArguments/m)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OptionalAndUnionArguments`*"]
    pub fn m_with_b_and_str_and_opt_bool(
        this: &OptionalAndUnionArguments,
        a: &str,
        b: bool,
        c: &str,
        d: Option<bool>,
    ) -> ::alloc::string::String;
}
