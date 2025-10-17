#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ReadableStream , extends = :: js_sys :: Object , js_name = WebTransportReceiveStream , typescript_type = "WebTransportReceiveStream")]
    #[derive(Debug, Clone, PartialEq, Eq, :: wasm_bindgen :: Upcast)]
    #[doc = "The `WebTransportReceiveStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportReceiveStream;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "WebTransportReceiveStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_stats(this: &WebTransportReceiveStream) -> ::js_sys::Promise;
}
#[cfg(web_sys_unstable_apis)]
#[automatically_derived]
impl ::wasm_bindgen::convert::Upcast<ReadableStream> for WebTransportReceiveStream {}
