#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestPromises , typescript_type = "TestPromises")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestPromises` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestPromises)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestPromises`*"]
    pub type TestPromises;
    #[wasm_bindgen(catch, constructor, js_class = "TestPromises")]
    #[doc = "The `new TestPromises(..)` constructor, creating a new instance of `TestPromises`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestPromises/TestPromises)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestPromises`*"]
    pub fn new() -> Result<TestPromises, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestPromises" , js_name = anyPromise)]
    #[doc = "The `anyPromise()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestPromises/anyPromise)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestPromises`*"]
    pub fn any_promise(this: &TestPromises) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "TestPromises" , js_name = optionalStringPromise)]
    #[doc = "The `optionalStringPromise()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestPromises/optionalStringPromise)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestPromises`*"]
    pub fn optional_string_promise(this: &TestPromises) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "TestPromises" , js_name = stringPromise)]
    #[doc = "The `stringPromise()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestPromises/stringPromise)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestPromises`*"]
    pub fn string_promise(this: &TestPromises) -> ::js_sys::Promise;
}
