// FLAGS: --target=bundler
// FLAGS: --target=no-modules
use wasm_bindgen::prelude::*;

// The JSPI runtime bridge used by `js_sys::futures::jspi` is emitted as
// wasm-bindgen intrinsics rather than an `inline_js` snippet, so that it works
// with every target — including `--target no-modules`, which cannot import
// from `./snippets/...`.  This test snapshots the generated JS for all of the
// `__wbindgen_jspi_*` intrinsics.
#[wasm_bindgen(raw_module = "__wbindgen_placeholder__")]
extern "C" {
    fn __wbindgen_jspi_set_pending(id: u32, promise: &JsValue);
    #[wasm_bindgen(suspending)]
    fn __wbindgen_jspi_suspend(id: u32);
    fn __wbindgen_jspi_is_rejected(id: u32) -> bool;
    fn __wbindgen_jspi_get_resolved(id: u32) -> JsValue;
    fn __wbindgen_jspi_cleanup(id: u32);
    fn __wbindgen_jspi_waker_create(id: u32) -> JsValue;
    fn __wbindgen_jspi_waker_wake(id: u32);
    fn __wbindgen_jspi_waker_cleanup(id: u32);
}

#[wasm_bindgen(jspi)]
pub fn drive(promise: &JsValue) -> JsValue {
    __wbindgen_jspi_set_pending(0, promise);
    __wbindgen_jspi_suspend(0);
    let rejected = __wbindgen_jspi_is_rejected(0);
    let resolved = __wbindgen_jspi_get_resolved(0);
    __wbindgen_jspi_cleanup(0);

    let waker = __wbindgen_jspi_waker_create(1);
    __wbindgen_jspi_waker_wake(1);
    __wbindgen_jspi_waker_cleanup(1);

    if rejected {
        waker
    } else {
        resolved
    }
}
