use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = Snippet)]
pub fn foo() {}

#[wasm_bindgen(module = "some-library")]
extern "C" {
    #[wasm_bindgen(js_name = "OriginalName", reexport = "RenamedClass")]
    pub type MyClass;
}

#[wasm_bindgen(module = "default-export-lib")]
extern "C" {
    #[wasm_bindgen(js_name = default, reexport)]
    pub type DefaultExport;
}

#[wasm_bindgen(module = "helpers")]
extern "C" {
    #[wasm_bindgen(reexport)]
    pub fn helperFunction(x: i32) -> i32;
}

#[wasm_bindgen(module = "utils")]
extern "C" {
    #[wasm_bindgen(js_name = "original", reexport = "renamedFunction")]
    pub fn original_fn() -> String;
}

#[wasm_bindgen(module = "weird-exports")]
extern "C" {
    #[wasm_bindgen(js_name = "invalid-name", reexport)]
    pub type InvalidIdentifier;
}

#[wasm_bindgen(module = "types-lib")]
extern "C" {
    #[wasm_bindgen(typescript_type = "CustomType", reexport)]
    pub type CustomType;
}

#[wasm_bindgen(inline_js = "
import { WorkerEntrypoint } from 'cloudflare:workers';
export class Snippet extends WorkerEntrypoint {
}
")]
extern "C" {
    #[wasm_bindgen(reexport)]
    pub type Snippet;
}

#[wasm_bindgen(module = "constants")]
extern "C" {
    #[wasm_bindgen(reexport)]
    pub static MY_CONSTANT: JsValue;
}

#[wasm_bindgen(module = "config")]
extern "C" {
    #[wasm_bindgen(js_name = "original_config", reexport = "renamedConfig")]
    pub static CONFIG: JsValue;
}
