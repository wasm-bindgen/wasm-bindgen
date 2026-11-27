// FLAGS: --debug

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Item;

#[wasm_bindgen]
impl Item {
    pub fn eat(self, other: Item, third: Option<Item>, fourth: &Item) {
        drop((other, third, fourth));
    }
}
