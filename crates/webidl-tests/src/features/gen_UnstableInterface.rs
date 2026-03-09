#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = UnstableInterface , typescript_type = "UnstableInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnstableInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UnstableInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UnstableInterface;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "UnstableInterface" , js_name = enum_value)]
    #[doc = "The `enum_value()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UnstableInterface/enum_value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn enum_value(this: &UnstableInterface) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableDictionary")]
    # [wasm_bindgen (method , structural , js_class = "UnstableInterface" , js_name = enum_value)]
    #[doc = "The `enum_value()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UnstableInterface/enum_value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableDictionary`, `UnstableInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn enum_value_with_unstable_dictionary(
        this: &UnstableInterface,
        unstable_dictionary: &UnstableDictionary,
    ) -> u32;
}
