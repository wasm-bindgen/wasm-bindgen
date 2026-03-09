#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestOptionalCallbacks , typescript_type = "TestOptionalCallbacks")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestOptionalCallbacks` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalCallbacks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalCallbacks`*"]
    pub type TestOptionalCallbacks;
    #[wasm_bindgen(catch, constructor, js_class = "TestOptionalCallbacks")]
    #[doc = "The `new TestOptionalCallbacks(..)` constructor, creating a new instance of `TestOptionalCallbacks`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalCallbacks/TestOptionalCallbacks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalCallbacks`*"]
    pub fn new() -> Result<TestOptionalCallbacks, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestOptionalCallbacks" , js_name = doWork)]
    #[doc = "The `doWork()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalCallbacks/doWork)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalCallbacks`*"]
    pub fn do_work(this: &TestOptionalCallbacks, input: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "TestOptionalCallbacks" , js_name = doWork)]
    #[doc = "The `doWork()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalCallbacks/doWork)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalCallbacks`*"]
    pub fn do_work_with_success_callback(
        this: &TestOptionalCallbacks,
        input: &str,
        success_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "TestOptionalCallbacks" , js_name = doWork)]
    #[doc = "The `doWork()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalCallbacks/doWork)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalCallbacks`*"]
    pub fn do_work_with_success_callback_and_error_callback(
        this: &TestOptionalCallbacks,
        input: &str,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
}
