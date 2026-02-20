use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[cfg(not(js_sys_unstable_apis))]
fn set2vec(s: &Set<JsValue>) -> Vec<JsValue> {
    let mut result = Vec::new();
    s.for_each(&mut |x, _, _| result.push(x));
    result
}

#[cfg(js_sys_unstable_apis)]
fn set2vec(s: &Set<JsValue>) -> Vec<JsValue> {
    let mut result = Vec::new();
    s.for_each(ImmediateClosure::new_mut(&mut |x| result.push(x)));
    result
}

#[cfg(not(js_sys_unstable_apis))]
macro_rules! new_set {
    () => {
        Set::new(&JsValue::undefined())
    };
}

#[cfg(js_sys_unstable_apis)]
macro_rules! new_set {
    () => {
        Set::<JsValue>::new()
    };
}

#[wasm_bindgen_test]
fn add() {
    let set = new_set!();
    set.add(&100.into());
    assert_eq!(set.size(), 1);
    assert_eq!(set2vec(&set)[0], 100);
}

#[wasm_bindgen_test]
fn clear() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());
    assert_eq!(set.size(), 3);
    set.clear();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn delete() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.delete(&3.into()));
    assert!(!set.delete(&3.into()));
    assert!(!set.delete(&4.into()));
}

#[wasm_bindgen_test]
fn for_each() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let v = set2vec(&set);
    assert_eq!(v.len(), 3);
    assert!(v.iter().any(|v| *v == 1));
    assert!(v.iter().any(|v| *v == 2));
    assert!(v.iter().any(|v| *v == 3));
}

#[wasm_bindgen_test]
fn has() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.has(&1.into()));
    assert!(!set.has(&1.3.into()));
}

#[wasm_bindgen_test]
fn new() {
    assert_eq!(new_set!().size(), 0);
}

#[wasm_bindgen_test]
fn size() {
    let set = new_set!();
    assert_eq!(set.size(), 0);
    set.add(&1.into());
    assert_eq!(set.size(), 1);
    set.add(&2.into());
    assert_eq!(set.size(), 2);
    set.add(&3.into());
    assert_eq!(set.size(), 3);
}

#[wasm_bindgen_test]
fn set_inheritance() {
    let set = new_set!();
    assert!(set.is_instance_of::<Set>());
    assert!(set.is_instance_of::<Object>());
    let _: &Object = set.as_ref();
}

#[wasm_bindgen_test]
fn keys() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let list = set
        .keys()
        .into_iter()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    assert_eq!(list.len(), 3);
    assert!(list.iter().any(|l| *l == 1));
    assert!(list.iter().any(|l| *l == 2));
    assert!(list.iter().any(|l| *l == 3));
}

#[wasm_bindgen_test]
fn values() {
    let set = new_set!();
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let list = set
        .values()
        .into_iter()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    assert_eq!(list.len(), 3);
    assert!(list.iter().any(|l| *l == 1));
    assert!(list.iter().any(|l| *l == 2));
    assert!(list.iter().any(|l| *l == 3));
}

// Typed Set tests
#[wasm_bindgen_test]
fn new_typed() {
    let set: Set<JsString> = Set::new_typed();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn new_empty() {
    let set: Set<JsString> = Set::new_empty();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn new_from_items() {
    let set: Set<JsString> = Set::new_from_items(&[
        JsString::from("a"),
        JsString::from("b"),
        JsString::from("c"),
    ]);
    assert_eq!(set.size(), 3);
    assert!(set.has(&JsString::from("a")));
    assert!(set.has(&JsString::from("b")));
    assert!(set.has(&JsString::from("c")));
}

#[wasm_bindgen_test]
fn typed_add_has() {
    let set: Set<JsString> = Set::new_typed();
    let val = JsString::from("test");
    assert!(!set.has(&val));
    set.add(&val);
    assert!(set.has(&val));
    assert_eq!(set.size(), 1);
}

#[wasm_bindgen_test]
fn typed_delete() {
    let set: Set<JsString> = Set::new_typed();
    let val = JsString::from("delete_me");
    set.add(&val);
    assert!(set.has(&val));
    assert!(set.delete(&val));
    assert!(!set.has(&val));
    assert!(!set.delete(&val));
}

#[wasm_bindgen_test]
fn typed_clear() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("a"));
    set.add(&JsString::from("b"));
    set.add(&JsString::from("c"));
    assert_eq!(set.size(), 3);
    set.clear();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn typed_values_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("x"));
    set.add(&JsString::from("y"));
    set.add(&JsString::from("z"));

    let values: Vec<JsString> = set.values().into_iter().map(|v| v.unwrap()).collect();
    assert_eq!(values.len(), 3);
    assert!(values.iter().any(|v| *v == "x"));
    assert!(values.iter().any(|v| *v == "y"));
    assert!(values.iter().any(|v| *v == "z"));
}

