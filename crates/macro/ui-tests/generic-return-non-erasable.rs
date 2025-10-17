use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Test: Generic import function with T return type
    // This should fail when called expecting non-JsValue-erasable types like u32
    pub fn get_value<T>() -> T;

    // Test: Generic import function with Option<T> return type
    // This should fail when called expecting non-JsValue-erasable types like u32
    pub fn get_optional<T>() -> Option<T>;
}

// Attempt to call these functions expecting non-JsValue-erasable return types
fn test_return_generic() {
    let _: u32 = get_value::<u32>();
}

fn test_return_optional_generic() {
    let _: Option<u32> = get_optional::<u32>();
}

fn main() {}
