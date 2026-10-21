//! Helpers shared by the user-spawned worker test binaries.

use js_sys::{Array, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, ErrorEvent, MessageEvent, Url};

/// Creates a blob URL for the JavaScript in `script`.
pub fn blob_url(script: &str) -> String {
    let parts = Array::new();
    parts.push(&JsValue::from_str(script));
    let opts = BlobPropertyBag::new();
    opts.set_type("application/javascript");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &opts).unwrap();
    Url::create_object_url_with_blob(&blob).unwrap()
}

/// Returns the data of the first message passed to the message handler that
/// `install` registers. Panics if the error handler it is given fires first,
/// or if no message arrives within 500ms.
pub async fn first_message(what: &str, install: impl FnOnce(&Function, &Function)) -> JsValue {
    let mut install = Some(install);
    let promise = Promise::new(&mut |resolve, reject| {
        let onmessage = Closure::once_into_js(move |e: MessageEvent| {
            resolve.call1(&JsValue::NULL, &e.data()).unwrap();
        });
        let reject_error = reject.clone();
        let onerror = Closure::once_into_js(move |e: ErrorEvent| {
            let message = format!("worker error: {}", e.message());
            reject_error
                .call1(&JsValue::NULL, &JsValue::from_str(&message))
                .unwrap();
        });
        install.take().unwrap()(onmessage.unchecked_ref(), onerror.unchecked_ref());

        let message = format!("timed out waiting for {what}");
        let timeout = Closure::once_into_js(move || {
            reject
                .call1(&JsValue::NULL, &JsValue::from_str(&message))
                .unwrap();
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.unchecked_ref(), 500)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap()
}
