#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Unforgeable , typescript_type = "Unforgeable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Unforgeable` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Unforgeable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Unforgeable`*"]
    pub type Unforgeable;
    # [wasm_bindgen (structural , method , getter , js_class = "Unforgeable" , js_name = uno)]
    #[doc = "Getter for the `uno` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Unforgeable/uno)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Unforgeable`*"]
    pub fn uno(this: &Unforgeable) -> i16;
    # [wasm_bindgen (structural , method , getter , js_class = "Unforgeable" , js_name = dos)]
    #[doc = "Getter for the `dos` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Unforgeable/dos)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Unforgeable`*"]
    pub fn dos(this: &Unforgeable) -> i16;
    #[wasm_bindgen(catch, constructor, js_class = "Unforgeable")]
    #[doc = "The `new Unforgeable(..)` constructor, creating a new instance of `Unforgeable`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Unforgeable/Unforgeable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Unforgeable`*"]
    pub fn new() -> Result<Unforgeable, JsValue>;
}
