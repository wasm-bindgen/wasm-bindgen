use wasm_bindgen::prelude::*;
use js_sys::Function;

fn test_function_call_with_i32() {
    // Create a Function type with i32 argument (uninhabitable type)
    let func: Function<JsValue, i32> = unsafe { std::mem::zeroed() };

    // Try to call it - should fail because i32 doesn't implement ErasableGeneric
    let _result = func.call1(&JsValue::undefined(), &42);
}

fn main() {}
