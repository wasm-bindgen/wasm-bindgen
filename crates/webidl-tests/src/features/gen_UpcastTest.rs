#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UpcastTest , typescript_type = "UpcastTest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UpcastTest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UpcastTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UpcastTest`*"]
    pub type UpcastTest;
    #[cfg(feature = "BaseType")]
    # [wasm_bindgen (static_method_of = UpcastTest , js_class = "UpcastTest" , js_name = processBase)]
    #[doc = "The `processBase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UpcastTest/processBase_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseType`, `UpcastTest`*"]
    pub fn process_base(obj: &BaseType) -> ::alloc::string::String;
    #[cfg(feature = "ChildType")]
    # [wasm_bindgen (static_method_of = UpcastTest , js_class = "UpcastTest" , js_name = processChild)]
    #[doc = "The `processChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UpcastTest/processChild_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChildType`, `UpcastTest`*"]
    pub fn process_child(obj: &ChildType) -> ::alloc::string::String;
}
