use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = SomeClass, js_name = Symbol.toPrimitive)]
    fn import_static_symbol(s: &str);

    #[wasm_bindgen(js_namespace = SomeClass, getter = Symbol.iterator)]
    fn import_static_symbol_getter();

    // We don't want to import JS strings as `String`, since Rust already has a
    // `String` type in its prelude, so rename it as `JsString`.
    #[wasm_bindgen(js_name = String)]
    type JsString;

    // This is a method on the JavaScript "String" class, so specify that with
    // the `js_class` attribute.
    #[wasm_bindgen(constructor)]
    fn new() -> JsString;

    // This is a method on the JavaScript "String" class, so specify that with
    // the `js_class` attribute.
    #[wasm_bindgen(method, js_class = "String", js_name = Symbol.iterator)]
    fn string_iterator(this: &JsString) -> u32;

    // This is a method on the JavaScript "String" class, so specify that with
    // the `js_class` attribute.
    #[wasm_bindgen(method, js_class = "String", getter = Symbol.toPrimitive)]
    fn string_getter(this: &JsString) -> String;
}

fn make_used() {
    import_static_symbol("");
    import_static_symbol_getter();
    let s = JsString::new();
    s.string_iterator();
    s.string_getter();
}

#[wasm_bindgen]
pub struct Foo;

#[wasm_bindgen]
impl Foo {
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive
    #[wasm_bindgen(js_name = Symbol.toPrimitive)]
    pub fn to_primitive(&self) -> String {
        make_used();
        "Why is it this string? I don't known.".to_string()
    }

    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag
    #[wasm_bindgen(getter = Symbol.toStringTag)]
    pub fn to_string_tag(&self) -> String {
        "RustFooClass".to_string()
    }
}
