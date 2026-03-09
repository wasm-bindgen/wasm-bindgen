#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = InvokeCallback , typescript_type = "InvokeCallback")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InvokeCallback` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InvokeCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InvokeCallback`*"]
    pub type InvokeCallback;
    #[wasm_bindgen(catch, constructor, js_class = "InvokeCallback")]
    #[doc = "The `new InvokeCallback(..)` constructor, creating a new instance of `InvokeCallback`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InvokeCallback/InvokeCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InvokeCallback`*"]
    pub fn new() -> Result<InvokeCallback, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "InvokeCallback" , js_name = callAdd)]
    #[doc = "The `callAdd()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InvokeCallback/callAdd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InvokeCallback`*"]
    pub fn call_add(this: &InvokeCallback, callback: &::js_sys::Function) -> i32;
    # [wasm_bindgen (method , structural , js_class = "InvokeCallback" , js_name = callRepeat)]
    #[doc = "The `callRepeat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InvokeCallback/callRepeat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InvokeCallback`*"]
    pub fn call_repeat(
        this: &InvokeCallback,
        callback: &::js_sys::Function,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "InvokeCallback" , js_name = invoke)]
    #[doc = "The `invoke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InvokeCallback/invoke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InvokeCallback`*"]
    pub fn invoke(this: &InvokeCallback, callback: &::js_sys::Function);
}
