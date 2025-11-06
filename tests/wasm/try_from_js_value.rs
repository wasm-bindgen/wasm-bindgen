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
    assert!(matches!(
        f64::try_from_js_value(JsValue::from_str("42.5")),
        Err(_)
    ));
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
    assert!(i64::try_from_js_value(JsValue::NULL).is_err());
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
    assert!(u64::try_from_js_value(JsValue::TRUE).is_err());
    assert!(u64::try_from_js_value(JsValue::from_str("45")).is_err());
    assert!(u64::try_from_js_value(JsValue::NULL).is_err());
    assert!(u64::try_from_js_value(JsValue::from(-1_i64)).is_err());
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
    assert_eq!(
        Option::<i64>::try_from_js_value(JsValue::NULL),
        Err(JsValue::NULL)
    );
    assert_eq!(
        Option::<String>::try_from_js_value(JsValue::UNDEFINED),
        Ok(None)
    );
    assert_eq!(
        Option::<String>::try_from_js_value(JsValue::NULL),
        Err(JsValue::NULL)
    );

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

    assert_eq!(
        Option::<i64>::try_from_js_value(JsValue::bigint_from_str("42")),
        Ok(Some(42))
    );
    assert!(Option::<String>::try_from_js_value(JsValue::from_f64(42.0)).is_err());

    assert_eq!(
        Option::<JsValue>::try_from_js_value(JsValue::NULL),
        Ok(Some(JsValue::NULL))
    );
    assert_eq!(
        Option::<JsValue>::try_from_js_value(JsValue::from_str("test")),
        Ok(Some(JsValue::from_str("test")))
    );
}

#[wasm_bindgen_test]
fn try_from_js_value_ref() {
    let val = JsValue::from(42_i64);
    assert_eq!(i64::try_from_js_value_ref(&val), Some(42_i64));
    assert_eq!(
        String::try_from_js_value_ref(&JsValue::from_str("hello")),
        Some("hello".to_string())
    );
    assert_eq!(bool::try_from_js_value_ref(&JsValue::TRUE), Some(true));
}

#[wasm_bindgen_test]
fn bool_try_from_js_value() {
    assert_eq!(bool::try_from_js_value(JsValue::TRUE), Ok(true));
    assert_eq!(bool::try_from_js_value(JsValue::FALSE), Ok(false));

    assert!(bool::try_from_js_value(JsValue::from_f64(1.0)).is_err());
    assert!(bool::try_from_js_value(JsValue::from_str("true")).is_err());
    assert!(bool::try_from_js_value(JsValue::NULL).is_err());
}

#[wasm_bindgen_test]
fn char_try_from_js_value() {
    assert_eq!(char::try_from_js_value(JsValue::from_str("a")), Ok('a'));
    assert!(char::try_from_js_value(JsValue::from_str("")).is_err());
    assert!(char::try_from_js_value(JsValue::from_str("ab")).is_err());
    assert!(char::try_from_js_value(JsValue::from_f64(65.0)).is_err());
    assert!(char::try_from_js_value(JsValue::NULL).is_err());
}

#[wasm_bindgen_test]
fn small_numbers_try_from_js_value() {
    assert_eq!(i8::try_from_js_value(JsValue::from_f64(42.0)), Ok(42i8));
    assert_eq!(i8::try_from_js_value(JsValue::from_f64(-128.0)), Ok(-128i8));
    assert_eq!(i8::try_from_js_value(JsValue::from_f64(127.0)), Ok(i8::MAX));
    assert_eq!(i8::try_from_js_value(JsValue::from_f64(128.0)), Ok(-128i8));
    assert_eq!(i8::try_from_js_value(JsValue::from_f64(42.5)), Ok(42i8));

    assert_eq!(u8::try_from_js_value(JsValue::from_f64(42.0)), Ok(42u8));
    assert_eq!(u8::try_from_js_value(JsValue::from_f64(0.0)), Ok(0u8));
    assert_eq!(u8::try_from_js_value(JsValue::from_f64(255.0)), Ok(u8::MAX));
    assert_eq!(u8::try_from_js_value(JsValue::from_f64(256.0)), Ok(0));
    assert_eq!(u8::try_from_js_value(JsValue::from_f64(-1.0)), Ok(255));

    assert_eq!(
        i16::try_from_js_value(JsValue::from_f64(1000.0)),
        Ok(1000i16)
    );
    assert_eq!(
        i16::try_from_js_value(JsValue::from_f64(i16::MIN as f64)),
        Ok(i16::MIN)
    );
    assert_eq!(
        i16::try_from_js_value(JsValue::from_f64(i16::MAX as f64)),
        Ok(i16::MAX)
    );

    assert_eq!(
        u16::try_from_js_value(JsValue::from_f64(1000.0)),
        Ok(1000u16)
    );
    assert_eq!(u16::try_from_js_value(JsValue::from_f64(0.0)), Ok(0u16));
    assert_eq!(
        u16::try_from_js_value(JsValue::from_f64(u16::MAX as f64)),
        Ok(u16::MAX)
    );

    assert_eq!(
        i32::try_from_js_value(JsValue::from_f64(100000.0)),
        Ok(100000i32)
    );
    assert_eq!(
        i32::try_from_js_value(JsValue::from_f64(-100000.0)),
        Ok(-100000i32)
    );

    assert_eq!(
        u32::try_from_js_value(JsValue::from_f64(100000.0)),
        Ok(100000u32)
    );

    assert_eq!(
        f32::try_from_js_value(JsValue::from_f64(3.14)),
        Ok(3.14f64 as f32)
    );
    assert_eq!(f32::try_from_js_value(JsValue::from_f64(0.0)), Ok(0.0f32));
    assert_eq!(
        f32::try_from_js_value(JsValue::from_f64(2.0_f64.powi(32))),
        Ok(2.0_f32.powi(32))
    );
}

