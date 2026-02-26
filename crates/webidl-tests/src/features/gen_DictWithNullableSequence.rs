#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DictWithNullableSequence)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DictWithNullableSequence` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithNullableSequence`*"]
    pub type DictWithNullableSequence;
    #[doc = "Get the `nullableSequence` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithNullableSequence`*"]
    #[wasm_bindgen(method, getter = "nullableSequence")]
    pub fn get_nullable_sequence(this: &DictWithNullableSequence) -> Option<::js_sys::Array>;
    #[doc = "Change the `nullableSequence` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithNullableSequence`*"]
    #[wasm_bindgen(method, setter = "nullableSequence")]
    pub fn set_nullable_sequence(this: &DictWithNullableSequence, val: &::wasm_bindgen::JsValue);
}
impl DictWithNullableSequence {
    #[doc = "Construct a new `DictWithNullableSequence`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DictWithNullableSequence`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_nullable_sequence()` instead."]
    pub fn nullable_sequence(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_nullable_sequence(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
}
impl Default for DictWithNullableSequence {
    fn default() -> Self {
        Self::new()
    }
}
