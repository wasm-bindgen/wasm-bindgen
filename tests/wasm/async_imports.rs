use js_sys::{Array, BigInt, Error, JsString, Number, Object, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

fn extract_error_message(js_value: &JsValue) -> Option<String> {
    if let Some(s) = js_value.as_string() {
        return Some(s);
    }

    if let Some(error) = js_value.dyn_ref::<Error>() {
        return Some(String::from(error.message()));
    }

    if let Some(s) = js_value.as_string() {
        return Some(s);
    }

    None
}

#[wasm_bindgen(module = "tests/wasm/async_imports.js")]
extern "C" {
    async fn async_return_js_string() -> JsString;
    async fn async_return_jsvalue() -> JsValue;
    async fn async_return_custom_import() -> CustomImportedType;
    async fn async_return_js_object() -> Object;
    async fn async_return_js_array() -> Array;
    async fn async_return_js_number() -> Number;

    #[wasm_bindgen(catch)]
    async fn async_return_custom_with_catch() -> Result<CustomImportedType, JsValue>;

    async fn async_return_unit();
    #[wasm_bindgen(catch)]
    async fn async_return_unit_with_catch() -> Result<(), JsString>;
    #[wasm_bindgen(catch)]
    async fn async_throw_jsstring_error() -> Result<(), JsString>;
    #[wasm_bindgen(catch)]
    async fn async_throw_unit_error() -> Result<(), JsValue>;

    async fn async_return_bigint() -> BigInt;
    async fn async_return_uint8array() -> Uint8Array;
    async fn async_return_f64() -> f64;
    async fn async_return_i32() -> i32;
    async fn async_return_u32() -> u32;
    async fn async_return_f32() -> f32;
    async fn async_return_i8() -> i8;
    async fn async_return_u8() -> u8;
    async fn async_return_i16() -> i16;
    async fn async_return_u16() -> u16;
    async fn async_return_i64() -> i64;
    async fn async_return_u64() -> u64;
    async fn async_return_bool() -> bool;
    async fn async_return_char() -> char;
    async fn async_return_option_i32_some() -> Option<i32>;
    async fn async_return_option_i32_none() -> Option<i32>;

    async fn async_return_option_jsstring_some() -> Option<JsString>;
    async fn async_return_option_jsstring_none() -> Option<JsString>;

    #[wasm_bindgen(catch)]
    async fn async_return_result_option_jsstring_some() -> Result<Option<JsString>, JsValue>;
    #[wasm_bindgen(catch)]
    async fn async_return_result_option_jsstring_none() -> Result<Option<JsString>, JsValue>;

    #[wasm_bindgen(catch)]
    async fn async_return_result_u32_ok() -> Result<u32, JsValue>;
    #[wasm_bindgen(catch)]
    async fn async_return_result_u32_err() -> Result<u32, JsValue>;

    #[wasm_bindgen(catch)]
    async fn async_return_result_option_u32_ok_some() -> Result<Option<u32>, JsValue>;
    #[wasm_bindgen(catch)]
    async fn async_return_result_option_u32_ok_none() -> Result<Option<u32>, JsValue>;
    #[wasm_bindgen(catch)]
    async fn async_return_result_option_u32_err() -> Result<Option<u32>, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    type CustomImportedType;
    #[wasm_bindgen(method, getter)]
    fn value(this: &CustomImportedType) -> u32;
}

#[wasm_bindgen_test]
async fn test_async_string_imports() {
    assert_eq!(
        String::from(async_return_js_string().await),
        "JsString from JavaScript!"
    );
}

#[wasm_bindgen_test]
async fn test_async_jsvalue_import() {
    let value = async_return_jsvalue().await;
    assert_eq!(value.as_f64(), Some(123.0));
}

#[wasm_bindgen_test]
async fn test_async_custom_imports() {
    let custom = async_return_custom_import().await;
    assert_eq!(custom.value(), 999);

    let obj = async_return_js_object().await;
    assert!(obj.is_object());

    let arr = async_return_js_array().await;
    assert_eq!(arr.length(), 3);

    let num = async_return_js_number().await;
    assert_eq!(num.value_of(), 456.0);
}

#[wasm_bindgen_test]
async fn test_async_custom_with_catch() {
    let custom = async_return_custom_with_catch().await.unwrap();
    assert_eq!(custom.value(), 999);
}

#[wasm_bindgen_test]
async fn test_async_unit_returns() {
    async_return_unit().await;
    async_return_unit_with_catch().await.unwrap();

    let unit_err = async_throw_unit_error().await;
    assert!(unit_err.is_err());
    let unit_error_msg = unit_err.unwrap_err();
    assert_eq!(
        extract_error_message(&unit_error_msg),
        Some("Unit error!".to_string())
    );
}

#[wasm_bindgen_test]
async fn test_async_jsstring_error() {
    let jsstring_err = async_throw_jsstring_error().await;
    assert!(jsstring_err.is_err());
    let jsstring_error_msg = jsstring_err.unwrap_err();
    assert_eq!(String::from(jsstring_error_msg), "JsString error message!");
}

#[wasm_bindgen_test]
async fn test_advanced_edge_cases() {
    let bigint = async_return_bigint().await;
    let expected_value = js_sys::BigInt::from(9007199254740991i64);
    assert_eq!(
        bigint.to_string(10).unwrap(),
        expected_value.to_string(10).unwrap()
    );

    let uint8array = async_return_uint8array().await;
    assert_eq!(uint8array.length(), 5);
    assert_eq!(uint8array.get_index(0), 1);
    assert_eq!(uint8array.get_index(4), 5);

    let f64_val = async_return_f64().await;
    #[allow(clippy::approx_constant)]
    {
        assert_eq!(f64_val, 3.14159);
    }
}

#[wasm_bindgen_test]
async fn test_async_primitive_types() {
    assert_eq!(async_return_i32().await, -42);
    assert_eq!(async_return_u32().await, 42);
    assert_eq!(async_return_i8().await, -127);
    assert_eq!(async_return_u8().await, 255);
    assert_eq!(async_return_i16().await, -32767);
    assert_eq!(async_return_u16().await, 65535);
    assert_eq!(async_return_i64().await, 9007199254740991i64);
    assert_eq!(async_return_u64().await, 18446744073709551615u64);

    #[allow(clippy::approx_constant)]
    {
        assert_eq!(async_return_f32().await, 3.14f32);
    }

    #[allow(clippy::bool_assert_comparison)]
    {
        assert_eq!(async_return_bool().await, true);
    }

    assert_eq!(async_return_char().await, 'A');

    assert_eq!(async_return_option_i32_some().await, Some(42));

    assert_eq!(async_return_option_i32_none().await, None);
}

#[wasm_bindgen_test]
async fn test_async_option_jsstring() {
    let opt_jsstring = async_return_option_jsstring_some().await;
    assert!(opt_jsstring.is_some());
    assert_eq!(String::from(opt_jsstring.unwrap()), "JsString option value");

    assert_eq!(async_return_option_jsstring_none().await, None);
}

#[wasm_bindgen_test]
async fn test_async_result_option_types() {
    let result_opt_some = async_return_result_option_jsstring_some().await;
    assert!(result_opt_some.is_ok());
    let opt_jsstring = result_opt_some.unwrap();
    assert!(opt_jsstring.is_some());
    assert_eq!(
        String::from(opt_jsstring.unwrap()),
        "Result Option JsString value"
    );

    let result_opt_none = async_return_result_option_jsstring_none().await;
    assert!(result_opt_none.is_ok());
    assert_eq!(result_opt_none.unwrap(), None);
}

#[wasm_bindgen_test]
async fn test_async_result_u32() {
    let result_ok = async_return_result_u32_ok().await;
    assert!(result_ok.is_ok());
    assert_eq!(result_ok.unwrap(), 42);

    let result_err = async_return_result_u32_err().await;
    assert!(result_err.is_err());
    let error_msg = result_err.unwrap_err();
    assert_eq!(
        extract_error_message(&error_msg),
        Some("u32 error!".to_string())
    );
}

#[wasm_bindgen_test]
async fn test_async_result_option_u32() {
    let result_ok_some = async_return_result_option_u32_ok_some().await;
    assert!(result_ok_some.is_ok());
    assert_eq!(result_ok_some.unwrap(), Some(123));

    let result_ok_none = async_return_result_option_u32_ok_none().await;
    assert!(result_ok_none.is_ok());
    assert_eq!(result_ok_none.unwrap(), None);

    let result_err = async_return_result_option_u32_err().await;
    assert!(result_err.is_err());
    let error_msg = result_err.unwrap_err();
    assert_eq!(
        extract_error_message(&error_msg),
        Some("Option<u32> error!".to_string())
    );
}
