use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generics.js")]
extern "C" {
    fn test_generic_ref<T>(val: &T, ty: &str);
    fn get_test_val<T>(ty: &str) -> T;
}

#[wasm_bindgen_test]
fn test_numeric_value_slots() {
    test_generic_ref(&JsValue::from(42i32), "number");
    test_generic_ref(&JsValue::from(123u32), "number");
    test_generic_ref(&JsValue::from(3.14f32), "number");
    test_generic_ref(&JsValue::from(2.718f64), "number");
}

#[wasm_bindgen_test]
fn test_bigint_value_slots() {
    test_generic_ref(&JsValue::from(9223372036854775807i64), "bigint");
    test_generic_ref(&JsValue::from(18446744073709551615u64), "bigint");
}

#[wasm_bindgen_test]
fn test_string_value_slots() {
    test_generic_ref(&JsValue::from("🦀"), "string");
}

#[wasm_bindgen_test]
fn test_bool_value_slots() {
    test_generic_ref(&JsValue::from(true), "boolean");
}

#[wasm_bindgen_test]
fn test_js_value_cast() {
    let js_num: i32 = get_test_val("number");
    assert_eq!(js_num, 42);

    let js_str: String = get_test_val("string");
    assert_eq!(js_str, "test");

    let js_bigint: i64 = get_test_val("bigint");
    assert_eq!(js_bigint, 9007199254740991i64);

    let js_bool: bool = get_test_val("boolean");
    assert_eq!(js_bool, true);
}

#[wasm_bindgen_test]
fn test_multiple_slots_and_cleanup() {
    let heap_before = wasm_bindgen::externref_heap_live_count();

    let vals = [1i32, 2i32, 3i32, 4i32, 5i32];
    for val in &vals {
        test_generic_ref(&JsValue::from(*val), "number");
    }

    let heap_after = wasm_bindgen::externref_heap_live_count();
    assert_eq!(heap_before, heap_after, "Value slots should be cleaned up");
}
