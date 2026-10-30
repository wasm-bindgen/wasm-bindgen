// A by-value generic argument that owns its value crosses borrowed and is
// dropped in Rust after the call, so `String` and `&str` share one binding,
// as do `JsString` and `&JsString`. Monomorphisations that differ only in a
// handle's TypeScript name share one binding too.

use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsStringLike;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string")]
    type Text;

    #[wasm_bindgen(typescript_type = "Thing")]
    type Thing;

    #[wasm_bindgen(experimental_generic_mono, js_name = take)]
    fn take(value: impl JsStringLike);

    #[wasm_bindgen(experimental_generic_mono, js_name = takeOptional)]
    fn take_optional(value: Option<impl JsStringLike>);

    #[wasm_bindgen(experimental_generic_mono, js_name = takeHandle)]
    fn take_handle<T: IntoWasmAbi>(value: T);
}

#[wasm_bindgen]
pub fn strings(s: String) {
    take("a");
    take(&s);
    take(s.clone());
    take_optional(Some("a"));
    take_optional(Some(s));
    take_optional(None::<&str>);
}

#[wasm_bindgen]
pub fn handles(text: Text, thing: Thing, value: JsValue) {
    take_handle(&value);
    take_handle(value);
    take_handle(text);
    take_handle(thing);
}
