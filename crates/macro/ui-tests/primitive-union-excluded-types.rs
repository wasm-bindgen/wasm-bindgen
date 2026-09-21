use js_sys::JsNumberOrStringLike;
use wasm_bindgen::JsValue;

fn assert_number_or_string<T: JsNumberOrStringLike>() {}

fn main() {
    assert_number_or_string::<char>();
    assert_number_or_string::<JsValue>();
}
