use crate::generated::*;
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_void_callback() {
    let test = TestCallbacks::new().unwrap();

    let called = std::rc::Rc::new(std::cell::Cell::new(false));
    let called_clone = called.clone();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        test.invoke_void_callback(closure.as_ref().unchecked_ref());
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        test.invoke_void_callback(
            &Function::from_closure(Closure::<dyn FnMut()>::new(move || {
                called_clone.set(true);
            }))
            .upcast(),
        );
    }

    assert!(called.get(), "Void callback should have been invoked");
}

#[wasm_bindgen_test]
fn test_number_callback() {
    let test = TestCallbacks::new().unwrap();

    let received_value = std::rc::Rc::new(std::cell::Cell::new(0));
    let received_clone = received_value.clone();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut(i32)>::new(move |value| {
            received_clone.set(value);
        });

        test.invoke_number_callback(closure.as_ref().unchecked_ref(), 42);
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let closure = Closure::<dyn FnMut(Number)>::new(move |value: Number| {
            received_clone.set(value.as_f64().unwrap_or(0f64) as i32);
        });

        #[allow(dead_code)]
        let func = Function::from_closure(closure);

        test.invoke_number_callback(&func.upcast(), 42);
    }

    assert_eq!(
        received_value.get(),
        42,
        "Number callback should have received 42"
    );
}

#[wasm_bindgen_test]
fn test_string_transformer() {
    let test = TestCallbacks::new().unwrap();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut(String) -> String>::new(move |input| {
            format!("transformed: {}", input)
        });

        let result = test.invoke_string_transformer(closure.as_ref().unchecked_ref(), "hello");
        assert_eq!(result, "transformed: hello");
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let closure = Closure::<dyn FnMut(String) -> JsString>::new(move |input| {
            let input_str = String::from(&input);
            JsString::from(format!("transformed: {}", input_str))
        });

        let result =
            test.invoke_string_transformer(&Function::from_closure(closure).upcast(), "hello");
        let result_str = String::from(&result);
        assert_eq!(result_str, "transformed: hello");
    }
}

#[wasm_bindgen_test]
fn test_binary_op() {
    let test = TestCallbacks::new().unwrap();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut(i32, i32) -> i32>::new(move |a, b| a + b);

        let result = test.invoke_binary_op(closure.as_ref().unchecked_ref(), 10, 20);
        assert_eq!(result, 30);
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let closure =
            Closure::<dyn FnMut(Number, Number) -> Number>::new(move |a: Number, b: Number| {
                let sum = a.value_of() + b.value_of();
                Number::from(sum)
            });

        let result = test.invoke_binary_op(&Function::from_closure(closure), 10, 20);
        assert_eq!(result as i32, 30);
    }
}

#[wasm_bindgen_test]
fn test_object_callback() {
    let test = TestCallbacks::new().unwrap();

    let was_called = std::rc::Rc::new(std::cell::Cell::new(false));
    let was_called_clone = was_called.clone();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |data: JsValue| {
            was_called_clone.set(true);
            let obj: &Object = data.unchecked_ref();
            let _ = Object::keys(obj).length();
        });

        let obj = Object::new();
        Reflect::set(&obj, &"test".into(), &123.into()).unwrap();
        test.invoke_object_callback(closure.as_ref().unchecked_ref(), &obj);
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let closure = Closure::<dyn FnMut(Object)>::new(move |data| {
            was_called_clone.set(true);
            let _ = Object::keys(&data).length();
        });

        let obj = Object::new();
        Reflect::set(&obj, &"test".into(), &123.into()).unwrap();
        test.invoke_object_callback(&Function::from_closure_upcast(closure), &obj);
    }

    assert!(was_called.get(), "Object callback should have been invoked");
}

#[wasm_bindgen_test]
fn test_sequence_callback() {
    let test = TestCallbacks::new().unwrap();

    #[cfg(feature = "idl-generics-compat")]
    {
        let closure = Closure::<dyn FnMut(JsValue) -> JsValue>::new(move |_| {
            let arr = Array::new();
            arr.push(&1.into());
            arr.push(&2.into());
            arr.push(&3.into());
            arr.into()
        });

        let input = Array::new();
        input.push(&"a".into());
        input.push(&"b".into());

        let result = test.invoke_sequence_callback(closure.as_ref().unchecked_ref(), &input);
        assert_eq!(result.length(), 3);
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let closure = Closure::<dyn FnMut(Array<JsString>) -> Array<Number>>::new(move |_input| {
            let arr = Array::new_typed();
            arr.push(&Number::from(1));
            arr.push(&Number::from(2));
            arr.push(&Number::from(3));
            arr
        });

        let input = Array::new_typed();
        input.push(&JsString::from("a"));
        input.push(&JsString::from("b"));

        let result = test.invoke_sequence_callback(&Function::from_closure(closure), &input);
        assert_eq!(result.length(), 3);
    }
}
