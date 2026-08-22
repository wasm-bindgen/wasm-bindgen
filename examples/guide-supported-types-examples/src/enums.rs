use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum NumberEnum {
    // numbers are optional; default ones will be generated if not provided
    Foo = 0,
    Bar = 1,
    Baz = 2,
}

#[wasm_bindgen]
pub enum StringEnum {
    Spam = "spam",
    Eggs = "eggs",
}

#[wasm_bindgen]
pub fn take_enum_number(x: NumberEnum) {}

#[wasm_bindgen]
pub fn take_enum_string(x: StringEnum) {}

#[wasm_bindgen]
pub fn take_enum_number_option(x: Option<NumberEnum>) {}

#[wasm_bindgen]
pub fn take_enum_string_option(x: Option<StringEnum>) {}

#[wasm_bindgen]
pub fn return_enum_number() -> NumberEnum {
    todo!()
}

#[wasm_bindgen]
pub fn return_enum_string() -> StringEnum {
    todo!()
}

#[wasm_bindgen]
pub fn return_enum_number_option() -> Option<NumberEnum> {
    todo!()
}

#[wasm_bindgen]
pub fn return_enum_string_option() -> Option<StringEnum> {
    todo!()
}
