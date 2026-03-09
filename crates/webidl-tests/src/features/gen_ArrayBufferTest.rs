#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ArrayBufferTest , typescript_type = "ArrayBufferTest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ArrayBufferTest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ArrayBufferTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ArrayBufferTest`*"]
    pub type ArrayBufferTest;
    #[wasm_bindgen(catch, constructor, js_class = "ArrayBufferTest")]
    #[doc = "The `new ArrayBufferTest(..)` constructor, creating a new instance of `ArrayBufferTest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ArrayBufferTest/ArrayBufferTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ArrayBufferTest`*"]
    pub fn new() -> Result<ArrayBufferTest, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "ArrayBufferTest" , js_name = getBuffer)]
    #[doc = "The `getBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ArrayBufferTest/getBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ArrayBufferTest`*"]
    pub fn get_buffer(this: &ArrayBufferTest) -> ::js_sys::ArrayBuffer;
    # [wasm_bindgen (method , structural , js_class = "ArrayBufferTest" , js_name = getDataView)]
    #[doc = "The `getDataView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ArrayBufferTest/getDataView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ArrayBufferTest`*"]
    pub fn get_data_view(this: &ArrayBufferTest) -> ::js_sys::DataView;
    # [wasm_bindgen (method , structural , js_class = "ArrayBufferTest" , js_name = setBuffer)]
    #[doc = "The `setBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ArrayBufferTest/setBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ArrayBufferTest`*"]
    pub fn set_buffer(this: &ArrayBufferTest, b: Option<&::js_sys::ArrayBuffer>);
}
