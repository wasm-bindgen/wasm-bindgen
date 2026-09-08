#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(experimental_generic_mono)]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Directory",
        typescript_type = "Directory"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Directory` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub type Directory;
    #[wasm_bindgen(catch, method, getter, js_class = "Directory", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn name(this: &Directory) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Directory", js_name = "name")]
    #[doc = "Like `name()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn name_js_string(this: &Directory) -> Result<::js_sys::JsString, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Directory", js_name = "path")]
    #[doc = "Getter for the `path` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/path)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn path(this: &Directory) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Directory", js_name = "path")]
    #[doc = "Like `path()`, but returning a `js_sys::JsString` handle to the string rather than copying it into wasm memory."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/path)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn path_js_string(this: &Directory) -> Result<::js_sys::JsString, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Directory", js_name = "getFiles")]
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn get_files(this: &Directory) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Directory", js_name = "getFiles")]
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn get_files_with_recursive_flag(
        this: &Directory,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Directory",
        js_name = "getFilesAndDirectories"
    )]
    #[doc = "The `getFilesAndDirectories()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFilesAndDirectories)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
    pub fn get_files_and_directories(this: &Directory) -> Result<::js_sys::Promise, JsValue>;
}
