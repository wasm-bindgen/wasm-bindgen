//! Bindings to the JS API.

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// Type for the [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
    pub(super) type Performance;

    /// Holds the [`Performance`](https://developer.mozilla.org/en-US/docs/Web/API/Performance) object.
    #[wasm_bindgen(thread_local_v2, js_namespace = globalThis, js_name = performance)]
    pub(super) static PERFORMANCE: Option<Performance>;

    /// Binding to [`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now).
    #[wasm_bindgen(method)]
    pub(super) fn now(this: &Performance) -> f64;

    /// Holds the [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
    #[cfg(target_feature = "atomics")]
    #[wasm_bindgen(thread_local_v2, js_namespace = ["globalThis", "performance"], js_name = timeOrigin)]
    pub(super) static TIME_ORIGIN: f64;

    /// Type for the [`Date` object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date).
    pub(super) type Date;

    /// Binding to [`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now).
    #[wasm_bindgen(static_method_of = Date)]
    pub(super) fn now() -> f64;
}
