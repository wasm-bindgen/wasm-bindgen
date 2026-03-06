#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestDoubleIterable , typescript_type = "TestDoubleIterable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestDoubleIterable` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub type TestDoubleIterable;
    #[wasm_bindgen(catch, constructor, js_class = "TestDoubleIterable")]
    #[doc = "The `new TestDoubleIterable(..)` constructor, creating a new instance of `TestDoubleIterable`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable/TestDoubleIterable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub fn new() -> Result<TestDoubleIterable, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestDoubleIterable" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub fn for_each(
        this: &TestDoubleIterable,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestDoubleIterable" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub fn entries(this: &TestDoubleIterable) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestDoubleIterable" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub fn keys(this: &TestDoubleIterable) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestDoubleIterable" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestDoubleIterable/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestDoubleIterable`*"]
    pub fn values(this: &TestDoubleIterable) -> ::js_sys::Iterator;
}
