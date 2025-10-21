use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generics.js")]
extern "C" {
    #[wasm_bindgen(js_name = "test_generic")]
    fn test_generic_ref<T>(val: &JsRef<T>, ty: &str);
    #[wasm_bindgen(js_name = "test_generic")]
    fn test_generic_val<T>(val: JsRef<T>, ty: &str);
    fn get_test_val<T>(ty: &str) -> JsRef<T>;
}

#[wasm_bindgen_test]
fn test_numeric_value_slots() {
    test_generic_ref(&42i32.to_js(), "number");
    test_generic_ref(&123u32.to_js(), "number");
    test_generic_ref(&3.14f32.to_js(), "number");
    test_generic_ref(&2.718f64.to_js(), "number");

    test_generic_val(JsValue::from(42i32), "number");
    test_generic_val(JsValue::from(123u32), "number");
    test_generic_val(JsValue::from(3.14f32), "number");
    test_generic_val(JsValue::from(2.718f64), "number");
}

#[wasm_bindgen_test]
fn test_bigint_value_slots() {
    test_generic_ref(&9223372036854775807i64.to_js(), "bigint");
    test_generic_ref(&18446744073709551615u64.to_js(), "bigint");
}

#[wasm_bindgen_test]
fn test_string_value_slots() {
    test_generic_ref(&"🦀".to_js(), "string");
}

#[wasm_bindgen_test]
fn test_bool_value_slots() {
    test_generic_ref(&true.to_js(), "boolean");
}

#[wasm_bindgen_test]
fn test_js_value_cast() {
    let js_num: i32 = get_test_val("number").from_js();
    assert_eq!(js_num, 42);

    let js_str: String = get_test_val("string").from_js();
    assert_eq!(js_str, "test");

    let js_bigint: i64 = get_test_val("bigint").from_js();
    assert_eq!(js_bigint, 9007199254740991i64);

    let js_bool: bool = get_test_val("boolean").from_js();
    assert_eq!(js_bool, true);
}
