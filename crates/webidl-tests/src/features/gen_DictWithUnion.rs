#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DictWithUnion)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DictWithUnion` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DictWithUnion;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "optionalView")]
    pub fn get_optional_view(this: &DictWithUnion) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TypeA")]
    #[doc = "Change the `optionalView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "optionalView")]
    pub fn set_optional_view(this: &DictWithUnion, val: &TypeA);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TypeB")]
    #[doc = "Change the `optionalView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "optionalView")]
    pub fn set_optional_view_type_b(this: &DictWithUnion, val: &TypeB);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &DictWithUnion) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TypeA")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "view")]
    pub fn set_view(this: &DictWithUnion, val: &TypeA);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TypeB")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "view")]
    pub fn set_view_type_b(this: &DictWithUnion, val: &TypeB);
}
#[cfg(web_sys_unstable_apis)]
impl DictWithUnion {
    #[cfg(feature = "TypeA")]
    #[doc = "Construct a new `DictWithUnion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`, `TypeA`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(view: &TypeA) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_view(view);
        ret
    }
    #[cfg(feature = "TypeB")]
    #[doc = "Construct a new `DictWithUnion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithUnion`, `TypeB`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_type_b(view: &TypeB) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_view_type_b(view);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_optional_view()` instead."]
    pub fn optional_view(&mut self, val: &TypeA) -> &mut Self {
        self.set_optional_view(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_view()` instead."]
    pub fn view(&mut self, val: &TypeA) -> &mut Self {
        self.set_view(val);
        self
    }
}
