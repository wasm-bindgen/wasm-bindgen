#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestReadWriteMapLike , typescript_type = "TestReadWriteMapLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestReadWriteMapLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub type TestReadWriteMapLike;
    # [wasm_bindgen (structural , method , getter , js_class = "TestReadWriteMapLike" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn size(this: &TestReadWriteMapLike) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "TestReadWriteMapLike")]
    #[doc = "The `new TestReadWriteMapLike(..)` constructor, creating a new instance of `TestReadWriteMapLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/TestReadWriteMapLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn new() -> Result<TestReadWriteMapLike, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = clear)]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn clear(this: &TestReadWriteMapLike);
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn delete(this: &TestReadWriteMapLike, key: &str) -> bool;
    # [wasm_bindgen (catch , method , structural , js_class = "TestReadWriteMapLike" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn for_each(
        this: &TestReadWriteMapLike,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn get(this: &TestReadWriteMapLike, key: &str) -> Option<u32>;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn has(this: &TestReadWriteMapLike, key: &str) -> bool;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = set)]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn set(this: &TestReadWriteMapLike, key: &str, value: u32) -> TestReadWriteMapLike;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn entries(this: &TestReadWriteMapLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn keys(this: &TestReadWriteMapLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteMapLike" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteMapLike/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteMapLike`*"]
    pub fn values(this: &TestReadWriteMapLike) -> ::js_sys::Iterator;
}
