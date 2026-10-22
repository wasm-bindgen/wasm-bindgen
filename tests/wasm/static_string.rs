use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/static_string.js")]
extern "C" {
    fn js_concat(a: &JsString, b: &JsString) -> JsString;
    fn js_length(s: &JsValue) -> u32;
}

// Rustfmt removes the value on these statics, see rustfmt#6267.
#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static HELLO: JsString = "Hello, World!";
    #[wasm_bindgen(thread_local_v2, static_string)]
    static EMOJI: JsString = "😀 ünïcödé";
    #[wasm_bindgen(thread_local_v2, static_string)]
    static EMPTY: JsString = "";
    #[wasm_bindgen(thread_local_v2, static_string)]
    static SPECIAL: JsString = "`${quotes}` \"'\\\n\r\t\0\u{2028}";
    #[wasm_bindgen(thread_local_v2, static_string)]
    static HELLO_AGAIN: JsString = "Hello, World!";
}

mod nested {
    use super::*;

    // The same static as the parent module's, sharing its shim.
    #[wasm_bindgen]
    #[rustfmt::skip]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, static_string)]
        pub(super) static HELLO: JsString = "Hello, World!";
    }
}

#[wasm_bindgen_test]
fn values() {
    HELLO.with(|s| assert_eq!(*s, "Hello, World!"));
    EMOJI.with(|s| assert_eq!(*s, "😀 ünïcödé"));
    EMPTY.with(|s| assert_eq!(*s, ""));
    SPECIAL.with(|s| assert_eq!(*s, "`${quotes}` \"'\\\n\r\t\0\u{2028}"));
    HELLO_AGAIN.with(|s| assert_eq!(*s, "Hello, World!"));
    nested::HELLO.with(|s| assert_eq!(*s, "Hello, World!"));
    HELLO.with(|s| assert_eq!(String::from(s), "Hello, World!"));
}

#[wasm_bindgen_test]
fn passed_to_js() {
    HELLO.with(|a| EMOJI.with(|b| assert_eq!(js_concat(a, b), "Hello, World!😀 ünïcödé")));
    SPECIAL.with(|s| assert_eq!(js_length(s), 20));
    EMPTY.with(|s| assert_eq!(js_length(s), 0));
}

// With an `externref` table the strings sit at fixed slots, so handing them
// around never touches the heap.
#[cfg(target_feature = "reference-types")]
#[wasm_bindgen_test]
fn statics_dont_allocate() {
    let before = wasm_bindgen::externref_heap_live_count();
    let owned: JsString = HELLO.with(|s| s.clone());
    let value = JsValue::from(owned.clone());
    let again = value.clone();
    assert_eq!(wasm_bindgen::externref_heap_live_count(), before);
    drop(again);
    drop(value);
    drop(owned);
    EMOJI.with(|s| assert_eq!(js_concat(s, s), "😀 ünïcödé😀 ünïcödé"));
    assert_eq!(wasm_bindgen::externref_heap_live_count(), before);
}
