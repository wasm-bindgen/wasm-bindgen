use js_sys::{Array, Function, JsString, Object, Reflect, TypedFunction};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/typed_fn.js")]
extern "C" {

    #[wasm_bindgen(js_name = "processWithCallback")]
    fn process_with_callback(value: i32, callback: &dyn Fn(i32) -> i32) -> TypedFunction<(), i32>;

    #[wasm_bindgen(js_name = "transformWithCallback")]
    fn transform_with_callback(
        value: i32,
        callback: &dyn Fn(i32) -> JsString,
    ) -> TypedFunction<(), JsString>;

    #[wasm_bindgen(js_name = "processWithMutCallback")]
    fn process_with_mut_callback(
        values: &[i32],
        callback: &mut dyn FnMut(i32) -> i32,
    ) -> TypedFunction<(), i32>;

    #[wasm_bindgen(js_name = "processJsString")]
    fn process_js_string(
        input: &str,
        callback: &dyn Fn(JsString) -> JsString,
    ) -> TypedFunction<(), JsString>;

    #[wasm_bindgen(js_name = "processOption")]
    fn process_option(
        value: Option<i32>,
        callback: &dyn Fn(Option<i32>) -> i32,
    ) -> TypedFunction<(), i32>;
}

#[wasm_bindgen_test]
fn test_import_fn_basic_callback() {
    let returned_fn = process_with_callback(10, &|num: i32| num * 2);
    let result: i32 = returned_fn.call();
    assert_eq!(result, 20);
}

#[wasm_bindgen_test]
fn test_import_fn_string_return() {
    let returned_fn = transform_with_callback(42, &|num: i32| format!("Value: {}", num).into());
}

#[wasm_bindgen_test]
fn test_import_fn_mut_callback() {
    let mut sum = 0;
    let values = [1, 2, 3, 4, 5];

    let returned_fn = process_with_mut_callback(&values, &mut |num: i32| {
        sum += num;
        sum
    });

    let result: i32 = returned_fn.call();
    assert_eq!(result, 15);
}

#[wasm_bindgen_test]
fn test_import_fn_js_types() {
    let returned_fn = process_js_string("hello", &|js_str: JsString| {
        let rust_string = js_str.as_string().unwrap_or_default();
        JsString::from(format!("{} world", rust_string))
    });

    let result: JsString = returned_fn.call();
    assert_eq!(result.as_string().unwrap(), "hello world");
}

#[wasm_bindgen_test]
fn test_import_fn_option_types() {
    let returned_fn1 = process_option(Some(10), &|opt: Option<i32>| match opt {
        Some(num) => num * 3,
        None => -1,
    });
    let result1: i32 = returned_fn1.call();
    assert_eq!(result1, 30);

    let returned_fn2 = process_option(None, &|opt: Option<i32>| match opt {
        Some(num) => num * 3,
        None => -1,
    });
    let result2: i32 = returned_fn2.call();
    assert_eq!(result2, -1);
}

#[wasm_bindgen_test]
fn test_import_fn_closure_with_capture() {
    let multiplier = 5;
    let offset = 100;

    let returned_fn = process_with_callback(7, &|num: i32| (num * multiplier) + offset);
    let result: i32 = returned_fn.call();
    assert_eq!(result, 135);
}

#[wasm_bindgen_test]
fn test_import_fn_complex_logic() {
    let returned_fn = transform_with_callback(42, &|num: i32| {
        if num > 40 {
            format!("Large: {}", num).into()
        } else if num > 10 {
            format!("Medium: {}", num).into()
        } else {
            format!("Small: {}", num).into()
        }
    });

    let result = returned_fn.call();
    assert_eq!(result.to_string(), "Large: 42");
}

#[wasm_bindgen_test]
fn test_typed_function_no_args() {
    let typed_fn = process_with_callback(10, &|num: i32| num * 2);
    let result: i32 = typed_fn.call();
    assert_eq!(result, 20);
}

#[wasm_bindgen_test]
fn test_typed_function_with_default_this() {
    let typed_fn = process_with_callback(5, &|num: i32| num + 10);
    let result: i32 = typed_fn.call();
    assert_eq!(result, 15);
}

#[wasm_bindgen_test]
fn test_typed_function_string_return() {
    let typed_fn = transform_with_callback(42, &|_num: i32| JsString::from("Value: 42"));
    let result: JsString = typed_fn.call();
    assert_eq!(result.as_string().unwrap(), "Value: 42");
}

#[wasm_bindgen_test]
fn test_typed_function_js_string_return() {
    let typed_fn = process_js_string("hello", &|js_str: JsString| {
        let rust_string = js_str.as_string().unwrap_or_default();
        JsString::from(format!("{} world", rust_string))
    });
    let result: JsString = typed_fn.call();
    assert_eq!(result.as_string().unwrap(), "hello world");
}

#[wasm_bindgen_test]
fn test_typed_function_option_return() {
    let typed_fn1 = process_option(Some(10), &|opt: Option<i32>| match opt {
        Some(num) => num * 3,
        None => -1,
    });
    let result1: i32 = typed_fn1.call();
    assert_eq!(result1, 30);

    let typed_fn2 = process_option(None, &|opt: Option<i32>| match opt {
        Some(num) => num * 3,
        None => -1,
    });
    let result2: i32 = typed_fn2.call();
    assert_eq!(result2, -1);
}

