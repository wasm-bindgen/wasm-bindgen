use wasm_bindgen::prelude::*;

// Test: Exported struct with generics (should fail)
#[wasm_bindgen]
struct ExportedStructWithGenerics<T>(T);

// Test: Exported struct without generics (should pass)
#[wasm_bindgen]
struct ValidExportedStruct {
    pub field: u32,
}

#[wasm_bindgen]
extern "C" {
    // Test 1: Type parameter with bounds (should fail)
    pub type TypeWithBounds<T: Clone>;

    // Test 2: Type parameter with where clause (should fail)
    pub type TypeWithWhere<T> where T: Clone;

    // Test 3: Lifetime parameter on type (should fail)
    pub type TypeWithLifetime<'a, T>;

    // Test 4: Const parameter on type (should fail)
    pub type TypeWithConst<const N: usize>;

    // Test 5: Function with bounds (should fail)
    pub fn func_with_bounds<T: Clone>() -> T;

    // Test 6: Function with where clause (should fail)
    pub fn func_with_where<T>() -> T where T: Clone;

    // Test 7: Function with lifetime parameter (should fail)
    pub fn func_with_lifetime<'a, T>() -> T;

    // Test 8: Function with const parameter (should fail)
    pub fn func_with_const<const N: usize>() -> i32;

    // Test 9: Valid generic type (should pass)
    pub type ValidType<T>;

    // Test 10: Valid generic type with default (should pass)
    pub type ValidTypeWithDefault<T = JsValue>;

    // Test 11: Valid generic function (should pass)
    pub fn valid_func<T>() -> ValidType<T>;

    // Test 12: Valid generic constructor (should pass)
    #[wasm_bindgen(constructor)]
    pub fn new<T>() -> ValidType<T>;
}

fn main() {}