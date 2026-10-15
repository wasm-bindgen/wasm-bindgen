#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsGeneric;

#[wasm_bindgen]
extern "C" {
    pub type Holder<T: JsGeneric>;

    #[wasm_bindgen(method)]
    pub fn get<T: JsGeneric>(this: &Holder<T>) -> T;

    #[wasm_bindgen(method, catch)]
    pub async fn load<T: JsGeneric>(this: &Holder<T>) -> Result<T, JsValue>;

    #[wasm_bindgen(method, catch, js_name = load)]
    pub async fn load_opt<T: JsGeneric>(this: &Holder<T>) -> Result<Option<T>, JsValue>;

    #[wasm_bindgen(method, js_name = load)]
    pub async fn load_uncaught<T: JsGeneric>(this: &Holder<T>) -> T;
}

async fn instantiate(holder: &Holder<js_sys::Object>) {
    let _: js_sys::Object = holder.load().await.unwrap();
    let _: Option<js_sys::Object> = holder.load_opt().await.unwrap();
    let _: js_sys::Object = holder.load_uncaught().await;
}

fn main() {}
