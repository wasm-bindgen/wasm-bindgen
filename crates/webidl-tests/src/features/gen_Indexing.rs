#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Indexing , typescript_type = "Indexing")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Indexing` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Indexing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Indexing`*"]
    pub type Indexing;
    #[wasm_bindgen(catch, constructor, js_class = "Indexing")]
    #[doc = "The `new Indexing(..)` constructor, creating a new instance of `Indexing`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Indexing/Indexing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Indexing`*"]
    pub fn new() -> Result<Indexing, JsValue>;
    #[wasm_bindgen(method, structural, js_class = "Indexing", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Indexing`*"]
    pub fn get(this: &Indexing, index: u32) -> Option<i16>;
    #[wasm_bindgen(method, structural, js_class = "Indexing", indexing_setter)]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Indexing`*"]
    pub fn set(this: &Indexing, index: u32, value: i16);
    #[wasm_bindgen(method, structural, js_class = "Indexing", indexing_deleter)]
    #[doc = "Indexing deleter. As in the literal Javascript `delete this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Indexing`*"]
    pub fn delete(this: &Indexing, index: u32);
}
