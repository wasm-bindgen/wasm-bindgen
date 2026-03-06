#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Overloads , typescript_type = "Overloads")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Overloads` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub type Overloads;
    #[wasm_bindgen(catch, constructor, js_class = "Overloads")]
    #[doc = "The `new Overloads(..)` constructor, creating a new instance of `Overloads`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/Overloads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn new() -> Result<Overloads, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "Overloads" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn foo(this: &Overloads);
    # [wasm_bindgen (method , structural , js_class = "Overloads" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn foo_with_arg(this: &Overloads, arg: &str);
    # [wasm_bindgen (method , structural , js_class = "Overloads" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn foo_with_arg_and_i32(this: &Overloads, arg: &str, a: i32);
    # [wasm_bindgen (method , structural , js_class = "Overloads" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn foo_with_arg_and_f32(this: &Overloads, arg: &str, b: f32);
    # [wasm_bindgen (method , structural , js_class = "Overloads" , js_name = foo)]
    #[doc = "The `foo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Overloads/foo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Overloads`*"]
    pub fn foo_with_arg_and_i16(this: &Overloads, arg: &str, b: i16);
}
