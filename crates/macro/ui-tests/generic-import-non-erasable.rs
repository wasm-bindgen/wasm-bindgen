use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Test: Generic import function with T parameter
    // This should fail when called with non-JsValue-erasable types like u32
    pub fn foo<T>(a: T) -> bool;

    // Test: Generic import function with &T parameter
    // This should fail when called with non-JsValue-erasable types like u32
    pub fn bar<T>(a: &T) -> bool;

    // Test: Generic import function with &mut T parameter
    // This should fail when called with non-JsValue-erasable types like u32
    pub fn baz<T>(a: &mut T) -> bool;
}

// Attempt to call these functions with non-JsValue-erasable types
fn test_direct_generic() {
    foo::<u32>(42);
}

fn test_ref_generic() {
    bar::<u32>(&42);
}

fn test_mut_ref_generic() {
    let mut x = 42u32;
    baz::<u32>(&mut x);
}

fn main() {}
