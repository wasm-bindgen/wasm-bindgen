use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Item;

#[wasm_bindgen]
extern "C" {
    fn takes_values(v: Vec<JsValue>);
    fn takes_optional_values(v: Option<Vec<JsValue>>);
    fn takes_beside(item: Item, value: JsValue, names: Vec<String>);
    fn takes_nullable_beside(names: Vec<String>, item: Nullable<Item>);
}

#[wasm_bindgen]
pub fn give() {
    takes_values(vec![JsValue::NULL]);
    takes_optional_values(None);
    takes_beside(Item, JsValue::NULL, Vec::new());
    takes_nullable_beside(Vec::new(), Nullable::Null);
}
