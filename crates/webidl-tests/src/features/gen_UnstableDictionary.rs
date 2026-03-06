#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UnstableDictionary)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnstableDictionary` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableDictionary`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UnstableDictionary;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableEnum")]
    #[doc = "Get the `unstableEnum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableDictionary`, `UnstableEnum`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unstableEnum")]
    pub fn get_unstable_enum(this: &UnstableDictionary) -> Option<UnstableEnum>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableEnum")]
    #[doc = "Change the `unstableEnum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableDictionary`, `UnstableEnum`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unstableEnum")]
    pub fn set_unstable_enum(this: &UnstableDictionary, val: UnstableEnum);
}
#[cfg(web_sys_unstable_apis)]
impl UnstableDictionary {
    #[doc = "Construct a new `UnstableDictionary`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnstableDictionary`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UnstableEnum")]
    #[deprecated = "Use `set_unstable_enum()` instead."]
    pub fn unstable_enum(&mut self, val: UnstableEnum) -> &mut Self {
        self.set_unstable_enum(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UnstableDictionary {
    fn default() -> Self {
        Self::new()
    }
}
