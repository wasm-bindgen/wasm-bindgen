// Attributes on imported statics must reach the generated item, so that an
// `allow` silences lints on the static itself and a `cfg` removes it.
#![deny(non_upper_case_globals)]

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A lower-case JS global.
    #[allow(non_upper_case_globals)]
    #[wasm_bindgen(thread_local_v2)]
    pub static caches: JsValue;

    #[cfg(any())]
    #[wasm_bindgen(thread_local_v2, js_name = a)]
    pub static GATED: JsValue;

    #[cfg(not(any()))]
    #[wasm_bindgen(thread_local_v2, js_name = b)]
    pub static GATED: JsValue;

    #[allow(non_upper_case_globals)]
    #[wasm_bindgen(thread_local_v2, static_string)]
    pub static string: JsString = "string";
}

fn main() {}
