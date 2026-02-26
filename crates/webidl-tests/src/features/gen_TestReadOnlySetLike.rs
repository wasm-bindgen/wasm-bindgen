#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestReadOnlySetLike , typescript_type = "TestReadOnlySetLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestReadOnlySetLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub type TestReadOnlySetLike;
    # [wasm_bindgen (structural , method , getter , js_class = "TestReadOnlySetLike" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn size(this: &TestReadOnlySetLike) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "TestReadOnlySetLike")]
    #[doc = "The `new TestReadOnlySetLike(..)` constructor, creating a new instance of `TestReadOnlySetLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/TestReadOnlySetLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn new() -> Result<TestReadOnlySetLike, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestReadOnlySetLike" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn for_each(
        this: &TestReadOnlySetLike,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlySetLike" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn has(this: &TestReadOnlySetLike, value: &str) -> bool;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlySetLike" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn entries(this: &TestReadOnlySetLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlySetLike" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn keys(this: &TestReadOnlySetLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlySetLike" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlySetLike/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlySetLike`*"]
    pub fn values(this: &TestReadOnlySetLike) -> ::js_sys::Iterator;
}
