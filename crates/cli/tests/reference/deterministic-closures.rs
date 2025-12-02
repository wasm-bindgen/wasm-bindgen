// DEPENDENCY: js-sys = { path = '{root}/crates/js-sys' }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn closure_fn() -> js_sys::Function {
    Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>)
        .into_js_value()
        .unchecked_into()
}

#[wasm_bindgen]
pub fn closure_fn_mut_i32() -> js_sys::Function {
    let mut x = 0;
    Closure::wrap(Box::new(move |y: i32| {
        x += y;
    }) as Box<dyn FnMut(i32)>)
        .into_js_value()
        .unchecked_into()
}

#[wasm_bindgen]
pub fn closure_fn_string() -> js_sys::Function {
    Closure::wrap(Box::new(|_s: String| {}) as Box<dyn Fn(String)>)
        .into_js_value()
        .unchecked_into()
}