#[wasm_bindgen_test]
fn test_typed_function_with_args() {
    let typed_fn = TypedFunction::<(i32, i32), i32>::new("a", "b", "return a + b;");
    let result: i32 = typed_fn.call(10, 20);
    assert_eq!(result, 30);
}

#[wasm_bindgen_test]
fn test_typed_function_new_separate_params() {
    let typed_fn = TypedFunction::<(i32, i32), i32>::new("x", "y", "return x + y;");
    let result: i32 = typed_fn.call(15, 25);
    assert_eq!(result, 40);
}

#[wasm_bindgen_test]
fn test_typed_function_new_no_args() {
    let typed_fn = TypedFunction::<(), i32>::new("return 42;");
    let result: i32 = typed_fn.call();
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn test_typed_function_mixed_arg_types() {
    let typed_fn = TypedFunction::<(i32, JsString), JsString>::new(
        "num",
        "str",
        "return `${str}: ${num * 2}`;",
    );
    let result = typed_fn.call(21, "Result".into()).to_string();
    assert_eq!(result, "Result: 42");
}

#[wasm_bindgen_test]
fn test_typed_function_custom_this() {
    let js_fn = Function::new_with_args("", "return this.value * 2;");

    let this_obj = Object::new();
    Reflect::set(&this_obj, &"value".into(), &JsValue::from(15)).unwrap();

    let typed_fn: TypedFunction<(), i32, Object> = js_fn.typed_unchecked();
    let result: i32 = typed_fn.call(this_obj);
    assert_eq!(result, 30);
}

#[wasm_bindgen_test]
fn test_typed_function_bind() {
    let typed_fn = TypedFunction::<(i32, i32), i32>::new("a", "b", "return a + b;");

    let bound_fn1 = typed_fn.clone().bind1(10);
    let result1: i32 = bound_fn1.call(20);
    assert_eq!(result1, 30);

    let bound_fn2 = typed_fn.bind2(15, 25);
    let result2: i32 = bound_fn2.call();
    assert_eq!(result2, 40);
}

#[wasm_bindgen_test]
fn test_typed_function_bind_chain() {
    let typed_fn = TypedFunction::<(i32, i32, i32), i32>::new("a", "b", "c", "return a + b + c;");

    let bound1 = typed_fn.bind1(10);
    let bound2 = bound1.bind1(20);
    let bound3 = bound2.bind1(30);

    let result: i32 = bound3.call();
    assert_eq!(result, 60);
}

#[wasm_bindgen_test]
fn test_typed_function_bind_different_types() {
    let typed_fn = TypedFunction::<(i32, JsString), JsString>::new(
        "num",
        "str",
        "return `${str}: ${num * 2}`;",
    );

    let bound_fn = typed_fn.bind1(21);
    let result = bound_fn.call("Result".into()).to_string();
    assert_eq!(result, "Result: 42");
}

#[wasm_bindgen_test]
fn test_typed_function_properties() {
    let typed_fn = TypedFunction::<(i32, i32), i32>::new("a", "b", "return a + b;");

    assert_eq!(typed_fn.length(), 2);

    let name = typed_fn.name().to_string();
    assert_eq!(name, "anonymous");

    let source = typed_fn.to_string();
    assert!(source.includes("function", 0));
    assert!(source.includes("a", 0));
    assert!(source.includes("b", 0));
}

#[wasm_bindgen_test]
fn test_typed_function_properties_after_bind() {
    let typed_fn = TypedFunction::<(i32, i32, i32), i32>::new("x", "y", "z", "return x + y + z;");

    assert_eq!(typed_fn.length(), 3);

    let bound_fn = typed_fn.bind1(10);
    assert_eq!(bound_fn.length(), 2);

    let bound_fn2 = bound_fn.bind2(20, 30);
    assert_eq!(bound_fn2.length(), 0);
}

#[wasm_bindgen_test]
fn test_typed_function_apply() {
    let typed_fn = TypedFunction::<(i32, i32), i32>::new("a", "b", "return a + b;");

    let args = Array::new();
    args.push(&JsValue::from(15));
    args.push(&JsValue::from(25));

    let result: i32 = typed_fn.apply_with_undefined(&args);
    assert_eq!(result, 40);
}

#[wasm_bindgen_test]
fn test_typed_function_apply_with_this() {
    let typed_fn = TypedFunction::<(), i32, Object>::new("return this.value * 3;");

    let this_obj = Object::new();
    Reflect::set(&this_obj, &"value".into(), &JsValue::from(7)).unwrap();

    let args = Array::new();
    let result: i32 = typed_fn.apply(this_obj.as_ref(), &args);
    assert_eq!(result, 21);
}

#[wasm_bindgen_test]
fn test_typed_function_apply_string_return() {
    let typed_fn = TypedFunction::<(JsString, i32), JsString>::new(
        "str",
        "num",
        "return `${str}: ${num * 2}`;",
    );

    let args = Array::new();
    args.push(&JsValue::from("Test"));
    args.push(&JsValue::from(21));

    let result: JsString = typed_fn.apply_with_undefined(&args);
    assert_eq!(result.to_string(), "Test: 42");
}
