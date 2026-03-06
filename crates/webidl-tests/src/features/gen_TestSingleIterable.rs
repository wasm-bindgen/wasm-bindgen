#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestSingleIterable , typescript_type = "TestSingleIterable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestSingleIterable` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub type TestSingleIterable;
    # [wasm_bindgen (structural , method , getter , js_class = "TestSingleIterable" , js_name = length)]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn length(this: &TestSingleIterable) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "TestSingleIterable")]
    #[doc = "The `new TestSingleIterable(..)` constructor, creating a new instance of `TestSingleIterable`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/TestSingleIterable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn new() -> Result<TestSingleIterable, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestSingleIterable" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn for_each(
        this: &TestSingleIterable,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, structural, js_class = "TestSingleIterable", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn get(this: &TestSingleIterable, index: u32) -> Option<::alloc::string::String>;
    # [wasm_bindgen (method , structural , js_class = "TestSingleIterable" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn entries(this: &TestSingleIterable) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestSingleIterable" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn keys(this: &TestSingleIterable) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestSingleIterable" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestSingleIterable/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestSingleIterable`*"]
    pub fn values(this: &TestSingleIterable) -> ::js_sys::Iterator;
}
