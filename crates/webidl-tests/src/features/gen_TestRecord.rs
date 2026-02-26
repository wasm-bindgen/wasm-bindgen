#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestRecord , typescript_type = "TestRecord")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestRecord` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestRecord`*"]
    pub type TestRecord;
    #[wasm_bindgen(catch, constructor, js_class = "TestRecord")]
    #[doc = "The `new TestRecord(..)` constructor, creating a new instance of `TestRecord`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestRecord/TestRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestRecord`*"]
    pub fn new() -> Result<TestRecord, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestRecord" , js_name = getNumberRecord)]
    #[doc = "The `getNumberRecord()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestRecord/getNumberRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestRecord`*"]
    pub fn get_number_record(this: &TestRecord) -> ::js_sys::Object;
    # [wasm_bindgen (method , structural , js_class = "TestRecord" , js_name = getStringRecord)]
    #[doc = "The `getStringRecord()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestRecord/getStringRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestRecord`*"]
    pub fn get_string_record(this: &TestRecord) -> ::js_sys::Object;
    # [wasm_bindgen (method , structural , js_class = "TestRecord" , js_name = setRecord)]
    #[doc = "The `setRecord()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestRecord/setRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestRecord`*"]
    pub fn set_record(this: &TestRecord, data: &::js_sys::Object);
}
