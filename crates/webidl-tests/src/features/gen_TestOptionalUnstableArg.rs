#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestOptionalUnstableArg , typescript_type = "TestOptionalUnstableArg")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestOptionalUnstableArg` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalUnstableArg)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalUnstableArg`*"]
    pub type TestOptionalUnstableArg;
    #[wasm_bindgen(catch, constructor, js_class = "TestOptionalUnstableArg")]
    #[doc = "The `new TestOptionalUnstableArg(..)` constructor, creating a new instance of `TestOptionalUnstableArg`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalUnstableArg/TestOptionalUnstableArg)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalUnstableArg`*"]
    pub fn new() -> Result<TestOptionalUnstableArg, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestOptionalUnstableArg" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalUnstableArg/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalUnstableArg`*"]
    pub fn read(this: &TestOptionalUnstableArg) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableOptions")]
    # [wasm_bindgen (method , structural , js_class = "TestOptionalUnstableArg" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestOptionalUnstableArg/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestOptionalUnstableArg`, `UnstableOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn read_with_options(
        this: &TestOptionalUnstableArg,
        options: &UnstableOptions,
    ) -> ::js_sys::Promise<::js_sys::JsString>;
}
