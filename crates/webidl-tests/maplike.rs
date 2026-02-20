use crate::generated::*;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! read_test_suite {
    ($maplike:ty, $name:ident) => {
        #[wasm_bindgen_test]
        fn $name() {
            // { "a": 1, "b": 2, "c": 3 }
            let maplike = <$maplike>::new().unwrap();

            // readonly attribute unsigned long size;
            assert_eq!(maplike.size(), 3);

            // boolean has(K key);
            assert!(maplike.has("a"));
            assert!(maplike.has("b"));
            assert!(maplike.has("c"));
            assert!(!maplike.has("d"));

            // V? get(K key);
            assert_eq!(maplike.get("a"), Some(1));
            assert_eq!(maplike.get("b"), Some(2));
            assert_eq!(maplike.get("c"), Some(3));
            assert_eq!(maplike.get("d"), None);

            // Test forEach with typed callback under next-unstable
            #[cfg(wbg_next_unstable)]
            {
                let closure = wasm_bindgen::closure::Closure::<
                    dyn FnMut(js_sys::Number, js_sys::JsString),
                >::new(|value: js_sys::Number, key: js_sys::JsString| {
                    let value = value.value_of() as u32;
                    let key: String = key.into();
                    match key.as_str() {
                        "a" => assert_eq!(value, 1),
                        "b" => assert_eq!(value, 2),
                        "c" => assert_eq!(value, 3),
                        _ => panic!("unexpected key: {}", key),
                    }
                });
                let cb: js_sys::Function<
                    fn(js_sys::Number, js_sys::JsString) -> js_sys::Undefined,
                > = Function::from_closure(closure);

                maplike.for_each(&cb).unwrap();
            }

            // Test forEach with untyped Function callback (compat mode)
            #[cfg(not(wbg_next_unstable))]
            {
                let cb = Closure::wrap(Box::new(|value: u32, key: String| match key.as_str() {
                    "a" => assert_eq!(value, 1),
                    "b" => assert_eq!(value, 2),
                    "c" => assert_eq!(value, 3),
                    _ => panic!("unexpected key"),
                }) as Box<dyn Fn(u32, String)>);
                maplike
                    .for_each(&cb.into_js_value().unchecked_ref::<Function>())
                    .unwrap();
            }

            let mut entries_vec = vec![];

            for entry in maplike.entries().into_iter() {
                let entry = entry.unwrap();
                let pair = entry.dyn_into::<js_sys::Array>().unwrap();
                let (key, value) = {
                    let key = pair.get(0).as_string().unwrap();
                    let value = pair.get(1).as_f64().unwrap() as u32;
                    (key, value)
                };

                entries_vec.push((key, value));
            }

            assert_eq!(
                &entries_vec,
                &[
                    ("a".to_string(), 1),
                    ("b".to_string(), 2),
                    ("c".to_string(), 3)
                ]
            );

            let mut keys_vec = vec![];

            for key in maplike.keys().into_iter() {
                let key = key.unwrap();
                #[cfg(wbg_next_unstable)]
                keys_vec.push(key.as_string().unwrap());
                #[cfg(not(wbg_next_unstable))]
                keys_vec.push(key.as_string().unwrap());
            }

            assert_eq!(
                &keys_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );

            let mut values_vec = vec![];

            for value in maplike.values().into_iter() {
                let value = value.unwrap();
                #[cfg(wbg_next_unstable)]
                values_vec.push(value.as_f64().unwrap() as u32);
                #[cfg(not(wbg_next_unstable))]
                values_vec.push(value.as_f64().unwrap() as u32);
            }

            assert_eq!(&values_vec, &[1, 2, 3]);
        }
    };
}

read_test_suite!(TestReadOnlyMapLike, read_readonly_maplike);
read_test_suite!(TestReadWriteMapLike, read_maplike);

#[wasm_bindgen_test]
fn write_maplike() {
    // { "a": 1, "b": 2, "c": 3 }
    let maplike = TestReadWriteMapLike::new().unwrap();

    // undefined set(K key, V value);
    let ret1 = maplike.set("a", 4);
    let ret2 = maplike.set("d", 5);
    assert_eq!(maplike.get("a"), Some(4));
    assert_eq!(maplike.get("d"), Some(5));
    assert_eq!(ret1, maplike);
    assert_eq!(ret2, maplike);

    // boolean delete(K key);
    assert!(maplike.delete("a"));
    assert_eq!(maplike.get("a"), None);
    assert!(!maplike.delete("a"));

    // undefined clear();
    maplike.clear();
    assert_eq!(maplike.size(), 0);
}
