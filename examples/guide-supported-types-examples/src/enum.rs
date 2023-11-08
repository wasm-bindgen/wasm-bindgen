use wasm_bindgen::prelude::*;

#[wasm_bindgen]
enum NumberEnum {
    Foo,
    Bar,
    Baz,
}

#[wasm_bindgen]
enum StringEnum {
    Spam = "spam",
    Eggs = "eggs",
}
