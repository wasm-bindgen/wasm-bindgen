use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type BaseType1;
    type BaseType2;
}

// Test 1: This should fail with: "exported structs can only extend one type"
#[wasm_bindgen(extends = BaseType1, extends = BaseType2)]
pub struct MultipleExtends {
    value: i32,
}

// Test 2: This is an exported type, not imported
#[wasm_bindgen]
pub struct ExportedType {
    value: i32,
}

// Test 3: This should fail because ExportedType is not imported
#[wasm_bindgen(extends = ExportedType)]
pub struct TryingToExtendExported {
    data: String,
}

fn main() {}