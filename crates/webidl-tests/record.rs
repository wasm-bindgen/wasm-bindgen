use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_record() {
    let test_record = TestRecord::new().unwrap();

    let number_record = test_record.get_number_record();

    #[cfg(not(wbg_next_unstable))]
    {
        let obj: &js_sys::Object = &number_record;
        let keys: Vec<String> = js_sys::Object::keys(obj)
            .iter()
            .map(|k| k.as_string().unwrap())
            .collect();
        assert_eq!(keys, vec!["a", "b", "c"]);

        let a_val = js_sys::Reflect::get(obj, &"a".into()).unwrap();
        assert_eq!(a_val.as_f64().unwrap() as i32, 1);
    }

    #[cfg(wbg_next_unstable)]
    {
        use wasm_bindgen::JsCast;
        let obj: &js_sys::Object<js_sys::Number> = &number_record;
        let keys: Vec<String> = js_sys::Object::keys(obj)
            .iter()
            .map(|k| k.as_string().unwrap())
            .collect();
        assert_eq!(keys, vec!["a", "b", "c"]);

        let a_val: js_sys::Number = js_sys::Reflect::get(obj, &"a".into())
            .unwrap()
            .unchecked_into();
        assert_eq!(a_val.value_of() as i32, 1);
    }

    let string_record = test_record.get_string_record();

    #[cfg(not(wbg_next_unstable))]
    {
        let obj: &js_sys::Object = &string_record;
        let x_val = js_sys::Reflect::get(obj, &"x".into())
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!(x_val, "hello");
    }

    #[cfg(wbg_next_unstable)]
    {
        use wasm_bindgen::JsCast;

        let obj: &js_sys::Object<js_sys::JsString> = &string_record;
        let x_val: js_sys::JsString = js_sys::Reflect::get(obj, &"x".into())
            .unwrap()
            .unchecked_into();
        assert_eq!(String::from(x_val), "hello");
    }

    #[cfg(not(wbg_next_unstable))]
    {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"foo".into(), &42.into()).unwrap();
        test_record.set_record(&obj);
    }

    #[cfg(wbg_next_unstable)]
    {
        let obj: js_sys::Object<js_sys::Number> = js_sys::Object::new_typed();
        js_sys::Reflect::set(&obj, &"foo".into(), &js_sys::Number::from(42).into()).unwrap();
        let _: () = test_record.set_record(&obj);
    }
}
