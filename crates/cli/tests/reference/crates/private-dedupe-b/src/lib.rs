use wasm_bindgen::prelude::*;

#[wasm_bindgen(private)]
pub struct Status {
    pub code: u32,
}

#[wasm_bindgen]
impl Status {
    pub fn describe(&self) -> String {
        format!("b:{}", self.code)
    }
}

pub fn make() -> Status {
    Status { code: 2 }
}
