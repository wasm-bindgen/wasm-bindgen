#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestCallbacks , typescript_type = "TestCallbacks")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestCallbacks` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub type TestCallbacks;
    #[wasm_bindgen(catch, constructor, js_class = "TestCallbacks")]
    #[doc = "The `new TestCallbacks(..)` constructor, creating a new instance of `TestCallbacks`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/TestCallbacks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn new() -> Result<TestCallbacks, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeBinaryOp)]
    #[doc = "The `invokeBinaryOp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeBinaryOp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_binary_op(
        this: &TestCallbacks,
        callback: &::js_sys::Function,
        a: i32,
        b: i32,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeNumberCallback)]
    #[doc = "The `invokeNumberCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeNumberCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_number_callback(this: &TestCallbacks, callback: &::js_sys::Function, value: i32);
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeObjectCallback)]
    #[doc = "The `invokeObjectCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeObjectCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_object_callback(
        this: &TestCallbacks,
        callback: &::js_sys::Function,
        data: &::js_sys::Object,
    );
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeSequenceCallback)]
    #[doc = "The `invokeSequenceCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeSequenceCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_sequence_callback(
        this: &TestCallbacks,
        callback: &::js_sys::Function,
        input: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Array;
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeStringTransformer)]
    #[doc = "The `invokeStringTransformer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeStringTransformer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_string_transformer(
        this: &TestCallbacks,
        callback: &::js_sys::Function,
        input: &str,
    ) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "TestCallbacks" , js_name = invokeVoidCallback)]
    #[doc = "The `invokeVoidCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestCallbacks/invokeVoidCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestCallbacks`*"]
    pub fn invoke_void_callback(this: &TestCallbacks, callback: &::js_sys::Function);
}
