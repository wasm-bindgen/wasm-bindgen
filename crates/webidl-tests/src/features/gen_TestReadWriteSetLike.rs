#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestReadWriteSetLike , typescript_type = "TestReadWriteSetLike")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestReadWriteSetLike` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub type TestReadWriteSetLike;
    # [wasm_bindgen (structural , method , getter , js_class = "TestReadWriteSetLike" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn size(this: &TestReadWriteSetLike) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "TestReadWriteSetLike")]
    #[doc = "The `new TestReadWriteSetLike(..)` constructor, creating a new instance of `TestReadWriteSetLike`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/TestReadWriteSetLike)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn new() -> Result<TestReadWriteSetLike, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = add)]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn add(this: &TestReadWriteSetLike, value: &str) -> TestReadWriteSetLike;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = clear)]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn clear(this: &TestReadWriteSetLike);
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn delete(this: &TestReadWriteSetLike, value: &str) -> bool;
    # [wasm_bindgen (catch , method , structural , js_class = "TestReadWriteSetLike" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn for_each(
        this: &TestReadWriteSetLike,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn has(this: &TestReadWriteSetLike, value: &str) -> bool;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn entries(this: &TestReadWriteSetLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn keys(this: &TestReadWriteSetLike) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "TestReadWriteSetLike" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestReadWriteSetLike/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestReadWriteSetLike`*"]
    pub fn values(this: &TestReadWriteSetLike) -> ::js_sys::Iterator;
}