#[wasm_bindgen_test]
fn typed_keys_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("key1"));
    set.add(&JsString::from("key2"));

    let keys: Vec<JsString> = set.keys().into_iter().map(|k| k.unwrap()).collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.iter().any(|k| *k == "key1"));
    assert!(keys.iter().any(|k| *k == "key2"));
}

#[wasm_bindgen_test]
fn typed_entries_iterator() {
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("a"));
    set.add(&JsString::from("b"));

    let entries = set.entries_typed();
    let mut count = 0;
    for result in entries.into_iter() {
        // entries_typed returns Iterator<ArrayTuple<T, T>> - the tuple is an Array
        let arr: Array = result.unwrap().unchecked_into();
        // For Set, entries are [value, value] pairs
        assert_eq!(arr.length(), 2);
        count += 1;
    }
    assert_eq!(count, 2);
}

#[wasm_bindgen_test]
fn new_from_iterable() {
    let arr: Array<JsString> = Array::of(&[
        JsString::from("x"),
        JsString::from("y"),
        JsString::from("z"),
    ]);
    let set: Set<JsString> = Set::new_from_iterable(arr).unwrap();
    assert_eq!(set.size(), 3);
    assert!(set.has(&JsString::from("x")));
    assert!(set.has(&JsString::from("y")));
    assert!(set.has(&JsString::from("z")));
}

// Helper function to check if Set methods are available
fn has_set_methods() -> bool {
    let set_proto: Option<Object> = Reflect::get_str(&new_set!(), &"constructor".into())
        .unwrap()
        .and_then(|c| Reflect::get_str(c.unchecked_ref(), &"prototype".into()).unwrap());

    if let Some(proto) = set_proto {
        Reflect::has_str(&proto, &"union".into()).unwrap_or(false)
    } else {
        false
    }
}

// New Set methods tests (ES2024+)

#[wasm_bindgen_test]
fn union() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));
    set2.add(&JsString::from("e"));

    let result = set1.union(&set2);
    assert_eq!(result.size(), 5);
    assert!(result.has(&JsString::from("a")));
    assert!(result.has(&JsString::from("b")));
    assert!(result.has(&JsString::from("c")));
    assert!(result.has(&JsString::from("d")));
    assert!(result.has(&JsString::from("e")));

    // Original sets unchanged
    assert_eq!(set1.size(), 3);
    assert_eq!(set2.size(), 3);
}

#[wasm_bindgen_test]
fn union_with_numbers() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<Number> = Set::new_typed();
    set1.add(&Number::from(1));
    set1.add(&Number::from(2));
    set1.add(&Number::from(3));

    let set2: Set<Number> = Set::new_typed();
    set2.add(&Number::from(3));
    set2.add(&Number::from(4));
    set2.add(&Number::from(5));

    let result = set1.union(&set2);
    assert_eq!(result.size(), 5);
    assert!(result.has(&Number::from(1)));
    assert!(result.has(&Number::from(2)));
    assert!(result.has(&Number::from(3)));
    assert!(result.has(&Number::from(4)));
    assert!(result.has(&Number::from(5)));
}

#[wasm_bindgen_test]
fn intersection() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    let result = set1.intersection(&set2);
    assert_eq!(result.size(), 2);
    assert!(result.has(&JsString::from("b")));
    assert!(result.has(&JsString::from("c")));
    assert!(!result.has(&JsString::from("a")));
    assert!(!result.has(&JsString::from("d")));
}

#[wasm_bindgen_test]
fn intersection_empty() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    let result = set1.intersection(&set2);
    assert_eq!(result.size(), 0);
}

#[wasm_bindgen_test]
fn difference() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    let result = set1.difference(&set2);
    assert_eq!(result.size(), 1);
    assert!(result.has(&JsString::from("a")));
    assert!(!result.has(&JsString::from("b")));
    assert!(!result.has(&JsString::from("c")));
    assert!(!result.has(&JsString::from("d")));
}

#[wasm_bindgen_test]
fn difference_with_numbers() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<Number> = Set::new_typed();
    set1.add(&Number::from(1));
    set1.add(&Number::from(2));
    set1.add(&Number::from(3));
    set1.add(&Number::from(4));

    let set2: Set<Number> = Set::new_typed();
    set2.add(&Number::from(2));
    set2.add(&Number::from(4));

    let result = set1.difference(&set2);
    assert_eq!(result.size(), 2);
    assert!(result.has(&Number::from(1)));
    assert!(result.has(&Number::from(3)));
    assert!(!result.has(&Number::from(2)));
    assert!(!result.has(&Number::from(4)));
}

