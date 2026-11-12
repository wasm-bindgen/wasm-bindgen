use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

fn has_is_error() -> bool {
    let ctor: Object = Reflect::get_str(global().as_ref(), &"Error".into())
        .unwrap()
        .unwrap()
        .unchecked_into();
    Reflect::has_str(&ctor, &"isError".into()).unwrap_or(false)
}

#[wasm_bindgen_test]
fn new() {
    let error = Error::new("some message");
    assert_eq!(JsValue::from(error.message()), "some message");
}

#[wasm_bindgen_test]
fn new_with_cause() {
    let options = Object::new();
    Reflect::set_str(
        options.as_ref(),
        &"cause".into(),
        &JsValue::from("some cause"),
    )
    .unwrap();
    let error = Error::new_with_options("some message", &options);
    assert_eq!(error.cause(), "some cause");
}

#[wasm_bindgen_test]
fn empty_cause() {
    let error = Error::new("test");
    assert_eq!(error.cause(), JsValue::UNDEFINED);
}

#[wasm_bindgen_test]
fn set_cause() {
    let error = Error::new("test");
    error.set_cause(&JsValue::from("different"));
    assert_eq!(error.cause(), "different");
}

#[wasm_bindgen_test]
fn set_message() {
    let error = Error::new("test");
    error.set_message("another");
    assert_eq!(JsValue::from(error.message()), "another");
}

#[wasm_bindgen_test]
fn name() {
    let error = Error::new("test");
    assert_eq!(JsValue::from(error.name()), "Error");
}

#[wasm_bindgen_test]
fn set_name() {
    let error = Error::new("test");
    error.set_name("different");
    assert_eq!(JsValue::from(error.name()), "different");
}

#[allow(deprecated)]
#[wasm_bindgen_test]
fn to_string() {
    let error = Error::new("error message 1");
    assert_eq!(JsValue::from(error.to_string()), "Error: error message 1");
    error.set_name("error_name_1");
    assert_eq!(
        JsValue::from(error.to_string()),
        "error_name_1: error message 1"
    );
}

#[wasm_bindgen_test]
fn stack_trace_limit() {
    let orig = Error::stack_trace_limit();
    Error::set_stack_trace_limit(&JsValue::from(5));
    assert_eq!(Error::stack_trace_limit(), 5);
    Error::set_stack_trace_limit(&orig);
}

#[wasm_bindgen_test]
fn error_inheritance() {
    let error = Error::new("test");
    assert!(error.is_instance_of::<Error>());
    assert!(error.is_instance_of::<Object>());
    let _: &Object = error.as_ref();
}

#[wasm_bindgen_test]
fn is_error() {
    if !has_is_error() {
        return;
    }

    assert!(Error::is_error(&Error::new("test").into()));
    assert!(Error::is_error(&TypeError::new("test").into()));

    let fake = Object::new();
    Reflect::set_str(fake.as_ref(), &"name".into(), &JsValue::from("Error")).unwrap();
    Reflect::set_str(fake.as_ref(), &"message".into(), &JsValue::from("test")).unwrap();
    Reflect::set_str(fake.as_ref(), &"stack".into(), &JsValue::from("")).unwrap();
    assert!(!Error::is_error(&fake.into()));

    let inherited = Object::create(Error::new("test").as_ref());
    assert!(!Error::is_error(&inherited.into()));

    assert!(!Error::is_error(&JsValue::from(42)));
    assert!(!Error::is_error(&JsValue::from("Error")));
    assert!(!Error::is_error(&JsValue::NULL));
    assert!(!Error::is_error(&JsValue::UNDEFINED));
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn string_returns() {
    let e = Error::new("boom");
    let message: String = e.message();
    assert_eq!(message, "boom");
    assert_eq!(e.name(), "Error");
    assert_eq!(e.to_string(), "Error: boom");
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn from_js_value_keeps_errors() {
    let original = Error::new("boom");
    assert_eq!(Error::from(JsValue::from(original.clone())), original);

    // Only the error shape is checked, not the class.
    let type_error = JsValue::from(TypeError::new("boom"));
    assert_eq!(
        JsValue::from(RangeError::from(type_error.clone())),
        type_error
    );
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn from_js_value_wraps_non_errors() {
    let e = Error::from(JsValue::from_str("boom"));
    assert_eq!(e.message(), "boom");
    assert_eq!(e.name(), "Error");
    assert_eq!(e.cause(), JsValue::from_str("boom"));

    assert_eq!(Error::from(JsValue::from_f64(42.0)).message(), "42");
    assert_eq!(Error::from(JsValue::UNDEFINED).message(), "undefined");

    let e = RangeError::from(JsValue::NULL);
    assert!(e.is_instance_of::<RangeError>());
    assert_eq!(e.message(), "null");
    assert_eq!(e.name(), "RangeError");
    assert_eq!(e.cause(), JsValue::NULL);

    let e = AggregateError::from(JsValue::from_str("boom"));
    assert!(e.is_instance_of::<AggregateError>());
    assert_eq!(e.message(), "boom");

    let e = WebAssembly::RuntimeError::from(JsValue::from_str("boom"));
    assert!(e.is_instance_of::<WebAssembly::RuntimeError>());
    assert_eq!(e.message(), "boom");
}

#[cfg(all(js_sys_unstable_apis, feature = "unsafe-eval"))]
#[wasm_bindgen_test]
fn from_js_value_checks_the_error_shape() {
    let like = eval("({ name: 'Custom', message: 'boom' })").unwrap();
    let e = Error::from(like.clone());
    assert_eq!(JsValue::from(e.clone()), like);
    assert_eq!(e.message(), "boom");

    let broken = eval("const e = new Error('boom'); e.message = undefined; e").unwrap();
    let e = Error::from(broken.clone());
    assert_ne!(JsValue::from(e.clone()), broken);
    assert_eq!(e.cause(), broken);
    let _: String = e.message();

    let object = eval("({ code: 1 })").unwrap();
    assert_eq!(Error::from(object).message(), r#"Object({"code":1})"#);

    let throws = eval("({ get message() { throw 1 }, name: 'Custom' })").unwrap();
    assert_eq!(Error::from(throws.clone()).cause(), throws);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = String)]
    fn string_as_type_error(value: &str) -> TypeError;
}

#[wasm_bindgen(module = "tests/wasm/Error.js")]
extern "C" {
    fn call_with(f: &dyn Fn(&RangeError) -> bool, value: &JsValue) -> bool;
}

// The casts and the ABI glue of a subclass stay unchecked.
#[wasm_bindgen_test]
fn subclass_casts_are_unchecked() {
    let value = JsValue::from_str("boom");
    assert_eq!(
        JsValue::from(value.clone().unchecked_into::<TypeError>()),
        value
    );
    assert_eq!(JsValue::from(string_as_type_error("boom")), value);
    assert!(call_with(
        &|e: &RangeError| *AsRef::<JsValue>::as_ref(e) == "boom",
        &value
    ));
    assert_eq!(value, "boom");
}
