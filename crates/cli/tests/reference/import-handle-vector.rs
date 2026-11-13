use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Item;

#[wasm_bindgen]
extern "C" {
    fn takes_values(v: Vec<JsValue>);
    fn takes_optional_values(v: Option<Vec<JsValue>>);
    fn takes_beside(item: Item, value: JsValue, names: Vec<String>);
}

#[wasm_bindgen]
pub fn give() {
    takes_values(vec![JsValue::NULL]);
    takes_optional_values(None);
    takes_beside(Item, JsValue::NULL, Vec::new());
}
