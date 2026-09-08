#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "UiEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "InputEvent",
        typescript_type = "InputEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub type InputEvent;
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "isComposing")]
    #[doc = "Getter for the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn is_composing(this: &InputEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "inputType")]
    #[doc = "Getter for the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/inputType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn input_type(this: &InputEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "inputType")]
    #[doc = "Like `input_type()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/inputType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn input_type_js_string(this: &InputEvent) -> ::js_sys::JsString;
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn data(this: &InputEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "data")]
    #[doc = "Like `data()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn data_js_string(this: &InputEvent) -> Option<::js_sys::JsString>;
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, getter, js_class = "InputEvent", js_name = "dataTransfer")]
    #[doc = "Getter for the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/dataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEvent`*"]
    pub fn data_transfer(this: &InputEvent) -> Option<DataTransfer>;
    #[wasm_bindgen(catch, constructor, js_class = "InputEvent")]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn new(type_: &str) -> Result<InputEvent, JsValue>;
    #[cfg(feature = "InputEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "InputEvent")]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`, `InputEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &InputEventInit,
    ) -> Result<InputEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "InputEvent", js_name = "getTargetRanges")]
    #[doc = "The `getTargetRanges()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/getTargetRanges)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn get_target_ranges(this: &InputEvent) -> ::js_sys::Array;
}
