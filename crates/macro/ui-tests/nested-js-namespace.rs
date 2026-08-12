use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["a"])]
extern "C" {
    #[wasm_bindgen(js_namespace = ["b"])]
    fn my_function();

    #[wasm_bindgen(js_namespace = ["b"])]
    type MyType;

    #[wasm_bindgen(js_namespace = ["b"], thread_local_v2)]
    static MY_STATIC: JsValue;
}

fn main() {}
