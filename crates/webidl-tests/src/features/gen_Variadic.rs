#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Variadic , typescript_type = "Variadic")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Variadic` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub type Variadic;
    #[wasm_bindgen(catch, constructor, js_class = "Variadic")]
    #[doc = "The `new Variadic(..)` constructor, creating a new instance of `Variadic`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/Variadic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn new() -> Result<Variadic, JsValue>;
    # [wasm_bindgen (method , structural , variadic , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects(this: &Variadic, items: &::js_sys::Array) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_0(this: &Variadic) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_1(this: &Variadic, items_1: &::js_sys::Object) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_2(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_3(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
        items_3: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_4(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
        items_3: &::js_sys::Object,
        items_4: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_5(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
        items_3: &::js_sys::Object,
        items_4: &::js_sys::Object,
        items_5: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_6(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
        items_3: &::js_sys::Object,
        items_4: &::js_sys::Object,
        items_5: &::js_sys::Object,
        items_6: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = countObjects)]
    #[doc = "The `countObjects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/countObjects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn count_objects_7(
        this: &Variadic,
        items_1: &::js_sys::Object,
        items_2: &::js_sys::Object,
        items_3: &::js_sys::Object,
        items_4: &::js_sys::Object,
        items_5: &::js_sys::Object,
        items_6: &::js_sys::Object,
        items_7: &::js_sys::Object,
    ) -> i32;
    # [wasm_bindgen (method , structural , variadic , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum(this: &Variadic, values: &::js_sys::Array) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_0(this: &Variadic) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_1(this: &Variadic, values_1: i16) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_2(this: &Variadic, values_1: i16, values_2: i16) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_3(this: &Variadic, values_1: i16, values_2: i16, values_3: i16) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_4(
        this: &Variadic,
        values_1: i16,
        values_2: i16,
        values_3: i16,
        values_4: i16,
    ) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_5(
        this: &Variadic,
        values_1: i16,
        values_2: i16,
        values_3: i16,
        values_4: i16,
        values_5: i16,
    ) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_6(
        this: &Variadic,
        values_1: i16,
        values_2: i16,
        values_3: i16,
        values_4: i16,
        values_5: i16,
        values_6: i16,
    ) -> i16;
    # [wasm_bindgen (method , structural , js_class = "Variadic" , js_name = sum)]
    #[doc = "The `sum()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Variadic/sum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Variadic`*"]
    pub fn sum_7(
        this: &Variadic,
        values_1: i16,
        values_2: i16,
        values_3: i16,
        values_4: i16,
        values_5: i16,
        values_6: i16,
        values_7: i16,
    ) -> i16;
}