#[wasm_bindgen_test]
fn symmetric_difference() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));
    set2.add(&JsString::from("e"));

    let result = set1.symmetric_difference(&set2);
    assert_eq!(result.size(), 3);
    assert!(result.has(&JsString::from("a")));
    assert!(!result.has(&JsString::from("b")));
    assert!(!result.has(&JsString::from("c")));
    assert!(result.has(&JsString::from("d")));
    assert!(result.has(&JsString::from("e")));
}

#[wasm_bindgen_test]
fn symmetric_difference_with_numbers() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<Number> = Set::new_typed();
    set1.add(&Number::from(1));
    set1.add(&Number::from(2));
    set1.add(&Number::from(3));

    let set2: Set<Number> = Set::new_typed();
    set2.add(&Number::from(2));
    set2.add(&Number::from(3));
    set2.add(&Number::from(4));

    let result = set1.symmetric_difference(&set2);
    assert_eq!(result.size(), 2);
    assert!(result.has(&Number::from(1)));
    assert!(!result.has(&Number::from(2)));
    assert!(!result.has(&Number::from(3)));
    assert!(result.has(&Number::from(4)));
}

#[wasm_bindgen_test]
fn is_subset_of_true() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("a"));
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    assert!(set1.is_subset_of(&set2));
}

#[wasm_bindgen_test]
fn is_subset_of_false() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("e"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("a"));
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("c"));

    assert!(!set1.is_subset_of(&set2));
}

#[wasm_bindgen_test]
fn is_subset_of_equal_sets() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<Number> = Set::new_typed();
    set1.add(&Number::from(1));
    set1.add(&Number::from(2));
    set1.add(&Number::from(3));

    let set2: Set<Number> = Set::new_typed();
    set2.add(&Number::from(1));
    set2.add(&Number::from(2));
    set2.add(&Number::from(3));

    assert!(set1.is_subset_of(&set2));
    assert!(set2.is_subset_of(&set1));
}

#[wasm_bindgen_test]
fn is_superset_of_true() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));
    set1.add(&JsString::from("d"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("a"));
    set2.add(&JsString::from("b"));

    assert!(set1.is_superset_of(&set2));
}

#[wasm_bindgen_test]
fn is_superset_of_false() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("a"));
    set2.add(&JsString::from("b"));
    set2.add(&JsString::from("e"));

    assert!(!set1.is_superset_of(&set2));
}

#[wasm_bindgen_test]
fn is_disjoint_from_true() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    assert!(set1.is_disjoint_from(&set2));
    assert!(set2.is_disjoint_from(&set1));
}

#[wasm_bindgen_test]
fn is_disjoint_from_false() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<JsString> = Set::new_typed();
    set1.add(&JsString::from("a"));
    set1.add(&JsString::from("b"));
    set1.add(&JsString::from("c"));

    let set2: Set<JsString> = Set::new_typed();
    set2.add(&JsString::from("c"));
    set2.add(&JsString::from("d"));

    assert!(!set1.is_disjoint_from(&set2));
    assert!(!set2.is_disjoint_from(&set1));
}

#[wasm_bindgen_test]
fn is_disjoint_from_with_numbers() {
    if !has_set_methods() {
        return;
    }

    let set1: Set<Number> = Set::new_typed();
    set1.add(&Number::from(1));
    set1.add(&Number::from(2));
    set1.add(&Number::from(3));

    let set2: Set<Number> = Set::new_typed();
    set2.add(&Number::from(4));
    set2.add(&Number::from(5));
    set2.add(&Number::from(6));

    assert!(set1.is_disjoint_from(&set2));
}

#[wasm_bindgen_test]
fn empty_set_operations() {
    if !has_set_methods() {
        return;
    }

    let empty: Set<JsString> = Set::new_typed();
    let set: Set<JsString> = Set::new_typed();
    set.add(&JsString::from("a"));
    set.add(&JsString::from("b"));
    assert_eq!(set.union(&empty).size(), 2);
    assert_eq!(empty.union(&set).size(), 2);
    assert_eq!(set.intersection(&empty).size(), 0);
    assert_eq!(empty.intersection(&set).size(), 0);
    assert_eq!(set.difference(&empty).size(), 2);
    assert_eq!(empty.difference(&set).size(), 0);
    assert_eq!(set.symmetric_difference(&empty).size(), 2);
    assert_eq!(empty.symmetric_difference(&set).size(), 2);
    assert!(empty.is_subset_of(&set));
    assert!(empty.is_subset_of(&empty));
    assert!(set.is_superset_of(&empty));
    assert!(empty.is_superset_of(&empty));
    assert!(set.is_disjoint_from(&empty));
    assert!(empty.is_disjoint_from(&set));
}
