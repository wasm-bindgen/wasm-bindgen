// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }
// DEPENDENCY: web-sys = { path = '{root}/crates/web-sys', features = ['console', 'Window'] }

use wasm_bindgen::prelude::*;
use web_sys::{console, window};

#[wasm_bindgen]
pub fn use_stack_callback(a: &js_sys::Array) {
    a.for_each(&mut |v, _, _| {
        console::log_1(&v);
    });
}

#[wasm_bindgen]
pub fn delayed_callback() -> Result<(), JsValue> {
    window().unwrap_throw().set_timeout_with_callback(
        Closure::once_into_js(|| {
            console::log_1(&"timeout fired".into());
        })
        .unchecked_ref(),
    )?;

    Ok(())
}
