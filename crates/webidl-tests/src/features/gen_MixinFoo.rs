#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MixinFoo , typescript_type = "MixinFoo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MixinFoo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub type MixinFoo;
    # [wasm_bindgen (structural , static_method_of = MixinFoo , getter , js_class = "MixinFoo" , js_name = defaultBar)]
    #[doc = "Getter for the `defaultBar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo/defaultBar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub fn default_bar() -> i16;
    # [wasm_bindgen (structural , static_method_of = MixinFoo , setter , js_class = "MixinFoo" , js_name = defaultBar)]
    #[doc = "Setter for the `defaultBar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo/defaultBar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub fn set_default_bar(value: i16);
    # [wasm_bindgen (structural , method , getter , js_class = "MixinFoo" , js_name = bar)]
    #[doc = "Getter for the `bar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo/bar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub fn bar(this: &MixinFoo) -> i16;
    #[wasm_bindgen(catch, constructor, js_class = "MixinFoo")]
    #[doc = "The `new MixinFoo(..)` constructor, creating a new instance of `MixinFoo`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo/MixinFoo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub fn new(bar: i16) -> Result<MixinFoo, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "MixinFoo" , js_name = addToBar)]
    #[doc = "The `addToBar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MixinFoo/addToBar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MixinFoo`*"]
    pub fn add_to_bar(this: &MixinFoo, other: i16);
}
