#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestReadOnlyMapLike , typescript_type = "TestReadOnlyMapLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestReadOnlyMapLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub type TestReadOnlyMapLike;
    # [wasm_bindgen (structural , method , getter , js_class = "TestReadOnlyMapLike" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn size(this: &TestReadOnlyMapLike) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "TestReadOnlyMapLike")]
    #[doc = "The `new TestReadOnlyMapLike(..)` constructor, creating a new instance of `TestReadOnlyMapLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/TestReadOnlyMapLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn new() -> Result<TestReadOnlyMapLike, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestReadOnlyMapLike" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn for_each(
        this: &TestReadOnlyMapLike,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlyMapLike" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn get(this: &TestReadOnlyMapLike, key: &str) -> Option<u32>;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlyMapLike" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn has(this: &TestReadOnlyMapLike, key: &str) -> bool;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlyMapLike" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn entries(this: &TestReadOnlyMapLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlyMapLike" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn keys(this: &TestReadOnlyMapLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadOnlyMapLike" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadOnlyMapLike/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadOnlyMapLike`*"]
    pub fn values(this: &TestReadOnlyMapLike) -> ::js_sys::Iterator;
}
