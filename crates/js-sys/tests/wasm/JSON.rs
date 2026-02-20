use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn parse_array() {
    let js_array = JSON::parse("[1, 2, 3]").unwrap();
    assert!(Array::is_array(&js_array));
    let js_array: Array = js_array.dyn_into().unwrap();

    let array = Array::from_iterable(&js_array).unwrap();
    assert_eq!(array.length(), 3);

    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(array.pop(), 3);
        assert_eq!(array.pop(), 2);
        assert_eq!(array.pop(), 1);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(array.pop().unwrap(), 3);
        assert_eq!(array.pop().unwrap(), 2);
        assert_eq!(array.pop().unwrap(), 1);
    }
}

#[wasm_bindgen_test]
fn parse_object() {
    let js_object = JSON::parse("{\"x\": 5, \"y\": true, \"z\": [\"foo\", \"bar\"]}").unwrap();
    assert!(js_object.is_object());

    let obj = Object::from(js_object);
    let keys = Object::keys(&obj);
    assert_eq!(keys.length(), 3);

    let values;
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(keys.pop().as_string().unwrap(), "z");
        assert_eq!(keys.pop().as_string().unwrap(), "y");
        assert_eq!(keys.pop().as_string().unwrap(), "x");

        values = Object::values(&obj);
        assert_eq!(values.length(), 3);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(keys.pop().unwrap().as_string().unwrap(), "z");
        assert_eq!(keys.pop().unwrap().as_string().unwrap(), "y");
        assert_eq!(keys.pop().unwrap().as_string().unwrap(), "x");

        values = Object::values(&obj).unwrap();
        assert_eq!(values.length(), 3);
    }

    let z = values.pop_checked().unwrap();
    assert!(Array::is_array(&z));
    let z_arr: Array = z.dyn_into().unwrap();
    let z_array = Array::from_iterable(&z_arr).unwrap();
    assert_eq!(z_array.length(), 2);

    let y = values.pop_checked().unwrap();
    assert_eq!(y.as_bool(), Some(true));

    let x = values.pop_checked().unwrap();
    assert_eq!(x.as_f64().unwrap(), 5.0);
}

#[wasm_bindgen_test]
fn parse_error() {
    let js_object = JSON::parse("invalid json");
    assert!(js_object.is_err());
    let err = js_object.unwrap_err();
    assert!(err.is_instance_of::<Error>());
}

#[wasm_bindgen_test]
fn stringify() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(true));
    arr.push(&JsValue::from("hello"));

    let str1: String = JSON::stringify(&JsValue::from(arr)).unwrap().into();
    assert_eq!(str1, "[1,true,\"hello\"]");

    let obj = Object::new();
    Reflect::set_str(obj.as_ref(), &"foo".into(), &JsValue::from("bar")).unwrap();
    let str2: String = JSON::stringify(&JsValue::from(obj)).unwrap().into();
    assert_eq!(str2, "{\"foo\":\"bar\"}");
}

#[wasm_bindgen_test]
fn stringify_error() {
    let func =
        Function::<fn() -> JsString>::new_no_args_typed("throw new Error(\"rust really rocks\")");
    let obj = Object::new();
    Reflect::set_str(obj.as_ref(), &"toJSON".into(), func.as_ref()).unwrap();

    let result = JSON::stringify(&JsValue::from(obj));
    assert!(result.is_err());
    let err_obj = result.unwrap_err();
    assert!(err_obj.is_instance_of::<Error>());
    let err: &Error = err_obj.dyn_ref().unwrap();
    let err_msg: String = From::from(err.message());
    assert!(err_msg.contains("rust really rocks"));
}

#[wasm_bindgen_test]
fn stringify_with_replacer() {
    let obj = Object::new();
    Reflect::set_str(obj.as_ref(), &"foo".into(), &JsValue::from("bar")).unwrap();
    Reflect::set_str(obj.as_ref(), &"hello".into(), &JsValue::from("world")).unwrap();

    let output1: String;
    #[cfg(not(js_sys_unstable_apis))]
    {
        let replacer_array = Array::new();
        replacer_array.push(&JsValue::from("hello"));
        output1 = JSON::stringify_with_replacer(
            &JsValue::from(obj.clone()),
            &JsValue::from(replacer_array),
        )
        .unwrap()
        .into();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        output1 = JSON::stringify_with_replacer_list(
            &JsValue::from(obj.clone()),
            vec!["hello".to_string()],
            None,
        )
        .unwrap()
        .into();
    }
    assert_eq!(output1, "{\"hello\":\"world\"}");

    let output2: String;
    #[cfg(not(js_sys_unstable_apis))]
    {
        let replacer_func =
            Function::new_with_args("key, value", "return key === 'hello' ? undefined : value");
        output2 = JSON::stringify_with_replacer(&JsValue::from(obj), &JsValue::from(replacer_func))
            .unwrap()
            .into();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        output2 = JSON::stringify_with_replacer(
            &JsValue::from(obj),
            ImmediateClosure::new_mut(&mut |k: JsString, v: JsValue| {
                if k == "hello" {
                    Ok(None)
                } else {
                    Ok(Some(v))
                }
            }),
            None,
        )
        .unwrap()
        .into();
    }
    assert_eq!(output2, "{\"foo\":\"bar\"}");
}

