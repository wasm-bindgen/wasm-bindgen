// FLAGS: --target=bundler
// FLAGS: --target=bundler --debug

// Reference test for a `()` argument, which is `undefined`. It takes its
// place in the JS signature and no Wasm parameter: an export or a closure
// checks it in a debug build, and an import is passed `undefined` where it
// stands. A returned `()` or `&()` is nothing.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn flat(a: u32, unit: (), names: Vec<String>, last: ()) -> u32 {
    let ((), ()) = (unit, last);
    a + names.len() as u32
}

#[wasm_bindgen]
pub struct Record {
    pub unit: (),
    pub a: u32,
}

#[wasm_bindgen]
impl Record {
    #[wasm_bindgen(setter)]
    pub fn set_other(&mut self, unit: ()) {
        let () = unit;
    }
}

#[wasm_bindgen]
extern "C" {
    fn js_flat(a: u32, unit: (), b: &str, last: &());
    fn js_call(f: &dyn Fn(u32, (), u32) -> &'static ());

    type Thing;
    #[wasm_bindgen(constructor)]
    fn new(unit: (), a: u32) -> Thing;
    #[wasm_bindgen(method)]
    fn method(this: &Thing, unit: ());
    #[wasm_bindgen(method, setter)]
    fn set_unit(this: &Thing, unit: ());
    #[wasm_bindgen(variadic)]
    fn js_rest(unit: (), rest: Vec<u32>);
}

#[wasm_bindgen]
pub fn driver() {
    js_flat(1, (), "b", &());
    js_call(&|_, (), _| &());
    let thing = Thing::new((), 1);
    thing.method(());
    thing.set_unit(());
    js_rest((), vec![1]);
}
