use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/value_slots.js")]
extern "C" {
    // Test functions that take generic references using value slots
    fn test_generic_i32_ref(val: &i32);
    fn test_generic_u32_ref(val: &u32);
    fn test_generic_f32_ref(val: &f32);
    fn test_generic_f64_ref(val: &f64);
    fn test_generic_i64_ref(val: &i64);
    fn test_generic_u64_ref(val: &u64);
    fn test_generic_char_ref(val: &char);
    fn test_generic_bool_ref(val: &bool);
    
    // Test functions that return generics using JsValueCast
    fn get_test_number() -> JsValue;
    fn get_test_string() -> JsValue;
    fn get_test_bigint() -> JsValue;
    fn get_test_boolean() -> JsValue;
}

#[wasm_bindgen_test]
fn test_numeric_value_slots() {
    // Test that numeric references properly use value slots
    let i32_val = 42i32;
    test_generic_i32_ref(&i32_val);
    
    let u32_val = 123u32;
    test_generic_u32_ref(&u32_val);
    
    let f32_val = 3.14f32;
    test_generic_f32_ref(&f32_val);
    
    let f64_val = 2.718f64;
    test_generic_f64_ref(&f64_val);
}

#[wasm_bindgen_test]
fn test_bigint_value_slots() {
    // Test that bigint references properly use value slots
    let i64_val = 9223372036854775807i64;
    test_generic_i64_ref(&i64_val);
    
    let u64_val = 18446744073709551615u64;
    test_generic_u64_ref(&u64_val);
}

#[wasm_bindgen_test]
fn test_string_value_slots() {
    // Test that char references properly use value slots
    let char_val = '🦀';
    test_generic_char_ref(&char_val);
}

#[wasm_bindgen_test]
fn test_bool_value_slots() {
    // Test that bool references use constants (not value slots)
    let bool_val = true;
    test_generic_bool_ref(&bool_val);
}

#[wasm_bindgen_test]
fn test_js_value_cast() {
    // Test JsValueCast implementations for return values
    use wasm_bindgen::cast::JsValueCast;
    
    let js_num = get_test_number();
    let num: i32 = JsValueCast::unchecked_from_js_value(js_num);
    assert_eq!(num, 42);
    
    let js_str = get_test_string();
    let s: String = js_str.as_string().expect("should be string");
    assert_eq!(s, "test");
    
    let js_bigint = get_test_bigint();
    let bigint_val: i64 = JsValueCast::unchecked_from_js_value(js_bigint);
    assert_eq!(bigint_val, 9007199254740991i64);
    
    let js_bool = get_test_boolean();
    let bool_val: bool = JsValueCast::unchecked_from_js_value(js_bool);
    assert_eq!(bool_val, true);
}

#[wasm_bindgen_test]
fn test_multiple_slots_and_cleanup() {
    // Test that multiple value slots work and get cleaned up properly
    let heap_before = wasm_bindgen::externref_heap_live_count();
    
    // Use multiple value slots simultaneously
    let vals = [1i32, 2i32, 3i32, 4i32, 5i32];
    for val in &vals {
        test_generic_i32_ref(val);
    }
    
    // After the function calls, slots should be cleaned up
    // and heap count should return to initial state
    let heap_after = wasm_bindgen::externref_heap_live_count();
    assert_eq!(heap_before, heap_after, "Value slots should be cleaned up");
}