#[wasm_bindgen_test]
fn stringify_with_replacer_error() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(true));
    arr.push(&JsValue::from("hello"));

    let result;
    #[cfg(not(js_sys_unstable_apis))]
    {
        let replacer = Function::new_no_args("throw new Error(\"rust really rocks\")");
        result = JSON::stringify_with_replacer(&JsValue::from(arr), &JsValue::from(replacer));
    }
    #[cfg(js_sys_unstable_apis)]
    {
        result = JSON::stringify_with_replacer(
            &JsValue::from(arr),
            ImmediateClosure::new_mut(&mut |_: JsString, _: JsValue| {
                Err(JsError::new("rust really rocks"))
            }),
            None,
        );
    }
    assert!(result.is_err());
    let err_obj = result.unwrap_err();
    assert!(err_obj.is_instance_of::<Error>());
    let err: &Error = err_obj.dyn_ref().unwrap();
    let err_msg: String = From::from(err.message());
    assert!(err_msg.contains("rust really rocks"));
}

#[wasm_bindgen_test]
fn stringify_with_replacer_and_space() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(true));
    arr.push(&JsValue::from("hello"));

    let output1: String = JSON::stringify_with_replacer_and_space(
        &JsValue::from(arr),
        &JsValue::NULL,
        &JsValue::from(4),
    )
    .unwrap()
    .into();
    assert_eq!(output1, "[\n    1,\n    true,\n    \"hello\"\n]");

    let obj = Object::new();
    Reflect::set_str(obj.as_ref(), &"foo".into(), &JsValue::from("bar")).unwrap();
    Reflect::set_str(obj.as_ref(), &"hello".into(), &JsValue::from("world")).unwrap();

    let replacer_array = Array::new();
    replacer_array.push(&JsValue::from("hello"));
    let output2: String = JSON::stringify_with_replacer_and_space(
        &JsValue::from(obj.clone()),
        &JsValue::from(replacer_array),
        &JsValue::from(4),
    )
    .unwrap()
    .into();
    assert_eq!(output2, "{\n    \"hello\": \"world\"\n}");

    let replacer_func = Function::<fn(JsString, JsValue) -> JsValue>::new_with_args_typed(
        "key, value",
        "return key === 'hello' ? undefined : value",
    );
    let output3: String = JSON::stringify_with_replacer_and_space(
        &JsValue::from(obj),
        &JsValue::from(replacer_func),
        &JsValue::from(4),
    )
    .unwrap()
    .into();
    assert_eq!(output3, "{\n    \"foo\": \"bar\"\n}");
}

#[wasm_bindgen_test]
fn stringify_with_replacer_and_space_error() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(true));
    arr.push(&JsValue::from("hello"));

    let result = JSON::stringify_with_replacer_func(
        &arr,
        ImmediateClosure::new_mut(&mut |_: JsString, _: JsValue| {
            Err(JsError::new("rust really rocks"))
        }),
        Some(4),
    );
    assert!(result.is_err());
    let err_obj = result.unwrap_err();
    assert!(err_obj.is_instance_of::<Error>());
    let err: &Error = err_obj.dyn_ref().unwrap();
    let err_msg: String = From::from(err.message());
    assert!(err_msg.contains("rust really rocks"));
}

#[wasm_bindgen_test]
fn stringify_with_replacer_func_typed() {
    let obj = Object::new();
    Reflect::set_str(&obj.as_ref(), &"a".into(), &JsValue::from(1)).unwrap();
    Reflect::set_str(&obj.as_ref(), &"b".into(), &JsValue::from(2)).unwrap();
    Reflect::set_str(&obj.as_ref(), &"c".into(), &JsValue::from(3)).unwrap();

    // Replacer function that doubles numbers
    let output: String = JSON::stringify_with_replacer_func(
        &JsValue::from(obj),
        ImmediateClosure::new_mut(&mut |_: JsString, value: JsValue| {
            if let Some(n) = value.as_f64() {
                Ok(Some(JsValue::from(n * 2.0)))
            } else {
                Ok(Some(value))
            }
        }),
        None,
    )
    .unwrap()
    .into();
    assert_eq!(output, "{\"a\":2,\"b\":4,\"c\":6}");
}

#[wasm_bindgen_test]
fn stringify_with_replacer_list_typed() {
    let obj = Object::new();
    Reflect::set_str(obj.as_ref(), &"a".into(), &JsValue::from(1)).unwrap();
    Reflect::set_str(obj.as_ref(), &"b".into(), &JsValue::from(2)).unwrap();
    Reflect::set_str(obj.as_ref(), &"c".into(), &JsValue::from(3)).unwrap();

    // Only include "a" and "c" in output
    let output: String = JSON::stringify_with_replacer_list(
        &JsValue::from(obj),
        vec!["a".to_string(), "c".to_string()],
        None,
    )
    .unwrap()
    .into();
    assert_eq!(output, "{\"a\":1,\"c\":3}");
}
