#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ViewTransition",
        typescript_type = "ViewTransition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ViewTransition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ViewTransition;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ViewTransition",
        js_name = "updateCallbackDone"
    )]
    #[doc = "Getter for the `updateCallbackDone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn update_callback_done(this: &ViewTransition) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ViewTransition", js_name = "ready")]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ready(this: &ViewTransition) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ViewTransition", js_name = "finished")]
    #[doc = "Getter for the `finished` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finished(this: &ViewTransition) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ViewTransitionTypeSet")]
    #[wasm_bindgen(method, getter, js_class = "ViewTransition", js_name = "types")]
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`, `ViewTransitionTypeSet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn types(this: &ViewTransition) -> ViewTransitionTypeSet;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ViewTransition",
        js_name = "transitionRoot"
    )]
    #[doc = "Getter for the `transitionRoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/transitionRoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transition_root(this: &ViewTransition) -> Element;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "ViewTransition", js_name = "skipTransition")]
    #[doc = "The `skipTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn skip_transition(this: &ViewTransition) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "ViewTransition", js_name = "waitUntil")]
    #[doc = "The `waitUntil()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/waitUntil)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn wait_until(this: &ViewTransition, promise: &::js_sys::Promise);
}
