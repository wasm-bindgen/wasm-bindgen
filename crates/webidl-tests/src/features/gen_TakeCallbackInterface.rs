#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TakeCallbackInterface , typescript_type = "TakeCallbackInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TakeCallbackInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TakeCallbackInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TakeCallbackInterface`*"]
    pub type TakeCallbackInterface;
    #[wasm_bindgen(catch, constructor, js_class = "TakeCallbackInterface")]
    #[doc = "The `new TakeCallbackInterface(..)` constructor, creating a new instance of `TakeCallbackInterface`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TakeCallbackInterface/TakeCallbackInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TakeCallbackInterface`*"]
    pub fn new() -> Result<TakeCallbackInterface, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TakeCallbackInterface" , js_name = a)]
    #[doc = "The `a()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TakeCallbackInterface/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TakeCallbackInterface`*"]
    pub fn a_with_callback(this: &TakeCallbackInterface, arg: &::js_sys::Function);
    #[cfg(feature = "CallbackInterface1")]
    # [wasm_bindgen (method , structural , js_class = "TakeCallbackInterface" , js_name = a)]
    #[doc = "The `a()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TakeCallbackInterface/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CallbackInterface1`, `TakeCallbackInterface`*"]
    pub fn a_with_callback_interface1(this: &TakeCallbackInterface, arg: &CallbackInterface1);
    #[cfg(feature = "CallbackInterface2")]
    # [wasm_bindgen (method , structural , js_class = "TakeCallbackInterface" , js_name = b)]
    #[doc = "The `b()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TakeCallbackInterface/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CallbackInterface2`, `TakeCallbackInterface`*"]
    pub fn b(this: &TakeCallbackInterface, arg: &CallbackInterface2);
}
