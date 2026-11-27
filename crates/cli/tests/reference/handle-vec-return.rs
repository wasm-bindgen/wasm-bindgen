// FLAGS: --target=web --reference-types

use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

// A type with hand-written conversions that crosses as a JS value, with
// nothing else in the module that drops one.
pub struct Thing(u32);

impl WasmDescribe for Thing {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for Thing {
    type Abi = u32;

    fn into_abi(self) -> u32 {
        JsValue::from(self.0).into_abi()
    }
}

#[wasm_bindgen]
pub fn things_out() -> Vec<Thing> {
    vec![Thing(1), Thing(2)]
}
