#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSViewTransitionRule",
        typescript_type = "CSSViewTransitionRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssViewTransitionRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSViewTransitionRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssViewTransitionRule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type CssViewTransitionRule;
}
