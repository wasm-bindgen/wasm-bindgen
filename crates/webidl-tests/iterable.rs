use crate::generated::*;
#[cfg(wbg_next_unstable)]
use js_sys::{JsString, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn single_iterable() {
    // Single-typed iterable - value iterator for indexed properties
    let iterable = TestSingleIterable::new().unwrap();

    // Test entries() - Iterator<V>
    let mut entries_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for entry in iterable.entries().into_iter() {
            let entry = entry.unwrap();
            entries_vec.push(entry.as_string().unwrap());
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for entry in iterable.entries().into_iter() {
            let value = entry.unwrap();
            entries_vec.push(value);
        }
    }

    assert_eq!(
        &entries_vec,
        &[
            "item0".to_string(),
            "item1".to_string(),
            "item2".to_string()
        ]
    );

    // Test keys() - Iterator<u32> (indices)
    let mut keys_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for key in iterable.keys().into_iter() {
            let key = key.unwrap();
            keys_vec.push(key.as_f64().unwrap() as u32);
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for key in iterable.keys().into_iter() {
            let key = key.unwrap();
            keys_vec.push(key);
        }
    }

    assert_eq!(&keys_vec, &[0, 1, 2]);

    // Test values() - Iterator<V>
    let mut values_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for value in iterable.values().into_iter() {
            let value = value.unwrap();
            values_vec.push(value.as_string().unwrap());
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for value in iterable.values().into_iter() {
            let value = value.unwrap();
            values_vec.push(value);
        }
    }

    assert_eq!(
        &values_vec,
        &[
            "item0".to_string(),
            "item1".to_string(),
            "item2".to_string()
        ]
    );

    // Test forEach callback
    let cb = Closure::wrap(Box::new(|value: String| {
        assert!(value.starts_with("item"));
    }) as Box<dyn Fn(String)>);

    iterable.for_each(cb.as_ref().unchecked_ref()).unwrap();
}

#[wasm_bindgen_test]
fn double_iterable() {
    // Double-typed iterable - pair iterator
    let iterable = TestDoubleIterable::new().unwrap();

    // Test entries() - Iterator<ArrayTuple<K, V>>
    let mut entries_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for entry in iterable.entries().into_iter() {
            let entry = entry.unwrap();
            let pair = entry.dyn_into::<js_sys::Array>().unwrap();
            let key = pair.get(0).as_string().unwrap();
            let value = pair.get(1).as_f64().unwrap() as i32;
            entries_vec.push((key, value));
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for entry in iterable.entries().into_iter() {
            let pair = entry.unwrap();
            entries_vec.push((pair.first(), pair.last()));
        }
    }

    #[cfg(wbg_next_unstable)]
    assert_eq!(
        &entries_vec,
        &[
            (JsString::from("key1"), Number::from(10)),
            (JsString::from("key2"), Number::from(20)),
            (JsString::from("key3"), Number::from(30))
        ]
    );

    #[cfg(not(wbg_next_unstable))]
    assert_eq!(
        &entries_vec,
        &[
            ("key1".to_string(), 10),
            ("key2".to_string(), 20),
            ("key3".to_string(), 30)
        ]
    );

    // Test keys() - Iterator<K>
    let mut keys_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for key in iterable.keys().into_iter() {
            let key = key.unwrap();
            keys_vec.push(key.as_string().unwrap());
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for key in iterable.keys().into_iter() {
            let key = key.unwrap();
            keys_vec.push(key);
        }
    }

    assert_eq!(
        &keys_vec,
        &["key1".to_string(), "key2".to_string(), "key3".to_string()]
    );

    // Test values() - Iterator<V>
    let mut values_vec = vec![];

    #[cfg(not(wbg_next_unstable))]
    {
        for value in iterable.values().into_iter() {
            let value = value.unwrap();
            values_vec.push(value.as_f64().unwrap() as i32);
        }
    }

    #[cfg(wbg_next_unstable)]
    {
        for value in iterable.values().into_iter() {
            let value = value.unwrap();
            values_vec.push(value);
        }
    }

    assert_eq!(&values_vec, &[10, 20, 30]);

    // Test forEach callback
    let cb = Closure::wrap(Box::new(|value: i32| {
        assert!(value == 10 || value == 20 || value == 30);
    }) as Box<dyn Fn(i32)>);

    iterable.for_each(cb.as_ref().unchecked_ref()).unwrap();
}
