use js_sys::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn keys() {
    let array = Array::new();
    array.push(&JsValue::from(1));
    array.push(&JsValue::from(2));
    array.push(&JsValue::from(3));
    array.push(&JsValue::from(4));
    array.push(&JsValue::from(5));

    let new_array = Array::from_iterable(&array.keys()).unwrap();

    let mut result = Vec::new();
    new_array.for_each(&mut |i, _, _| result.push(i.as_f64().unwrap()));
    assert_eq!(result, [0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[wasm_bindgen_test]
fn entries() {
    let array = Array::new();
    array.push(&JsValue::from(1));
    array.push(&JsValue::from(2));
    array.push(&JsValue::from(3));
    array.push(&JsValue::from(4));
    array.push(&JsValue::from(5));

    #[allow(deprecated)]
    let new_array = Array::from_iterable(&array.entries()).unwrap();

    new_array.for_each(&mut |a, i, _| {
        assert!(a.is_object());
        #[cfg(not(js_sys_unstable_apis))]
        {
            let array: Array = a.into();
            assert_eq!(array.shift().as_f64().unwrap(), i as f64);
            assert_eq!(array.shift().as_f64().unwrap(), (i + 1) as f64);
            assert_eq!(array.length(), 0);
        }
        #[cfg(js_sys_unstable_apis)]
        {
            use wasm_bindgen::prelude::Upcast;

            let array: Array = a.upcast_into();
            assert_eq!(array.shift().unwrap().as_f64().unwrap(), i as f64);
            assert_eq!(array.shift().unwrap().as_f64().unwrap(), (i + 1) as f64);
            assert_eq!(array.length(), 0);
        }
    });
}
