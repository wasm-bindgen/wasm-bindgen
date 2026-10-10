#![deny(deprecated)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Foo;

    #[wasm_bindgen(method, getter, setter)]
    fn set_getter_setter(this: &Foo, value: u32);

    // The order of the options doesn't matter, `setter` still wins.
    #[wasm_bindgen(method, setter, getter)]
    fn set_setter_getter(this: &Foo, value: u32);

    #[wasm_bindgen(method, getter)]
    #[wasm_bindgen(setter)]
    fn set_split_attributes(this: &Foo, value: u32);

    #[wasm_bindgen(method, setter, js_name = renamed, getter)]
    fn set_with_js_name(this: &Foo, value: u32);

    #[wasm_bindgen(static_method_of = Foo, getter, setter)]
    fn set_static_getter_setter(value: u32);

    #[wasm_bindgen(method, getter, indexing_getter)]
    fn getter_indexing_getter(this: &Foo, key: u32) -> u32;

    #[wasm_bindgen(method, structural, indexing_getter, indexing_setter)]
    fn indexing_getter_setter(this: &Foo, key: u32, value: u32);

    #[wasm_bindgen(method, structural, indexing_getter, indexing_deleter)]
    fn indexing_getter_deleter(this: &Foo, key: u32);

    #[wasm_bindgen(method, structural, indexing_setter, indexing_deleter)]
    fn indexing_setter_deleter(this: &Foo, key: u32);

    #[wasm_bindgen(method, structural, getter, setter, indexing_deleter)]
    fn three_options(this: &Foo, key: u32);

    // A repeated option isn't a conflict. Only its first use is read.
    #[wasm_bindgen(method, getter, getter = other)]
    fn repeated_getter(this: &Foo) -> u32;

    // A single option is fine.
    #[wasm_bindgen(method, getter)]
    fn value(this: &Foo) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_value(this: &Foo, value: u32);
    #[wasm_bindgen(static_method_of = Foo, getter = staticValue)]
    fn static_value() -> u32;
    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Foo, key: u32) -> u32;
    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set(this: &Foo, key: u32, value: u32);
    #[wasm_bindgen(method, structural, indexing_deleter)]
    fn delete(this: &Foo, key: u32);
}

#[wasm_bindgen]
pub struct Bar {
    value: u32,
}

#[wasm_bindgen]
impl Bar {
    #[wasm_bindgen(getter, setter)]
    pub fn set_getter_setter(&mut self, value: u32) {
        self.value = value;
    }

    #[wasm_bindgen(getter, setter, js_name = renamed)]
    pub fn set_with_js_name(&mut self, value: u32) {
        self.value = value;
    }

    #[wasm_bindgen(getter, setter)]
    pub fn set_static_getter_setter(_value: u32) {}

    #[wasm_bindgen(indexing_getter, indexing_deleter)]
    pub fn indexing_getter_deleter(&self, _key: u32) {}

    // A single option is fine.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.value
    }
    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}

#[wasm_bindgen(this, getter)]
pub fn this_getter(_this: JsValue) {}

// A single option is fine.
#[wasm_bindgen(this)]
pub fn this_only(_this: JsValue) {}

fn main() {}