#[wasm_bindgen_test]
fn jsvalue_try_from_js_value() {
    let val = JsValue::from_f64(42.0);
    assert_eq!(JsValue::try_from_js_value(val.clone()), Ok(val.clone()));

    let val2 = JsValue::from_str("hello");
    assert_eq!(JsValue::try_from_js_value(val2.clone()), Ok(val2.clone()));

    assert_eq!(JsValue::try_from_js_value(JsValue::NULL), Ok(JsValue::NULL));
    assert_eq!(
        JsValue::try_from_js_value(JsValue::UNDEFINED),
        Ok(JsValue::UNDEFINED)
    );
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

#[wasm_bindgen_test]
fn numeric_types_reject_invalid_js_values() {
    // Test that all numeric types reject boolean values (true/false)
    assert!(i8::try_from_js_value(JsValue::TRUE).is_err());
    assert!(i8::try_from_js_value(JsValue::FALSE).is_err());
    assert!(u8::try_from_js_value(JsValue::TRUE).is_err());
    assert!(u8::try_from_js_value(JsValue::FALSE).is_err());

    assert!(i16::try_from_js_value(JsValue::TRUE).is_err());
    assert!(i16::try_from_js_value(JsValue::FALSE).is_err());
    assert!(u16::try_from_js_value(JsValue::TRUE).is_err());
    assert!(u16::try_from_js_value(JsValue::FALSE).is_err());

    assert!(i32::try_from_js_value(JsValue::TRUE).is_err());
    assert!(i32::try_from_js_value(JsValue::FALSE).is_err());
    assert!(u32::try_from_js_value(JsValue::TRUE).is_err());
    assert!(u32::try_from_js_value(JsValue::FALSE).is_err());

    assert!(i64::try_from_js_value(JsValue::TRUE).is_err());
    assert!(i64::try_from_js_value(JsValue::FALSE).is_err());

    assert!(i128::try_from_js_value(JsValue::TRUE).is_err());
    assert!(i128::try_from_js_value(JsValue::FALSE).is_err());
    assert!(u128::try_from_js_value(JsValue::TRUE).is_err());
    assert!(u128::try_from_js_value(JsValue::FALSE).is_err());

    assert!(f32::try_from_js_value(JsValue::TRUE).is_err());
    assert!(f32::try_from_js_value(JsValue::FALSE).is_err());
    assert!(f64::try_from_js_value(JsValue::TRUE).is_err());
    assert!(f64::try_from_js_value(JsValue::FALSE).is_err());

    assert!(i8::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(u8::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(i16::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(u16::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(i32::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(u32::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(i64::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(u64::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(i128::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(u128::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(f32::try_from_js_value(JsValue::UNDEFINED).is_err());
    assert!(f64::try_from_js_value(JsValue::UNDEFINED).is_err());

    assert!(i8::try_from_js_value(JsValue::NULL).is_err());
    assert!(u8::try_from_js_value(JsValue::NULL).is_err());
    assert!(i16::try_from_js_value(JsValue::NULL).is_err());
    assert!(u16::try_from_js_value(JsValue::NULL).is_err());
    assert!(i32::try_from_js_value(JsValue::NULL).is_err());
    assert!(u32::try_from_js_value(JsValue::NULL).is_err());
    assert!(i64::try_from_js_value(JsValue::NULL).is_err());
    assert!(u64::try_from_js_value(JsValue::NULL).is_err());
    assert!(i128::try_from_js_value(JsValue::NULL).is_err());
    assert!(u128::try_from_js_value(JsValue::NULL).is_err());
    assert!(f32::try_from_js_value(JsValue::NULL).is_err());
    assert!(f64::try_from_js_value(JsValue::NULL).is_err());

    assert!(i64::try_from_js_value(JsValue::from_f64(0.0)).is_err());
    assert!(i64::try_from_js_value(JsValue::from_f64(25.0)).is_err());

    assert!(i8::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(i8::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(u8::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(u8::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(i16::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(i16::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(u16::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(u16::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(i32::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(i32::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(u32::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(u32::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(i64::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(i64::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(i128::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(i128::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(u128::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(u128::try_from_js_value(JsValue::from_str("25")).is_err());
    assert!(f32::try_from_js_value(JsValue::from_str("0")).is_err());
    assert!(f32::try_from_js_value(JsValue::from_str("25")).is_err());
}

// https://github.com/wasm-bindgen/wasm-bindgen/issues/4289
// TryFromJsValue::try_from_js_value() invalidates the converted value on failure
#[wasm_bindgen]
pub struct SquareShape {
    #[allow(dead_code)]
    side_length: f64,
}

#[wasm_bindgen]
pub struct CircleShape {
    #[allow(dead_code)]
    radius: f64,
}

#[wasm_bindgen_test]
fn try_from_js_value_invalidates_cloned_value() {
    let circle = CircleShape { radius: 5.0 };
    let value = JsValue::from(circle);

    // First check fails (value is CircleShape, not SquareShape)
    assert!(SquareShape::try_from_js_value(value.clone()).is_err());

    // Second check should succeed, but the first check invalidated the cloned value
    let result = CircleShape::try_from_js_value(value.clone());
    assert!(
        result.is_ok(),
        "Bug #4289: cloned value was invalidated by first failed try_from_js_value"
    );
}
