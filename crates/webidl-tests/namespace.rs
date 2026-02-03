use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn simple_namespace_test() {
    assert_eq!(math_test::add_one(1), 2);
    assert_eq!(math_test::pow(1.0, 100.0), 1.0);
    assert_eq!(math_test::pow(10.0, 2.0), 100.0);
}

#[wasm_bindgen_test]
fn namespace_attribute_test() {
    // Test that namespace attributes (readonly) are properly generated
    // Note: The pi() getter is accessed via the JsNamespaceMathTest type
    // due to how static_method_of works with namespace attributes
    let pi = math_test::JsNamespaceMathTest::pi();
    assert!((pi - std::f64::consts::PI).abs() < 0.0001);
}
