use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/try_from_js_value.js")]
extern "C" {
    type TryFromJsValueCustomType;
    #[wasm_bindgen(constructor)]
    fn new() -> TryFromJsValueCustomType;
    #[wasm_bindgen(method)]
    fn get_value(this: &TryFromJsValueCustomType) -> i32;

    fn make_custom_type() -> JsValue;
    fn make_plain_object() -> JsValue;
}

#[wasm_bindgen_test]
fn f64_try_from_js_value() {
    assert_eq!(f64::try_from_js_value(JsValue::from_f64(42.5)), Ok(42.5));
    assert_eq!(f64::try_from_js_value(JsValue::from_f64(0.0)), Ok(0.0));
    assert_eq!(
        f64::try_from_js_value(JsValue::from_f64(-123.456)),
        Ok(-123.456)
    );

    assert_eq!(f64::try_from_js_value(JsValue::from_str("42.5")), Ok(42.5));

    assert!(f64::try_from_js_value(JsValue::symbol(None)).is_err());
}

#[wasm_bindgen_test]
fn string_try_from_js_value() {
    assert_eq!(
        String::try_from_js_value(JsValue::from_str("hello")),
        Ok("hello".to_string())
    );
    assert_eq!(
        String::try_from_js_value(JsValue::from_str("")),
        Ok("".to_string())
    );

    assert!(String::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(String::try_from_js_value(JsValue::NULL).is_err());
}

#[wasm_bindgen_test]
fn i64_try_from_js_value() {
    assert_eq!(i64::try_from_js_value(JsValue::from(42_i64)), Ok(42_i64));
    assert_eq!(i64::try_from_js_value(JsValue::from(-42_i64)), Ok(-42_i64));
    assert_eq!(
        i64::try_from_js_value(JsValue::from(i64::MIN)),
        Ok(i64::MIN)
    );
    assert_eq!(
        i64::try_from_js_value(JsValue::from(i64::MAX)),
        Ok(i64::MAX)
    );

    assert!(i64::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(i64::try_from_js_value(JsValue::NULL).is_err());

    assert!(i64::try_from_js_value(JsValue::from(i64::MIN) - JsValue::from(1_i64)).is_err());
    assert!(i64::try_from_js_value(JsValue::from(i64::MAX) + JsValue::from(1_i64)).is_err());
}

#[wasm_bindgen_test]
fn u64_try_from_js_value() {
    assert_eq!(u64::try_from_js_value(JsValue::from(42_u64)), Ok(42_u64));
    assert_eq!(u64::try_from_js_value(JsValue::from(0_u64)), Ok(0_u64));
    assert_eq!(
        u64::try_from_js_value(JsValue::from(u64::MAX)),
        Ok(u64::MAX)
    );

    assert!(u64::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(u64::try_from_js_value(JsValue::NULL).is_err());

    assert!(u64::try_from_js_value(JsValue::from(-1_i64)).is_err());
    assert!(u64::try_from_js_value(JsValue::from(u64::MAX) + JsValue::from(1_i64)).is_err());
}

#[wasm_bindgen_test]
fn i128_try_from_js_value() {
    assert_eq!(i128::try_from_js_value(JsValue::from(42_i128)), Ok(42_i128));
    assert_eq!(
        i128::try_from_js_value(JsValue::from(-42_i128)),
        Ok(-42_i128)
    );
    assert_eq!(
        i128::try_from_js_value(JsValue::from(i128::MIN)),
        Ok(i128::MIN)
    );
    assert_eq!(
        i128::try_from_js_value(JsValue::from(i128::MAX)),
        Ok(i128::MAX)
    );

    assert!(i128::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(i128::try_from_js_value(JsValue::NULL).is_err());

    let below_min = JsValue::from(i128::MIN) - JsValue::from(1_i64);
    assert!(i128::try_from_js_value(below_min).is_err());

    let above_max = JsValue::from(i128::MAX) + JsValue::from(1_i64);
    assert!(i128::try_from_js_value(above_max).is_err());
}

#[wasm_bindgen_test]
fn u128_try_from_js_value() {
    assert_eq!(u128::try_from_js_value(JsValue::from(42_u128)), Ok(42_u128));
    assert_eq!(u128::try_from_js_value(JsValue::from(0_u128)), Ok(0_u128));
    assert_eq!(
        u128::try_from_js_value(JsValue::from(u128::MAX)),
        Ok(u128::MAX)
    );

    assert!(u128::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(u128::try_from_js_value(JsValue::NULL).is_err());

    assert!(u128::try_from_js_value(JsValue::from(-1_i64)).is_err());

    let above_max = JsValue::from(u128::MAX) + JsValue::from(1_i64);
    assert!(u128::try_from_js_value(above_max).is_err());
}

#[wasm_bindgen_test]
fn unit_try_from_js_value() {
    assert_eq!(<()>::try_from_js_value(JsValue::UNDEFINED), Ok(()));

    assert!(<()>::try_from_js_value(JsValue::NULL).is_err());
    assert!(<()>::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(<()>::try_from_js_value(JsValue::from_str("hello")).is_err());
}

#[wasm_bindgen_test]
fn option_try_from_js_value() {
    assert_eq!(
        Option::<i64>::try_from_js_value(JsValue::UNDEFINED),
        Ok(None)
    );
    assert_eq!(Option::<i64>::try_from_js_value(JsValue::NULL), Ok(None));
    assert_eq!(
        Option::<String>::try_from_js_value(JsValue::UNDEFINED),
        Ok(None)
    );
    assert_eq!(Option::<String>::try_from_js_value(JsValue::NULL), Ok(None));

    assert_eq!(
        Option::<i64>::try_from_js_value(JsValue::from(42_i64)),
        Ok(Some(42_i64))
    );
    assert_eq!(
        Option::<String>::try_from_js_value(JsValue::from_str("hello")),
        Ok(Some("hello".to_string()))
    );
    assert_eq!(
        Option::<f64>::try_from_js_value(JsValue::from_f64(3.14)),
        Ok(Some(3.14))
    );

    assert!(Option::<i64>::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(Option::<String>::try_from_js_value(JsValue::from_f64(42.0)).is_err());
}

#[wasm_bindgen_test]
fn imported_type_try_from_js_value() {
    let custom = make_custom_type();
    let result = TryFromJsValueCustomType::try_from_js_value(custom);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_value(), 42);

    let plain_obj = make_plain_object();
    assert!(TryFromJsValueCustomType::try_from_js_value(plain_obj).is_err());

    assert!(TryFromJsValueCustomType::try_from_js_value(JsValue::from_f64(42.0)).is_err());
    assert!(TryFromJsValueCustomType::try_from_js_value(JsValue::NULL).is_err());
}
