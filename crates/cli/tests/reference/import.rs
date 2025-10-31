// FLAGS: --target=bundler
// FLAGS: --target=web
// FLAGS: --target=nodejs
// FLAGS: --target=experimental-nodejs-module
// FLAGS: --target=deno
// FLAGS: --target=module
// FLAGS: --target=module --experimental-reset-state-function

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Both `catch_me` and `no_catch` should be defined in the JS and invoke
    // their respective JS function inside a JS shim function. This is
    // important, because these 2 function may not be defined when the WASM
    // module is instantiated.
    #[wasm_bindgen(catch)]
    fn catch_me() -> Result<(), JsValue>;
    fn no_catch();

    // Reload needs to be passed the right `this` parameter in JS.
    #[wasm_bindgen(js_namespace = ["window", "location"])]
    fn reload();
    #[wasm_bindgen(js_namespace = ["window", "document"])]
    fn write(s: &str);
}

// module import
#[wasm_bindgen(module = "tests/wasm/imports.js")]
extern "C" {
    fn bar_from_foo();
}

#[wasm_bindgen(raw_module = "foo-raw")]
extern "C" {
    fn bar_from_foo_raw();
}

#[wasm_bindgen(inline_js = "export function add(a,b) { return a + b; }")]
extern "C" {
    fn add(a: f64, b: f64) -> f64;
}

#[link(wasm_import_module = "pure-extern")]
extern "C" {
    fn extern_fn();
}

#[wasm_bindgen(js_namespace = ["a"])]
extern "C" {
    // test that namespaces are overwritten and not inherited/concatenated
    #[wasm_bindgen(js_namespace = ["b"])]
    fn my_function();
    #[wasm_bindgen(thread_local_v2)]
    static CONST: f64;
}

#[wasm_bindgen(module = "tests/wasm/import_class.js")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    type RenamedTypes;
    #[wasm_bindgen(constructor, js_class = default)]
    fn new(arg: i32) -> RenamedTypes;
    #[wasm_bindgen(method, js_class = default)]
    fn get(this: &RenamedTypes) -> i32;
}

#[wasm_bindgen]
pub fn exported() -> Result<(), JsValue> {
    bar_from_foo();
    bar_from_foo_raw();
    unsafe {
        extern_fn();
    }
    let _ = add(CONST.with(Clone::clone), 2.0);
    reload();
    write("");
    no_catch();
    my_function();

    let f = RenamedTypes::new(1);
    assert_eq!(f.get(), 2);

    catch_me()
}
