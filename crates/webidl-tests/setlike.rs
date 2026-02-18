use crate::generated::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! read_test_suite {
    ($setlike:ty, $name:ident) => {
        #[wasm_bindgen_test]
        fn $name() {
            // { "a", "b", "c" }
            let setlike = <$setlike>::new().unwrap();

            // readonly attribute unsigned long size;
            assert_eq!(setlike.size(), 3);

            // boolean has(V value);
            assert!(setlike.has("a"));
            assert!(setlike.has("b"));
            assert!(setlike.has("c"));
            assert!(!setlike.has("d"));

            // Test forEach with typed callback under next-unstable
            #[cfg(wbg_next_unstable)]
            {
                // Create a typed Function<fn(JsString) -> Undefined> callback
                let cb: js_sys::Function<fn(js_sys::JsString) -> js_sys::Undefined> =
                    Closure::wrap(Box::new(|value: js_sys::JsString| {
                        let value: String = value.into();
                        match value.as_str() {
                            "a" | "b" | "c" => {}
                            _ => panic!("unexpected value: {}", value),
                        }
                    }) as Box<dyn Fn(js_sys::JsString)>)
                    .into_js_value()
                    .unchecked_into();

                setlike.for_each(&cb).unwrap();
            }

            // Test forEach with untyped Function callback (compat mode)
            #[cfg(not(wbg_next_unstable))]
            {
                let cb = Closure::wrap(Box::new(|value: String| match value.as_str() {
                    "a" => assert_eq!(value, "a"),
                    "b" => assert_eq!(value, "b"),
                    "c" => assert_eq!(value, "c"),
                    _ => panic!("unexpected key"),
                }) as Box<dyn Fn(String)>);

                setlike.for_each(cb.as_ref().unchecked_ref()).unwrap();
            }

            let mut entries_vec = vec![];

            #[cfg(not(wbg_next_unstable))]
            {
                for entry in setlike.entries().into_iter() {
                    let entry = entry.unwrap();
                    let pair = entry.dyn_into::<js_sys::Array>().unwrap();
                    let value = pair.get(1).as_string().unwrap();

                    entries_vec.push(value);
                }
            }

            #[cfg(wbg_next_unstable)]
            {
                for entry in setlike.entries().into_iter() {
                    let value = entry.unwrap().first();

                    entries_vec.push(value);
                }
            }

            assert_eq!(
                &entries_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );

            let mut keys_vec = vec![];

            #[cfg(not(wbg_next_unstable))]
            {
                for key in setlike.keys().into_iter() {
                    let key = key.unwrap();
                    keys_vec.push(key.as_string().unwrap());
                }
            }

            #[cfg(wbg_next_unstable)]
            {
                for key in setlike.keys().into_iter() {
                    let key = key.unwrap();
                    keys_vec.push(key);
                }
            }

            assert_eq!(
                &keys_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );

            let mut values_vec = vec![];

            #[cfg(not(wbg_next_unstable))]
            {
                for value in setlike.values().into_iter() {
                    let value = value.unwrap();
                    values_vec.push(value.as_string().unwrap());
                }
            }

            #[cfg(wbg_next_unstable)]
            {
                for value in setlike.values().into_iter() {
                    let value = value.unwrap();
                    values_vec.push(value);
                }
            }

            assert_eq!(
                &values_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );
        }
    };
}

read_test_suite!(TestReadOnlySetLike, read_readonly_setlike);
read_test_suite!(TestReadWriteSetLike, read_setlike);

#[wasm_bindgen_test]
fn write_setlike() {
    // { "a", "b", "c" }
    let setlike = TestReadWriteSetLike::new().unwrap();

    let ret = setlike.add("d");
    assert_eq!(setlike.size(), 4);
    assert_eq!(ret, setlike);

    assert!(setlike.delete("d"));
    assert_eq!(setlike.size(), 3);

    // undefined clear();
    setlike.clear();
    assert_eq!(setlike.size(), 0);
}
