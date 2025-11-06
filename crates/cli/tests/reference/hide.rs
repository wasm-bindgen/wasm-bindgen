use wasm_bindgen::prelude::*;

/// A hidden struct that is not exported but can be used as an argument type
#[wasm_bindgen(private)]
pub struct HiddenStruct {
    pub value: i32,
}

/// A public struct that is exported
#[wasm_bindgen]
pub struct PublicStruct {
    pub value: i32,
}

/// A hidden enum that is not exported
#[wasm_bindgen(private)]
pub enum HiddenEnum {
    Variant1,
    Variant2,
}

/// A public enum that is exported
#[wasm_bindgen]
pub enum PublicEnum {
    A,
    B,
}

/// Function that takes a hidden struct as an argument
#[wasm_bindgen]
pub fn use_hidden_struct(hidden: HiddenStruct) -> i32 {
    hidden.value
}

/// Function that takes a hidden enum as an argument
#[wasm_bindgen]
pub fn use_hidden_enum(hidden: HiddenEnum) -> i32 {
    match hidden {
        HiddenEnum::Variant1 => 1,
        HiddenEnum::Variant2 => 2,
    }
}

/// Function that returns a public struct
#[wasm_bindgen]
pub fn get_public_struct() -> PublicStruct {
    PublicStruct { value: 42 }
}

// Test with js_namespace
#[wasm_bindgen(private, js_namespace = internal)]
pub struct NamespacedHidden {
    pub data: i32,
}

#[wasm_bindgen(js_namespace = internal)]
pub fn create_namespaced() -> NamespacedHidden {
    NamespacedHidden { data: 100 }
}
