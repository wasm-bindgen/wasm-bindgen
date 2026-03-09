#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = NoInterfaceObject , typescript_type = "NoInterfaceObject")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NoInterfaceObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NoInterfaceObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NoInterfaceObject`*"]
    pub type NoInterfaceObject;
    # [wasm_bindgen (structural , method , getter , js_class = "NoInterfaceObject" , js_name = number)]
    #[doc = "Getter for the `number` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NoInterfaceObject/number)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NoInterfaceObject`*"]
    pub fn number(this: &NoInterfaceObject) -> f64;
    # [wasm_bindgen (method , structural , js_class = "NoInterfaceObject" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NoInterfaceObject/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NoInterfaceObject`*"]
    pub fn foo(this: &NoInterfaceObject);
}
