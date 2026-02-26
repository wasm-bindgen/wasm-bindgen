#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GetNoInterfaceObject , typescript_type = "GetNoInterfaceObject")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetNoInterfaceObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetNoInterfaceObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetNoInterfaceObject`*"]
    pub type GetNoInterfaceObject;
    #[cfg(feature = "NoInterfaceObject")]
    # [wasm_bindgen (static_method_of = GetNoInterfaceObject , js_class = "GetNoInterfaceObject" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetNoInterfaceObject/get_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetNoInterfaceObject`, `NoInterfaceObject`*"]
    pub fn get() -> NoInterfaceObject;
}
